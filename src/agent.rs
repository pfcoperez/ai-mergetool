use crate::config::Config;
use std::io::Write;
use std::process::{Command, Stdio};

/// Strip markdown code fences that LLMs often wrap output in despite instructions.
fn strip_markdown_fences(text: &str) -> String {
    let trimmed = text.trim();
    // Match opening fence: ``` optionally followed by a language tag
    if let Some(rest) = trimmed.strip_prefix("```") {
        // Remove optional language identifier on the opening fence line
        let rest = match rest.find('\n') {
            Some(pos) => &rest[pos + 1..],
            None => return String::new(),
        };
        // Remove closing fence
        if let Some(content) = rest.strip_suffix("```") {
            return content.trim_end_matches('\n').to_string() + "\n";
        }
        // Closing fence might have trailing whitespace/newline
        let rest_trimmed = rest.trim_end();
        if let Some(content) = rest_trimmed.strip_suffix("```") {
            return content.trim_end_matches('\n').to_string() + "\n";
        }
    }
    text.to_string()
}

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
    let stdout = strip_markdown_fences(&stdout);
    if stdout.trim().is_empty() {
        return Err("Agent produced empty output".to_string());
    }

    Ok(AgentResult { stdout })
}
