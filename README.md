# Pane View

A modern window management tool built with Tauri.

## Features

- 🚀 Built with Tauri for optimal performance
- 🎯 Drag and drop support
- 🔒 Secure system integration
- 🎨 Modern user interface
- 💻 Cross-platform support (Windows, macOS, Linux)

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
  - `src/lib.rs` - Core library code
  - `Cargo.toml` - Rust project configuration and dependencies

## Features

- Drag and drop support
- Protocol integration
- Developer tools
- Transparent window support
- Fullscreen mode
- Linux-specific features
- macOS proxy support
- System WebView integration

## Contributing

Pull Requests and Issues are welcome!

## License

[MIT License](LICENSE)
