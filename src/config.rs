use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const DEFAULT_STRATEGY: &str =
    "Try to resolve conflicts in a way that the REMOTE changes are preserved as much as possible.";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub agent_command: String,
    #[serde(default)]
    pub agent_args: Vec<String>,
    pub strategy_file: Option<String>,
    pub default_strategy: String,
    pub fallback_merge_tool: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            agent_command: "claude".to_string(),
            agent_args: vec!["--dangerously-skip-permissions".to_string(), "-".to_string()],
            strategy_file: None,
            default_strategy: DEFAULT_STRATEGY.to_string(),
            fallback_merge_tool: None,
        }
    }
}

fn default_config_path() -> PathBuf {
    dirs::home_dir()
        .expect("Could not determine home directory")
        .join(".ai-mergetool-config.json")
}

pub fn load_or_create(config_path: Option<&Path>) -> Config {
    let path = config_path
        .map(PathBuf::from)
        .unwrap_or_else(default_config_path);

    if path.exists() {
        let content = std::fs::read_to_string(&path).unwrap_or_else(|e| {
            eprintln!("Warning: could not read config at {}: {}", path.display(), e);
            String::new()
        });

        if !content.is_empty() {
            match json5::from_str::<Config>(&content) {
                Ok(config) => {
                    eprintln!("Loaded config from {}", path.display());
                    return config;
                }
                Err(e) => {
                    eprintln!("Warning: could not parse config: {}. Using defaults.", e);
                }
            }
        }
    }

    let config = Config::default();
    let json = serde_json::to_string_pretty(&config).expect("Failed to serialize default config");

    if let Err(e) = std::fs::write(&path, &json) {
        eprintln!("Warning: could not write default config to {}: {}", path.display(), e);
    } else {
        eprintln!("Generated default config at {}", path.display());
    }

    config
}
