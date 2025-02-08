# Pane View

A modern window management tool built with Tauri.

## Features

- 🚀 Built with Tauri for optimal performance
- 📊 Multi-pane TradingView integration
- 🔄 Persistent web context with local data storage
- ⌨️ Keyboard shortcuts support (F11 for fullscreen toggle)
- 🖱️ Custom right-click menu handling
- 🎨 Modern split-pane interface
- 💻 Cross-platform support (Windows, macOS, Linux)

## Technical Details

### Window Management
- Split-pane layout with dynamic resizing
- Each pane takes exactly half of the window width
- Automatic layout adjustment on window resize
- Fullscreen toggle support with F11 key

### Web Integration
- Unified web context across panes
- Persistent data storage in `~/.config/pane-view/` (Linux)
- Custom JavaScript injection for UI customization
- Default TradingView chart configuration

### Default URLs
- Left pane: TradingView JP225 chart
- Right pane: TradingView ETHBTC chart

## Tech Stack

- **Backend**: Rust + Tauri
- **Frontend**: HTML, CSS, JavaScript (Vanilla)
- **System Integration**:
  - WebKit2GTK
  - GTK
  - JavaScriptCore

## Development Setup

### Prerequisites

- [Rust](https://www.rust-lang.org/) (Latest stable version)
- System Dependencies:
  - Linux: WebKit2GTK and other GTK dependencies
  - macOS: Xcode Command Line Tools
  - Windows: Microsoft Visual Studio C++ Build Tools

### Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) 
  - [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
  - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Getting Started

1. Clone the repository
```bash
git clone <repository-url>
cd pane-view
```

2. Install dependencies
```bash
# Install Rust dependencies
cargo check
```

3. Run development environment
```bash
cargo tauri dev
```

4. Build for production
```bash
cargo tauri build
```

## Project Structure

- `/src` - Frontend source code
- `/src-tauri` - Rust backend code
  - `src/lib.rs` - Core window management and webview integration
  - `src/main.rs` - Application entry point
  - `Cargo.toml` - Rust project configuration and dependencies

## Contributing

Pull Requests and Issues are welcome!

## License

[MIT License](LICENSE)
