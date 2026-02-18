use crate::config::Config;
use colored::*;
use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Command, Stdio};
use std::thread;

pub struct AgentResult {
    pub stdout: String,
}

fn stream_reader<R: Read>(reader: R, color: Option<Color>) -> Vec<String> {
    let buf = BufReader::new(reader);
    let mut lines = Vec::new();
    for line in buf.lines() {
        match line {
            Ok(line) => {
                match color {
                    Some(c) => println!("{}", line.color(c)),
                    None => println!("{}", line),
                }
                lines.push(line);
            }
            Err(e) => {
                eprintln!("Error reading agent stream: {}", e);
                break;
            }
        }
    }
    lines
}

pub fn invoke(config: &Config, prompt: &str) -> Result<AgentResult, String> {
    eprintln!(
        "Invoking agent: {} {}",
        config.agent_command,
        config.agent_args.join(" ")
    );

    let mut agent_process = Command::new(&config.agent_command)
        .args(&config.agent_args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn agent '{}': {}", config.agent_command, e))?;

    if let Some(mut stdin) = agent_process.stdin.take() {
        stdin
            .write_all(prompt.as_bytes())
            .map_err(|e| format!("Failed to write prompt to agent stdin: {}", e))?;
    }

    let stdout_handle = agent_process.stdout.take().expect("stdout was piped");
    let stderr_handle = agent_process.stderr.take().expect("stderr was piped");

    let stdout_thread = thread::spawn(move || stream_reader(stdout_handle, None));
    let stderr_thread =
        thread::spawn(move || stream_reader(stderr_handle, Some(Color::TrueColor { r: 255, g: 165, b: 0 })));

    let stdout_lines = stdout_thread.join().map_err(|_| "stdout reader thread panicked")?;
    let stderr_lines = stderr_thread.join().map_err(|_| "stderr reader thread panicked")?;

    let status = agent_process
        .wait()
        .map_err(|e| format!("Failed to wait for agent: {}", e))?;

    if !status.success() {
        return Err(format!(
            "Agent exited with status {}\nStderr:\n{}",
            status.code().unwrap_or(-1),
            stderr_lines.join("\n")
        ));
    }

    Ok(AgentResult {
        stdout: stdout_lines.join("\n"),
    })
}
