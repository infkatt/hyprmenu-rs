use gtk::prelude::*;
use gtk::{ApplicationWindow, Box as GtkBox, Button, Grid, Label, Orientation, Stack};
use std::process::Command;

use crate::config::CommandEntry;

pub fn create_main_view(
    commands: &[CommandEntry],
    _stack: &Stack,
    window: &ApplicationWindow,
) -> GtkBox {
    let main_box = GtkBox::new(Orientation::Vertical, 0);
    main_box.set_homogeneous(false);
    main_box.set_vexpand(true);
    main_box.set_hexpand(true);

    // Title
    let title_label = Label::new(None);
    title_label.set_markup("⚡ HYPRMENU ⚡");
    title_label.add_css_class("title-text");
    title_label.set_halign(gtk::Align::Center);
    title_label.set_valign(gtk::Align::Start);

    // Grid container
    let grid = Grid::builder()
        .row_spacing(8)
        .column_spacing(8)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(20)
        .margin_end(20)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .vexpand(true)
        .build();

    let shortcuts = ['a', 's', 'd', 'f', 'h', 'j', 'k', 'l'];

    for (index, command_entry) in commands.iter().enumerate() {
        let button_box = GtkBox::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(2)
            .halign(gtk::Align::Center)
            .build();

        let button = Button::with_label(&command_entry.label);

        // Restore the colored shortcut markup
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

    // Help label - now larger and more visible
    let help_label = Label::new(Some("Navigate: a s d f h j k l • Close: Esc • Help: ?"));
    help_label.add_css_class("help-label");
    help_label.set_halign(gtk::Align::Center);
    help_label.set_valign(gtk::Align::End);

    main_box.append(&title_label);
    main_box.append(&grid);
    main_box.append(&help_label);

    main_box
}

pub fn create_help_view() -> GtkBox {
    let help_box = GtkBox::new(Orientation::Vertical, 0);

    let help_title = Label::new(None);
    help_title.set_markup("⚡ HYPRMENU - HELP ⚡");
    help_title.add_css_class("title-text");
    help_title.set_halign(gtk::Align::Center);
    help_title.set_margin_top(10);

    // Restore colored help text with increased font size
    let help_text = "
<b><span color=\"#ffffff\" size=\"12pt\">Keyboard Navigation:</span></b>
• <span color=\"#ff41aa\" weight=\"bold\">a,s,d,f,h,j,k,l</span> - Execute corresponding command
• <span color=\"#1aff28\" weight=\"bold\">Escape</span> - Close menu / Return to main view
• <span color=\"#f1ff5e\" weight=\"bold\">?</span> - Show/hide this help

<b><span color=\"#ffffff\" size=\"12pt\">Mouse Navigation:</span></b>
• Click any button to execute command

<b><span color=\"#ffffff\" size=\"12pt\">Configuration:</span></b>
• Config file: <span color=\"#f1ff5e\">~/.config/hyprmenu/commands.json</span>
• Edit the JSON file to customize commands
• Restart hyprmenu to reload configuration

<b><span color=\"#ffffff\" size=\"12pt\">Tips:</span></b>
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
