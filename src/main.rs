mod app;
mod config;
mod styling;
mod ui;

use app::{QuickMenuApp, APP_ID};
use gtk::prelude::*; // Add this import
use gtk::{glib, Application};

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(|app| {
        let quickmenu = QuickMenuApp::new();
        quickmenu.setup_styling();
        quickmenu.build_ui(app);
    });

    app.run()
}
