# Rclone UI

A Terminal User Interface (TUI) for [rclone](https://rclone.org/) - manage your cloud storage with ease from the terminal.

## Features

- 🌥️ Browse and manage cloud storage remotes
- 🖥️ Interactive Terminal User Interface (TUI)
- 🔧 Command Line Interface (CLI)
- 🧪 Comprehensive test suite
- 🚀 CI/CD with GitHub Actions
- 📦 Cross-platform releases
- 🔒 Security auditing
- 🐳 Docker and Docker Compose support
- ❄️ Nix flakes for reproducible environments
- 📦 Devcontainer configuration for GitHub Codespaces

## Prerequisites

**Rclone must be installed and configured on your system.**

Install rclone:
- **macOS**: `brew install rclone`
- **Linux**: `curl https://rclone.org/install.sh | sudo bash`
- **Windows**: Download from [rclone.org/downloads](https://rclone.org/downloads/)
- **Nix**: Included in the development environment

Configure remotes:
```bash
rclone config
```

## Installation

> **💡 Quick Start**: See [SETUP.md](SETUP.md) for detailed setup instructions using Docker, Nix, Codespaces, or local development.

### From Source

```bash
git clone https://github.com/np2023v2/rclone-ui.git
cd rclone-ui
cargo build --release
```

### From Releases

Download the latest binary from the [Releases](https://github.com/np2023v2/rclone-ui/releases) page.

### With Docker

```bash
# Build the image
docker build -t rclone-ui:latest .

# Run with interactive TUI (mount rclone config)
docker run --rm -it -v ~/.config/rclone:/root/.config/rclone rclone-ui:latest

# Or use Docker Compose
docker compose up
```

### With Nix

```bash
# Enter development environment (includes rclone)
nix develop

# Or run directly
nix run
```

### With GitHub Codespaces

Click the "Code" button on GitHub and select "Create codespace on main" - everything is pre-configured!

## Usage

### Terminal User Interface (TUI)

Start the interactive mode (default):

```bash
./rclone-ui
# or
./rclone-ui tui
```

#### TUI Commands:
- `h` - Show help
- `Enter` - Open selected remote or navigate into directory
- `b` - Go back (to parent directory or remotes list)
- `r` - Refresh current view
- `↑↓` - Navigate items
- `q` - Quit application

### Command Line Interface

```bash
# Show help
./rclone-ui --help

# List all configured remotes
./rclone-ui remotes

# List files in a remote
./rclone-ui list myremote:

# List files in a specific path
./rclone-ui list myremote:Documents

# Check rclone version
./rclone-ui version
```

## Project Structure

```
rclone-ui/
├── .github/workflows/    # CI/CD workflows
├── src/
│   ├── rclone/           # Rclone CLI interaction
│   ├── tui/              # Terminal UI
│   ├── lib.rs            # Library root
│   └── main.rs           # CLI application
├── tests/                # Integration tests
├── docs/                 # Documentation
└── examples/             # Usage examples
```

## Development

> **📚 Full Setup Guide**: See [SETUP.md](SETUP.md) for comprehensive development environment setup instructions.

### Prerequisites

Choose your preferred development method:

- **Local**: Rust 1.70 or later, rclone
- **Docker**: Docker 20.10+ and Docker Compose
- **Nix**: Nix package manager with flakes enabled (includes rclone)
- **Codespaces**: Just a GitHub account!

### Building

```bash
# Local
cargo build

# Docker
docker compose up --build

# Nix
nix build
```

### Running Tests

```bash
cargo test
```

### Running Clippy (Linter)

```bash
cargo clippy -- -D warnings
```

### Formatting Code

```bash
cargo fmt
```

### Development Environments

The project provides multiple development environment options:

- **Docker Compose**: `docker compose up dev` - Containerized development with live code mounting
- **Nix Flakes**: `nix develop` - Reproducible environment with all dependencies including rclone
- **Devcontainer**: Open in VS Code or GitHub Codespaces - Fully configured IDE
- **Traditional**: Local Rust installation with cargo

## Rclone Configuration

The application uses your system's rclone configuration. Configure remotes using:

```bash
rclone config
```

Supported remote types include:
- Amazon S3
- Google Drive
- Dropbox
- Microsoft OneDrive
- Backblaze B2
- And many more...

See [rclone.org](https://rclone.org/) for full documentation.

## CI/CD

The project includes comprehensive GitHub Actions workflows:

- **CI** (`ci.yml`): Build, test, lint, and format checks on multiple platforms (Linux, macOS, Windows)
- **Security** (`security.yml`): Weekly security audits with `cargo audit`
- **Release** (`release.yml`): Automated binary releases for Linux, macOS, and Windows on version tags
- **Docker** (`docker.yml`): Docker image build testing and docker-compose validation

All workflows run automatically on push and pull requests to ensure code quality and security.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
