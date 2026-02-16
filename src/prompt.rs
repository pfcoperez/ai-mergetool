use crate::config::Config;
use std::path::Path;

pub fn build(config: &Config, base: &Path, local: &Path, remote: &Path) -> Result<String, String> {
    let strategy = match &config.strategy_file {
        Some(path) => std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read strategy file '{}': {}", path, e))?,
        None => config.default_strategy.clone(),
    };

    let base_content = std::fs::read_to_string(base)
        .map_err(|e| format!("Failed to read BASE '{}': {}", base.display(), e))?;
    let local_content = std::fs::read_to_string(local)
        .map_err(|e| format!("Failed to read LOCAL '{}': {}", local.display(), e))?;
    let remote_content = std::fs::read_to_string(remote)
        .map_err(|e| format!("Failed to read REMOTE '{}': {}", remote.display(), e))?;

    Ok(format!(
        r#"# Merge Strategy

{strategy}

## Files

### BASE (common ancestor)
```
{base_content}
```

### LOCAL (current branch)
```
{local_content}
```

### REMOTE (incoming changes)
```
{remote_content}
```

## Instructions

Merge the LOCAL and REMOTE changes relative to BASE, following the strategy above.
Output ONLY the merged file content. No explanation, no markdown fences, no extra text."#
    ))
}
