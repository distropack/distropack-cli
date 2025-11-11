# Installation Guide

## Prerequisites

- Rust 1.70+ (for building from source)
- Network access to download dependencies

## Installation Methods

### Method 1: Build from Source

1. Clone the repository:
   ```bash
   git clone https://github.com/distropack/distropack-cli.git
   cd distropack-cli
   ```

2. Install using Cargo:
   ```bash
   cargo install --path . --locked
   ```

3. Verify installation:
   ```bash
   distropack-cli --version
   ```

### Method 2: Install from Releases (Coming Soon)

Pre-built binaries will be available for download from the [Releases page](https://github.com/distropack/distropack-cli/releases).

**Linux:**
```bash
curl -L https://github.com/distropack/distropack-cli/releases/latest/download/distropack-cli-x86_64-unknown-linux-gnu -o distropack-cli
chmod +x distropack-cli
sudo mv distropack-cli /usr/local/bin/
```

**macOS:**
```bash
curl -L https://github.com/distropack/distropack-cli/releases/latest/download/distropack-cli-x86_64-apple-darwin -o distropack-cli
chmod +x distropack-cli
sudo mv distropack-cli /usr/local/bin/
```

**Windows:**
Download the `.exe` file and add it to your PATH.

### Method 3: Install via Cargo (When Published)

Once published to crates.io:
```bash
cargo install distropack-cli
```

## Post-Installation

After installation, configure your API token:

```bash
distropack-cli config set-token YOUR_TOKEN_HERE
```

Get your API token from the [DistroPack Dashboard](https://distropack.com/Dashboard/ApiTokens).

## Uninstallation

**If installed via Cargo:**
```bash
cargo uninstall distropack-cli
```

**If installed manually:**
Remove the binary from your PATH and delete the config directory:
- Linux/macOS: `~/.config/distropack/`
- Windows: `%APPDATA%\distropack\`

## Troubleshooting

### "command not found" error

Make sure the binary is in your PATH. You can check with:
```bash
which distropack-cli  # Linux/macOS
where distropack-cli  # Windows
```

### Permission denied

On Linux/macOS, you may need to use `sudo` when moving to `/usr/local/bin/`, or install to a user directory like `~/.local/bin/` instead.


