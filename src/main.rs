use gdk::Key;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, CssProvider, EventControllerKey, Grid};
use std::sync::OnceLock;
use tokio::runtime::{Builder, Runtime};

const APP_ID: &str = "org.example.QuickMenu";

fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| {
        Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Setting up tokio runtime needs to succeed.")
    })
}

struct QuickMenuApp {
    commands: Vec<(&'static str, &'static str)>,
}

impl QuickMenuApp {
    fn new() -> Self {
        Self {
            commands: vec![
                ("Terminal", "kitty"),
                ("Firefox", "firefox"),
                ("Files", "thunar"),
                ("VS Code", "code"),
                ("Spotify", "spotify"),
                ("Discord", "discord"),
                ("Screenshot", "grim -g \"$(slurp)\" - | wl-copy"),
                ("Lock", "hyprlock"),
            ],
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
                min-width: 120px;
                min-height: 40px;
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
            .default_width(280)
            .default_height(240)
            .resizable(false)
            .decorated(false)
            .build();

        let grid = Grid::builder()
            .row_spacing(8)
            .column_spacing(8)
            .margin_top(16)
            .margin_bottom(16)
            .margin_start(16)
            .margin_end(16)
            .build();

        for (index, (label, command)) in self.commands.iter().enumerate() {
            let button = Button::with_label(label);
            let command_clone = command.to_string();
            let window_clone = window.clone();

            button.connect_clicked(move |_| {
                let cmd = command_clone.clone();

                runtime().spawn(async move {
                    let _result = tokio::process::Command::new("sh")
                        .arg("-c")
                        .arg(&cmd)
                        .spawn();
                });

                window_clone.close();
            });

            let row = (index / 2) as i32;
            let col = (index % 2) as i32;
            grid.attach(&button, col, row, 1, 1);
        }

        window.set_child(Some(&grid));

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
