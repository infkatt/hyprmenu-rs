use gdk::Key;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, EventControllerKey, Stack};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::config::{load_config, CommandEntry};
use crate::styling::setup_styling;
use crate::ui::{create_help_view, create_main_view};

pub const APP_ID: &str = "org.hyprmenu.app";

pub struct QuickMenuApp {
    commands: Vec<CommandEntry>,
    _config_path: PathBuf,
}

impl QuickMenuApp {
    pub fn new() -> Self {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("~/.config"))
            .join("hyprmenu");

        if let Err(e) = fs::create_dir_all(&config_dir) {
            eprintln!("Failed to create config directory: {}", e);
        }

        let config_path = config_dir.join("commands.json");
        let commands = load_config(&config_path);

        Self {
            commands,
            _config_path: config_path,
        }
    }

    pub fn setup_styling(&self) {
        setup_styling();
    }

    pub fn build_ui(&self, app: &Application) {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("hyprmenu")
            .default_width(500)
            .default_height(180)
            .resizable(false)
            .decorated(false)
            .build();

        window.set_size_request(500, 180);
        window.set_resizable(false);
        window.add_css_class("csd");

        let stack = Stack::new();

        let main_view = create_main_view(&self.commands, &stack, &window);
        stack.add_named(&main_view, Some("main"));

        let help_view = create_help_view();
        stack.add_named(&help_view, Some("help"));

        stack.set_visible_child_name("main");
        window.set_child(Some(&stack));

        // Fix the lifetime issue by cloning commands instead of capturing self
        setup_keyboard_shortcuts(&self.commands, &window, &stack);
        window.present();
    }
}

// Move this function outside of impl block to avoid lifetime issues
fn setup_keyboard_shortcuts(commands: &[CommandEntry], window: &ApplicationWindow, stack: &Stack) {
    let key_controller = EventControllerKey::new();
    let commands_clone = commands.to_vec(); // Clone the commands
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
                if current_view.as_ref().map(|s| s.as_str()) == Some("main") {
                    handle_command_key(key, &commands_clone, &window_clone)
                } else {
                    glib::Propagation::Proceed
                }
            }
        }
    });

    window.add_controller(key_controller);
}

// Move this function outside impl block as well
fn handle_command_key(
    key: Key,
    commands: &[CommandEntry],
    window: &ApplicationWindow,
) -> glib::Propagation {
    let command_index = match key {
        Key::a => Some(0),
        Key::s => Some(1),
        Key::d => Some(2),
        Key::f => Some(3),
        Key::h => Some(4),
        Key::j => Some(5),
        Key::k => Some(6),
        Key::l => Some(7),
        _ => None,
    };

    if let Some(index) = command_index {
        if let Some(cmd) = commands.get(index) {
            let command = cmd.command.clone();
            gio::spawn_blocking(move || {
                let _result = Command::new("sh").arg("-c").arg(&command).spawn();
            });
            window.close();
            return glib::Propagation::Stop;
        }
    }

    glib::Propagation::Proceed
}
