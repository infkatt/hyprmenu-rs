use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct CommandEntry {
    pub label: String,
    pub command: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub commands: Vec<CommandEntry>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            commands: vec![
                CommandEntry {
                    label: "Terminal".to_string(),
                    command: "kitty".to_string(),
                },
                CommandEntry {
                    label: "Firefox".to_string(),
                    command: "firefox".to_string(),
                },
                CommandEntry {
                    label: "Files".to_string(),
                    command: "thunar".to_string(),
                },
                CommandEntry {
                    label: "VS Code".to_string(),
                    command: "code".to_string(),
                },
                CommandEntry {
                    label: "Spotify".to_string(),
                    command: "spotify".to_string(),
                },
                CommandEntry {
                    label: "Discord".to_string(),
                    command: "discord".to_string(),
                },
                CommandEntry {
                    label: "Screenshot".to_string(),
                    command: "grim -g \"$(slurp)\" - | wl-copy".to_string(),
                },
                CommandEntry {
                    label: "Lock".to_string(),
                    command: "hyprlock".to_string(),
                },
            ],
        }
    }
}

pub fn load_config(config_path: &PathBuf) -> Vec<CommandEntry> {
    match fs::read_to_string(config_path) {
        Ok(content) => match serde_json::from_str::<Config>(&content) {
            Ok(config) => {
                if config.commands.len() == 8 {
                    config.commands
                } else {
                    eprintln!("Config must contain exactly 8 commands. Using defaults.");
                    let default_config = Config::default();
                    save_config(config_path, &default_config);
                    default_config.commands
                }
            }
            Err(e) => {
                eprintln!("Failed to parse config: {}. Using defaults.", e);
                let default_config = Config::default();
                save_config(config_path, &default_config);
                default_config.commands
            }
        },
        Err(_) => {
            let default_config = Config::default();
            save_config(config_path, &default_config);
            default_config.commands
        }
    }
}

pub fn save_config(config_path: &PathBuf, config: &Config) {
    match serde_json::to_string_pretty(config) {
        Ok(json) => {
            if let Err(e) = fs::write(config_path, json) {
                eprintln!("Failed to save config: {}", e);
            } else {
                println!("Config saved to: {}", config_path.display());
            }
        }
        Err(e) => eprintln!("Failed to serialize config: {}", e),
    }
}
