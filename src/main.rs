use gdk::Key;
use gtk::prelude::*;
use gtk::{
    glib, Application, ApplicationWindow, Button, CssProvider, EventControllerKey, Grid, Label,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const APP_ID: &str = "org.hyprmenu.app";

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
    _config_path: PathBuf,
}

impl QuickMenuApp {
    fn new() -> Self {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("~/.config"))
            .join("hyprmenu");

        if let Err(e) = fs::create_dir_all(&config_dir) {
            eprintln!("Failed to create config directory: {}", e);
        }

        let config_path = config_dir.join("commands.json");
        let commands = Self::load_config(&config_path);

        Self {
            commands,
            _config_path: config_path,
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
            @keyframes fadeIn {
                from { opacity: 0; transform: scale(0.95); }
                to { opacity: 1; transform: scale(1.0); }
            }
            
            @keyframes buttonPress {
                0% { transform: scale(1.0); }
                50% { transform: scale(0.95); }
                100% { transform: scale(1.0); }
            }
            
            window {
                background: linear-gradient(135deg, 
                    rgb(30, 30, 35) 0%, 
                    rgb(45, 45, 55) 50%, 
                    rgb(35, 35, 45) 100%);
                border-radius: 16px;
                border: 2px solid rgb(120, 120, 140);
                box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
                animation: fadeIn 0.2s ease-out;
            }
            
            window.csd {
                border-radius: 16px;
                border: 2px solid rgb(120, 120, 140);
            }
            
            button {
                background: linear-gradient(135deg, 
                    rgb(70, 70, 85) 0%, 
                    rgb(90, 90, 110) 50%, 
                    rgb(75, 75, 95) 100%);
                border: 1px solid rgb(140, 140, 160);
                border-radius: 12px;
                color: rgb(255, 255, 255);
                font-weight: 600;
                font-size: 11px;
                margin: 3px;
                padding: 8px 6px;
                min-width: 85px;
                min-height: 52px;
                transition: all 0.15s cubic-bezier(0.4, 0, 0.2, 1);
                box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
                text-shadow: 0 1px 2px rgba(0, 0, 0, 0.5);
            }
            
            button:hover {
                background: linear-gradient(135deg, 
                    rgb(100, 100, 125) 0%, 
                    rgb(120, 120, 145) 50%, 
                    rgb(105, 105, 135) 100%);
                border-color: rgb(180, 180, 200);
                transform: translateY(-1px);
                box-shadow: 0 4px 16px rgba(0, 0, 0, 0.3);
                color: rgb(255, 255, 255);
            }
            
            button:active {
                background: linear-gradient(135deg, 
                    rgb(120, 120, 145) 0%, 
                    rgb(140, 140, 165) 50%, 
                    rgb(125, 125, 155) 100%);
                transform: translateY(0px) scale(0.98);
                box-shadow: 0 2px 8px rgba(0, 0, 0, 0.4);
                animation: buttonPress 0.1s ease-out;
            }
            
            button:focus {
                outline: 2px solid #1aff28;
                outline-offset: 2px;
            }
            
            .shortcut-hint {
                color: #f1ff5e;
                font-size: 9px;
                font-weight: 700;
                margin-top: 2px;
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
            .title("hyprmenu")
            .default_width(420)
            .default_height(160)
            .resizable(false)
            .decorated(false)
            .build();

        window.add_css_class("csd");

        let grid = Grid::builder()
            .row_spacing(6)
            .column_spacing(6)
            .margin_top(20)
            .margin_bottom(20)
            .margin_start(20)
            .margin_end(20)
            .build();

        // Keyboard shortcuts mapping
        let shortcuts = ['a', 's', 'd', 'f', 'h', 'j', 'k', 'l'];

        for (index, command_entry) in self.commands.iter().enumerate() {
            let button_box = gtk::Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .spacing(0)
                .build();

            let button = Button::with_label(&command_entry.label);
            let shortcut_label = Label::new(Some(&format!("[{}]", shortcuts[index])));
            shortcut_label.add_css_class("shortcut-hint");

            button_box.append(&button);
            button_box.append(&shortcut_label);

            let command_clone = command_entry.command.clone();
            let window_clone = window.clone();
            button.connect_clicked(move |_| {
                let cmd = command_clone.clone();

                gio::spawn_blocking(move || {
                    let _result = Command::new("sh").arg("-c").arg(&cmd).spawn();
                });

                window_clone.close();
            });

            let row = (index / 4) as i32;
            let col = (index % 4) as i32;
            grid.attach(&button_box, col, row, 1, 1);
        }

        window.set_child(Some(&grid));

        // Setup keyboard shortcuts
        let key_controller = EventControllerKey::new();
        let commands_clone = self.commands.clone();
        let window_clone = window.clone();

        key_controller.connect_key_pressed(move |_, key, _, _| match key {
            Key::Escape => {
                window_clone.close();
                glib::Propagation::Stop
            }
            Key::a => {
                if let Some(cmd) = commands_clone.get(0) {
                    let command = cmd.command.clone();
                    gio::spawn_blocking(move || {
                        let _result = Command::new("sh").arg("-c").arg(&command).spawn();
                    });
                    window_clone.close();
                }
                glib::Propagation::Stop
            }
            Key::s => {
                if let Some(cmd) = commands_clone.get(1) {
                    let command = cmd.command.clone();
                    gio::spawn_blocking(move || {
                        let _result = Command::new("sh").arg("-c").arg(&command).spawn();
                    });
                    window_clone.close();
                }
                glib::Propagation::Stop
            }
            Key::d => {
                if let Some(cmd) = commands_clone.get(2) {
                    let command = cmd.command.clone();
                    gio::spawn_blocking(move || {
                        let _result = Command::new("sh").arg("-c").arg(&command).spawn();
                    });
                    window_clone.close();
                }
                glib::Propagation::Stop
            }
            Key::f => {
                if let Some(cmd) = commands_clone.get(3) {
                    let command = cmd.command.clone();
                    gio::spawn_blocking(move || {
                        let _result = Command::new("sh").arg("-c").arg(&command).spawn();
                    });
                    window_clone.close();
                }
                glib::Propagation::Stop
            }
            Key::h => {
                if let Some(cmd) = commands_clone.get(4) {
                    let command = cmd.command.clone();
                    gio::spawn_blocking(move || {
                        let _result = Command::new("sh").arg("-c").arg(&command).spawn();
                    });
                    window_clone.close();
                }
                glib::Propagation::Stop
            }
            Key::j => {
                if let Some(cmd) = commands_clone.get(5) {
                    let command = cmd.command.clone();
                    gio::spawn_blocking(move || {
                        let _result = Command::new("sh").arg("-c").arg(&command).spawn();
                    });
                    window_clone.close();
                }
                glib::Propagation::Stop
            }
            Key::k => {
                if let Some(cmd) = commands_clone.get(6) {
                    let command = cmd.command.clone();
                    gio::spawn_blocking(move || {
                        let _result = Command::new("sh").arg("-c").arg(&command).spawn();
                    });
                    window_clone.close();
                }
                glib::Propagation::Stop
            }
            Key::l => {
                if let Some(cmd) = commands_clone.get(7) {
                    let command = cmd.command.clone();
                    gio::spawn_blocking(move || {
                        let _result = Command::new("sh").arg("-c").arg(&command).spawn();
                    });
                    window_clone.close();
                }
                glib::Propagation::Stop
            }
            _ => glib::Propagation::Proceed,
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
