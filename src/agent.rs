use crate::config::Config;
use std::io::Write;
use std::process::{Command, Stdio};

pub struct AgentResult {
    pub stdout: String,
}

pub fn invoke(config: &Config, prompt: &str) -> Result<AgentResult, String> {
    eprintln!(
        "Invoking agent: {} {}",
        config.agent_command,
        config.agent_args.join(" ")
    );

    let mut child = Command::new(&config.agent_command)
        .args(&config.agent_args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn agent '{}': {}", config.agent_command, e))?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(prompt.as_bytes())
            .map_err(|e| format!("Failed to write prompt to agent stdin: {}", e))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Failed to wait for agent: {}", e))?;

    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    if !stderr.is_empty() {
        eprintln!("Agent stderr:\n{}", stderr);
    }

    if !output.status.success() {
        return Err(format!(
            "Agent exited with status {}",
            output.status.code().unwrap_or(-1)
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    Ok(AgentResult { stdout })
}
