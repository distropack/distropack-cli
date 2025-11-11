# API Reference

Complete reference for DistroPack CLI commands and options.

## Commands

### `upload`

Upload a file to a package by reference ID.

**Usage:**
```bash
distropack-cli upload --package-id <id> --ref-id <accessName> --file <path>
```

**Options:**
- `--package-id <id>` - Package ID (required)
- `--ref-id <accessName>` - File reference ID/access name (required)
- `--file <path>` - Path to file to upload (required)

**Example:**
```bash
distropack-cli upload --package-id 123 --ref-id source-tarball --file dist/myapp-1.0.0.tar.gz
```

**Exit Codes:**
- `0` - Success
- `1` - Error (file not found, upload failed, authentication error)

### `build`

Trigger package build(s) for one or all enabled distributions.

**Usage:**
```bash
distropack-cli build --package-id <id> --version <version> [--target <target>]
```

**Options:**
- `--package-id <id>` - Package ID (required)
- `--version <version>` - Version string (required)
- `--target <target>` - Target distribution: `deb`, `rpm`, or `pacman` (optional)

**Examples:**
```bash
# Build for all enabled targets
distropack-cli build --package-id 123 --version 1.0.0

# Build for specific target
distropack-cli build --package-id 123 --version 1.0.0 --target deb
```

**Exit Codes:**
- `0` - Success
- `1` - Error (build request failed, validation error, authentication error)

### `config`

Manage CLI configuration.

#### `config set-token`

Set the API token.

**Usage:**
```bash
distropack-cli config set-token <token>
```

**Example:**
```bash
distropack-cli config set-token abc123xyz789
```

#### `config set-base-url`

Set the API base URL.

**Usage:**
```bash
distropack-cli config set-base-url <url>
```

**Example:**
```bash
distropack-cli config set-base-url https://distropack.dev
```

#### `config show`

Display current configuration.

**Usage:**
```bash
distropack-cli config show
```

**Output:**
```
Configuration:
  Base URL: https://distropack.dev
  API Token: abcd...wxyz
```

## Environment Variables

All commands respect these environment variables:

- `DISTROPACK_API_TOKEN` - API token (overrides config file)
- `DISTROPACK_API_URL` - Base API URL (overrides config file)

## Configuration File

Configuration is stored in:
- **Linux/macOS**: `~/.config/distropack/config.toml`
- **Windows**: `%APPDATA%\distropack\config.toml`

**Format:**
```toml
api_token = "your-token-here"
base_url = "https://distropack.dev"
```

## Exit Codes

- `0` - Success
- `1` - General error (see error message for details)

## Error Messages

The CLI provides clear error messages:

- **Authentication errors**: "API token not set" or "Unauthorized"
- **File errors**: "File not found: <path>"
- **Network errors**: "Failed to send request: <details>"
- **Validation errors**: "Upload failed with status <code>: <message>"

## JSON Output (Future)

Future versions may support `--json` flag for machine-readable output:

```bash
distropack-cli build --package-id 123 --version 1.0.0 --json
```

Output would be JSON format for easier parsing in scripts.

