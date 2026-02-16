use crate::config::Config;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::process::{Command, Stdio};

fn get_git_merge_tool() -> Option<String> {
    Command::new("git")
        .args(["config", "merge.tool"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                let tool = String::from_utf8_lossy(&o.stdout).trim().to_string();
                if tool.is_empty() { None } else { Some(tool) }
            } else {
                None
            }
        })
}

fn ask_user_for_tool() -> Option<String> {
    eprint!("Enter fallback merge tool command (e.g. vimdiff, meld, kdiff3): ");
    io::stderr().flush().ok();

    let stdin = io::stdin();
    let line = stdin.lock().lines().next()?.ok()?;
    let tool = line.trim().to_string();
    if tool.is_empty() { None } else { Some(tool) }
}

pub fn run(config: &Config, base: &Path, local: &Path, remote: &Path, merged: &Path) -> Result<(), String> {
    let tool = config
        .fallback_merge_tool
        .clone()
        .or_else(|| {
            eprintln!("No fallback_merge_tool in config, checking git config...");
            get_git_merge_tool()
        })
        .or_else(|| {
            eprintln!("No git merge.tool configured.");
            ask_user_for_tool()
        })
        .ok_or_else(|| "No fallback merge tool available".to_string())?;

    eprintln!("Running fallback merge tool: {}", tool);

    let status = Command::new(&tool)
        .arg(base)
        .arg(local)
        .arg(remote)
        .arg(merged)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|e| format!("Failed to run fallback tool '{}': {}", tool, e))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "Fallback tool '{}' exited with status {}",
            tool,
            status.code().unwrap_or(-1)
        ))
    }
}
