# hyprmenu-rs ⚡

A fast, lightweight floating command launcher for Hyprland written in Rust with GTK4.

- Rust
- GTK4

## Features

- **Keyboard-driven** - Navigate with `a,s,d,f,h,j,k,l` keys
- **Mouse support** - Click buttons for command execution
- **Highly customizable** - JSON configuration for all commands
- **Wayland native** - Built specifically for Hyprland compositor
- **Compact & Modern UI** - Dark theme with smooth animations
- **In-app help** - Press `?` for comprehensive usage guide

## Quick Start

### Prerequisites

- **Arch Linux** with Hyprland compositor
- **Wayland** environment
- **Rust** 1.70+ (for building from source)

### Installation

#### Build from Source

```bash
git clone https://github.com/yourusername/hyprmenu-rs.git
```

```bash
cd hyprmenu-rs
```

```bash
cargo build --release
```

```bash
sudo cp target/release/hyprmenu-rs /usr/local/bin/
```

### Hyprland Integration

Add to your `~/.config/hypr/hyprland.conf`:

```hyprlang
Bind Super+Space to launch hyprmenu
bind = SUPER, SPACE, exec, hyprmenu-rs
```

Window rules for optimal floating behavior

```hyprlang
windowrulev2 = float, class:^(org.hyprmenu.app)$
windowrulev2 = center, class:^(org.hyprmenu.app)$
windowrulev2 = stayfocused, class:^(org.hyprmenu.app)$
windowrulev2 = noborder, class:^(org.hyprmenu.app)$
windowrulev2 = noshadow, class:^(org.hyprmenu.app)$
windowrulev2 = noblur, class:^(org.hyprmenu.app)$
```

## Usage

### Keyboard Shortcuts

| Key | Command  | Key | Command    |
| --- | -------- | --- | ---------- |
| `a` | Terminal | `h` | Spotify    |
| `s` | Firefox  | `j` | Discord    |
| `d` | Files    | `k` | Screenshot |
| `f` | VS Code  | `l` | Lock       |

### Special Keys

- `Escape` - Close menu or return to main view from help
- `?` - Toggle help page
- `Mouse Click` - Execute command

## Configuration

hyprmenu-rs automatically creates a configuration file at `~/.config/hyprmenu/commands.json`:

```json
{
"commands": [
{
"label": "Terminal",
"command": "kitty"
},
{
"label": "Firefox",
"command": "firefox"
},
{
"label": "Files",
"command": "thunar"
},
{
"label": "VS Code",
"command": "code"
},
{
"label": "Spotify",
"command": "spotify"
},
{
"label": "Discord",
"command": "discord"
},
{
"label": "Screenshot",
"command": "grim -g "$(slurp)" - | wl-copy"
},
{
"label": "Lock",
"command": "hyprlock"
}
]
}
```

### Customization

1. Edit the configuration file: `vim ~/.config/hyprmenu/commands.json`
2. Modify any command label or command string
3. Restart hyprmenu-rs to reload configuration
4. **Note:** Exactly 8 commands are required

### Example Custom Commands

```json
{
"label": "File Manager",
"command": "thunar ~/Downloads"
},
{
"label": "System Monitor",
"command": "gnome-system-monitor"
},
{
"label": "Color Picker",
"command": "hyprpicker -a"
}
```

## Development

### Project Structure

```bash
src/
├── main.rs # Application entry point
├── app.rs # Main application logic and keyboard handling
├── config.rs # Configuration loading and saving
├── styling.rs # GTK4 CSS themes and animations
└── ui.rs # UI creation and component management
```

### Building

Debug build
`cargo build`

Release build (optimized)
`cargo build --release`

Run directly
`cargo run --release`

### Dependencies

- **GTK4** - Modern GUI framework
- **GDK4** - Platform abstraction layer
- **GIO** - Async I/O operations
- **GLib** - Core utilities
- **Serde** - JSON serialization
- **dirs** - Cross-platform directory utilities

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup

1. Fork the repository
2. Create your feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes following Rust best practices
4. Test on Hyprland environment
5. Commit your changes: `git commit -m 'Add amazing feature'`
6. Push to the branch: `git push origin feature/amazing-feature`
7. Open a Pull Request

### Code Style

- Follow standard Rust formatting: `cargo fmt`
- Ensure no warnings: `cargo clippy`
- Add documentation for public functions
- Test on Arch Linux with Hyprland

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- **Hyprland** - For the amazing Wayland compositor
- **GTK4** - For the modern GUI framework
- **Rust Community** - For excellent documentation and tools
- **Arch Linux** - For the rolling release development environment

**Made with ⚡ for the Hyprland community**
