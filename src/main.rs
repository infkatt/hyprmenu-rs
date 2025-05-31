use gdk::Key;
use gtk::prelude::*;
use gtk::{
    glib, Application, ApplicationWindow, Box as GtkBox, Button, CssProvider, EventControllerKey,
    Grid, Label, Orientation, Stack,
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
            
            window {
                background: linear-gradient(135deg, 
                    rgb(15, 15, 20) 0%, 
                    rgb(25, 25, 35) 50%, 
                    rgb(20, 20, 30) 100%);
                border-radius: 16px;
                border: 2px solid rgb(80, 80, 100);
                box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
                animation: fadeIn 0.2s ease-out;
            }
            
            window.csd {
                border-radius: 16px;
                border: 2px solid rgb(80, 80, 100);
            }
            
            .title-text {
                color: #f1ff5e;
                font-size: 12pt;
                font-weight: 700;
                margin: 8px 0;
                text-shadow: 0 1px 2px rgba(0, 0, 0, 0.5);
            }
            
            button {
                background: linear-gradient(135deg, 
                    rgb(40, 40, 55) 0%, 
                    rgb(55, 55, 75) 50%, 
                    rgb(45, 45, 65) 100%);
                border: 1px solid rgb(90, 90, 110);
                border-radius: 12px;
                color: rgb(255, 255, 255);
                font-weight: 600;
                font-size: 10px;
                margin: 3px;
                padding: 8px 6px;
                min-width: 85px;
                min-height: 45px;
                transition: all 0.15s cubic-bezier(0.4, 0, 0.2, 1);
                box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
                text-shadow: 0 1px 2px rgba(0, 0, 0, 0.7);
            }
            
            button:hover {
                background: linear-gradient(135deg, 
                    rgb(60, 60, 85) 0%, 
                    rgb(75, 75, 105) 50%, 
                    rgb(65, 65, 95) 100%);
                border-color: rgb(120, 120, 140);
                transform: translateY(-1px);
                box-shadow: 0 4px 16px rgba(0, 0, 0, 0.4);
                color: rgb(255, 255, 255);
            }
            
            button:active {
                background: linear-gradient(135deg, 
                    rgb(80, 80, 105) 0%, 
                    rgb(95, 95, 125) 50%, 
                    rgb(85, 85, 115) 100%);
                transform: translateY(0px) scale(0.98);
                box-shadow: 0 2px 8px rgba(0, 0, 0, 0.5);
            }
            
            button:focus {
                outline: 2px solid #1aff28;
                outline-offset: 2px;
            }
            
            .shortcut-hint {
                margin-top: 2px;
                font-size: 11px;
            }
            
            .help-label {
                color: rgba(200, 200, 200, 0.8);
                font-size: 8px;
                margin: 6px 0;
            }
            
            .help-content {
                color: rgba(220, 220, 220, 0.95);
                font-size: 10px;
                margin: 15px;
                font-family: monospace;
            }
        ",
        );

        gtk::style_context_add_provider_for_display(
            &gdk::Display::default().unwrap(),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    fn create_main_view(&self, _stack: &Stack, window: &ApplicationWindow) -> GtkBox {
        let main_box = GtkBox::new(Orientation::Vertical, 0);
        main_box.set_homogeneous(false);
        main_box.set_vexpand(true); // Make the main box expand vertically
        main_box.set_hexpand(true); // Make the main box expand horizontally

        // Compact title
        let title_label = Label::new(None);
        title_label.set_markup("⚡ HYPRMENU ⚡");
        title_label.add_css_class("title-text");
        title_label.set_halign(gtk::Align::Center);
        title_label.set_valign(gtk::Align::Start);

        // Grid container with better expansion
        let grid = Grid::builder()
            .row_spacing(8) // Increased spacing
            .column_spacing(8) // Increased spacing
            .margin_top(15) // More top margin
            .margin_bottom(15) // More bottom margin
            .margin_start(20) // More side margins
            .margin_end(20) // More side margins
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center) // Center the grid vertically
            .vexpand(true) // Make grid expand vertically
            .build();

        // Keyboard shortcuts mapping
        let shortcuts = ['a', 's', 'd', 'f', 'h', 'j', 'k', 'l'];

        for (index, command_entry) in self.commands.iter().enumerate() {
            let button_box = GtkBox::builder()
                .orientation(gtk::Orientation::Vertical)
                .spacing(2)
                .halign(gtk::Align::Center)
                .build();

            let button = Button::with_label(&command_entry.label);

            // Restore colored shortcut markup
            let shortcut_markup = format!(
                "<span color=\"#f1ff5e\" size=\"11pt\">[</span><span color=\"#ff41aa\" size=\"12pt\" weight=\"bold\">{}</span><span color=\"#f1ff5e\" size=\"11pt\">]</span>", 
                shortcuts[index]
            );
            let shortcut_label = Label::new(None);
            shortcut_label.set_markup(&shortcut_markup);
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

        // Help label at bottom
        let help_label = Label::new(Some("Navigate: a s d f h j k l • Close: Esc • Help: ?"));
        help_label.add_css_class("help-label");
        help_label.set_halign(gtk::Align::Center);
        help_label.set_valign(gtk::Align::End);

        main_box.append(&title_label);
        main_box.append(&grid);
        main_box.append(&help_label);

        main_box
    }

    fn create_help_view(&self) -> GtkBox {
        let help_box = GtkBox::new(Orientation::Vertical, 0);

        // Style the help title the same as main title
        let help_title = Label::new(None);
        help_title.set_markup("⚡ HYPRMENU - HELP ⚡");
        help_title.add_css_class("title-text"); // Use same class as main title
        help_title.set_halign(gtk::Align::Center);
        help_title.set_margin_top(10);

        // Updated help text without command layout section
        let help_text = "
    <b><span color=\"#ffffff\" size=\"11pt\">Keyboard Navigation:</span></b>
    • <span color=\"#ff41aa\" weight=\"bold\">a,s,d,f,h,j,k,l</span> - Execute corresponding command
    • <span color=\"#1aff28\" weight=\"bold\">Escape</span> - Close menu / Return to main view
    • <span color=\"#f1ff5e\" weight=\"bold\">?</span> - Show/hide this help

    <b><span color=\"#ffffff\" size=\"11pt\">Mouse Navigation:</span></b>
    • Click any button to execute command

    <b><span color=\"#ffffff\" size=\"11pt\">Configuration:</span></b>
    • Config file: <span color=\"#f1ff5e\">~/.config/hyprmenu/commands.json</span>
    • Edit the JSON file to customize commands
    • Restart hyprmenu to reload configuration

    <b><span color=\"#ffffff\" size=\"11pt\">Tips:</span></b>
    • Use keyboard shortcuts for fastest navigation
    • Commands execute immediately and close the menu
    • Customize your workflow by editing the config file
    • Press Escape to quickly close without executing anything

    <span color=\"#f1ff5e\" weight=\"bold\">Press Escape to return to main menu</span>";

        let help_content = Label::new(None);
        help_content.set_markup(help_text);
        help_content.add_css_class("help-content");
        help_content.set_justify(gtk::Justification::Left);
        help_content.set_margin_start(15);
        help_content.set_margin_end(15);
        help_content.set_margin_bottom(10);

        help_box.append(&help_title);
        help_box.append(&help_content);

        help_box
    }

    fn build_ui(&self, app: &Application) {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("hyprmenu")
            .default_width(500)
            .default_height(180) // Reduced from 300 to 180
            .resizable(false)
            .decorated(false)
            .build();

        // Force the exact size and prevent expansion
        window.set_size_request(500, 180); // Reduced height
        window.set_resizable(false);
        window.add_css_class("csd");

        // Create stack for switching between main and help views
        let stack = Stack::new();

        // Create main view
        let main_view = self.create_main_view(&stack, &window);
        stack.add_named(&main_view, Some("main"));

        // Create help view
        let help_view = self.create_help_view();
        stack.add_named(&help_view, Some("help"));

        // Set initial view
        stack.set_visible_child_name("main");

        window.set_child(Some(&stack));

        // Setup keyboard shortcuts (keeping the same key handler code...)
        let key_controller = EventControllerKey::new();
        let commands_clone = self.commands.clone();
        let window_clone = window.clone();
        let stack_clone = stack.clone();

        key_controller.connect_key_pressed(move |_, key, _, _| {
            let current_view = stack_clone.visible_child_name();

            match key {
                Key::Escape => {
                    if current_view.as_ref().map(|s| s.as_str()) == Some("help") {
                        stack_clone.set_visible_child_name("main");
                    } else {
                        window_clone.close();
                    }
                    glib::Propagation::Stop
                }
                Key::question => {
                    if current_view.as_ref().map(|s| s.as_str()) == Some("main") {
                        stack_clone.set_visible_child_name("help");
                    } else {
                        stack_clone.set_visible_child_name("main");
                    }
                    glib::Propagation::Stop
                }
                _ => {
                    // Only handle command keys when on main view
                    if current_view.as_ref().map(|s| s.as_str()) == Some("main") {
                        match key {
                            Key::a => {
                                if let Some(cmd) = commands_clone.get(0) {
                                    let command = cmd.command.clone();
                                    gio::spawn_blocking(move || {
                                        let _result =
                                            Command::new("sh").arg("-c").arg(&command).spawn();
                                    });
                                    window_clone.close();
                                }
                                glib::Propagation::Stop
                            }
                            Key::s => {
                                if let Some(cmd) = commands_clone.get(1) {
                                    let command = cmd.command.clone();
                                    gio::spawn_blocking(move || {
                                        let _result =
                                            Command::new("sh").arg("-c").arg(&command).spawn();
                                    });
                                    window_clone.close();
                                }
                                glib::Propagation::Stop
                            }
                            Key::d => {
                                if let Some(cmd) = commands_clone.get(2) {
                                    let command = cmd.command.clone();
                                    gio::spawn_blocking(move || {
                                        let _result =
                                            Command::new("sh").arg("-c").arg(&command).spawn();
                                    });
                                    window_clone.close();
                                }
                                glib::Propagation::Stop
                            }
                            Key::f => {
                                if let Some(cmd) = commands_clone.get(3) {
                                    let command = cmd.command.clone();
                                    gio::spawn_blocking(move || {
                                        let _result =
                                            Command::new("sh").arg("-c").arg(&command).spawn();
                                    });
                                    window_clone.close();
                                }
                                glib::Propagation::Stop
                            }
                            Key::h => {
                                if let Some(cmd) = commands_clone.get(4) {
                                    let command = cmd.command.clone();
                                    gio::spawn_blocking(move || {
                                        let _result =
                                            Command::new("sh").arg("-c").arg(&command).spawn();
                                    });
                                    window_clone.close();
                                }
                                glib::Propagation::Stop
                            }
                            Key::j => {
                                if let Some(cmd) = commands_clone.get(5) {
                                    let command = cmd.command.clone();
                                    gio::spawn_blocking(move || {
                                        let _result =
                                            Command::new("sh").arg("-c").arg(&command).spawn();
                                    });
                                    window_clone.close();
                                }
                                glib::Propagation::Stop
                            }
                            Key::k => {
                                if let Some(cmd) = commands_clone.get(6) {
                                    let command = cmd.command.clone();
                                    gio::spawn_blocking(move || {
                                        let _result =
                                            Command::new("sh").arg("-c").arg(&command).spawn();
                                    });
                                    window_clone.close();
                                }
                                glib::Propagation::Stop
                            }
                            Key::l => {
                                if let Some(cmd) = commands_clone.get(7) {
                                    let command = cmd.command.clone();
                                    gio::spawn_blocking(move || {
                                        let _result =
                                            Command::new("sh").arg("-c").arg(&command).spawn();
                                    });
                                    window_clone.close();
                                }
                                glib::Propagation::Stop
                            }
                            _ => glib::Propagation::Proceed,
                        }
                    } else {
                        glib::Propagation::Proceed
                    }
                }
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
