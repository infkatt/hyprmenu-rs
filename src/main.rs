use gdk::Key;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, CssProvider, EventControllerKey, Grid};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const APP_ID: &str = "org.example.QuickMenu";

#[derive(Serialize, Deserialize, Clone)]
struct CommandEntry {
    label: String,
    command: String,
}

#[derive(Serialize, Deserialize)]
struct Config {
    commands: Vec<CommandEntry>,
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

struct QuickMenuApp {
    commands: Vec<CommandEntry>,
    config_path: PathBuf,
}

impl QuickMenuApp {
    fn new() -> Self {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("~/.config"))
            .join("quickmenu");

        // Create config directory if it doesn't exist
        if let Err(e) = fs::create_dir_all(&config_dir) {
            eprintln!("Failed to create config directory: {}", e);
        }

        let config_path = config_dir.join("commands.json");
        let commands = Self::load_config(&config_path);

        Self {
            commands,
            config_path,
        }
    }

    fn load_config(config_path: &PathBuf) -> Vec<CommandEntry> {
        match fs::read_to_string(config_path) {
            Ok(content) => match serde_json::from_str::<Config>(&content) {
                Ok(config) => {
                    if config.commands.len() == 8 {
                        config.commands
                    } else {
                        eprintln!("Config must contain exactly 8 commands. Using defaults.");
                        let default_config = Config::default();
                        Self::save_config(config_path, &default_config);
                        default_config.commands
                    }
                }
                Err(e) => {
                    eprintln!("Failed to parse config: {}. Using defaults.", e);
                    let default_config = Config::default();
                    Self::save_config(config_path, &default_config);
                    default_config.commands
                }
            },
            Err(_) => {
                // Config file doesn't exist, create default
                let default_config = Config::default();
                Self::save_config(config_path, &default_config);
                default_config.commands
            }
        }
    }

    fn save_config(config_path: &PathBuf, config: &Config) {
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

    fn setup_styling(&self) {
        let provider = CssProvider::new();
        provider.load_from_string(
            "
            window {
                background-color: rgba(40, 40, 40, 0.95);
                border-radius: 12px;
                border: 2px solid rgba(100, 100, 100, 0.3);
            }
            
            button {
                background-color: rgba(60, 60, 60, 0.8);
                border: 1px solid rgba(120, 120, 120, 0.5);
                border-radius: 8px;
                color: white;
                font-weight: bold;
                margin: 4px;
                padding: 12px;
                min-width: 90px;
                min-height: 50px;
            }
            
            button:hover {
                background-color: rgba(80, 80, 80, 0.9);
                border-color: rgba(150, 150, 150, 0.7);
            }
            
            button:active {
                background-color: rgba(100, 100, 100, 0.8);
            }
        ",
        );

        gtk::style_context_add_provider_for_display(
            &gdk::Display::default().unwrap(),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    fn build_ui(&self, app: &Application) {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("QuickMenu")
            .default_width(400)
            .default_height(140)
            .resizable(false)
            .decorated(false)
            .build();

        // Create 4x2 grid for 8 buttons (4 columns, 2 rows)
        let grid = Grid::builder()
            .row_spacing(8)
            .column_spacing(8)
            .margin_top(16)
            .margin_bottom(16)
            .margin_start(16)
            .margin_end(16)
            .build();

        // Create buttons for each command
        for (index, command_entry) in self.commands.iter().enumerate() {
            let button = Button::with_label(&command_entry.label);
            let command_clone = command_entry.command.clone();
            let window_clone = window.clone();

            button.connect_clicked(move |_| {
                let cmd = command_clone.clone();

                std::thread::spawn(move || {
                    let _result = Command::new("sh").arg("-c").arg(&cmd).spawn();
                });

                window_clone.close();
            });

            // 4x2 layout: 4 columns, 2 rows
            let row = (index / 4) as i32;
            let col = (index % 4) as i32;
            grid.attach(&button, col, row, 1, 1);
        }

        window.set_child(Some(&grid));

        // Setup Escape key
        let key_controller = EventControllerKey::new();
        let window_clone = window.clone();
        key_controller.connect_key_pressed(move |_, key, _, _| {
            if key == Key::Escape {
                window_clone.close();
                glib::Propagation::Stop
            } else {
                glib::Propagation::Proceed
            }
        });

        window.add_controller(key_controller);
        window.present();
    }
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(|app| {
        let quickmenu = QuickMenuApp::new();
        quickmenu.setup_styling();
        quickmenu.build_ui(app);
    });

    app.run()
}
