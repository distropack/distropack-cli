# DistroPack CLI

Official command-line tool for DistroPack - automate Linux package builds from CI/CD pipelines.

## Features

- 🔐 Secure API token authentication
- 📤 Upload files to packages by reference ID
- 🏗️ Trigger package builds for multiple distributions
- ⚙️ Simple configuration management
- 🚀 CI/CD ready with environment variable support

## Using GitHub Action (Recommended)

The easiest way to use DistroPack in GitHub Actions is with our official action:

```yaml
- name: DistroPack
  uses: distropack/distropack-action@v1.0
  with:
    api-token: ${{ secrets.DISTROPACK_API_TOKEN }}
    package-id: 123
    version: 1.0.0
    files: |
      {
        "source-tarball": "dist/app.tar.gz",
        "changelog": "CHANGELOG.md"
      }
```

See the [GitHub Action documentation](.github/actions/distropack-action/README.md) for complete usage examples.

## Installation

### From Source

```bash
git clone https://github.com/distropack/distropack-cli.git
cd distropack-cli
cargo install --path . --locked
```

### From Releases (Coming Soon)

Pre-built binaries will be available for:
- Linux (x86_64, ARM64)
- macOS (x86_64, ARM64)
- Windows (x86_64)

## Quick Start

1. **Get your API token** from the [DistroPack Dashboard](https://distropack.com/Dashboard/ApiTokens)

2. **Configure the CLI**:
   ```bash
   distropack-cli config set-token YOUR_TOKEN_HERE
   distropack-cli config set-base-url https://distropack.dev
   ```

3. **Upload files**:
   ```bash
   distropack-cli upload --package-id 123 --ref-id source-tarball --file dist/myapp-1.0.0.tar.gz
   ```

4. **Trigger builds**:
   ```bash
   distropack-cli build --package-id 123 --version 1.0.0
   ```

## Commands

### Upload

Upload a file to a package by its reference ID (access name).

```bash
distropack-cli upload --package-id <id> --ref-id <accessName> --file <path>
```

**Example:**
```bash
distropack-cli upload --package-id 123 --ref-id source-tarball --file dist/myapp-1.0.0.tar.gz
```

### Build

Trigger package build(s) for one or all enabled distributions.

```bash
distropack-cli build --package-id <id> --version <version> [--target <deb|rpm|pacman>]
```

**Examples:**
```bash
# Build for all enabled targets
distropack-cli build --package-id 123 --version 1.0.0

# Build for specific target
distropack-cli build --package-id 123 --version 1.0.0 --target deb
distropack-cli build --package-id 123 --version 1.0.0 --target rpm
distropack-cli build --package-id 123 --version 1.0.0 --target pacman
```

### Config

Manage CLI configuration.

```bash
# Set API token
distropack-cli config set-token <token>

# Set base URL
distropack-cli config set-base-url <url>

# Show current configuration
distropack-cli config show
```

## Environment Variables

The CLI supports environment variables for CI/CD integration:

- `DISTROPACK_API_TOKEN` - API token (overrides config file)
- `DISTROPACK_API_URL` - Base API URL (overrides config file)

**Example:**
```bash
export DISTROPACK_API_TOKEN="your-token-here"
export DISTROPACK_API_URL="https://distropack.dev"
distropack-cli build --package-id 123 --version 1.0.0
```

## Configuration File

Configuration is stored in a platform-specific location:

- **Linux/macOS**: `~/.config/distropack/config.toml`
- **Windows**: `%APPDATA%\distropack\config.toml`

The configuration file uses TOML format:

```toml
api_token = "your-token-here"
base_url = "https://distropack.dev"
```

## CI/CD Integration

### GitHub Actions

Use the [DistroPack GitHub Action](.github/actions/distropack-action/README.md) for the simplest integration - no manual installation required!

### Other Platforms

See the [CI Integration Guide](docs/ci-integration.md) for examples with:
- GitLab CI
- CircleCI
- Jenkins
- Generic CI platforms

## Documentation

- [Installation Guide](docs/installation.md)
- [Authentication Guide](docs/authentication.md)
- [CI Integration Guide](docs/ci-integration.md)
- [API Reference](docs/api-reference.md)
- [Examples](docs/examples.md)

## Security Best Practices

1. **Never commit tokens** to version control
2. **Use environment variables** in CI/CD pipelines
3. **Rotate tokens regularly** from the dashboard
4. **Delete unused tokens** to minimize attack surface
5. **Use least privilege** - create tokens with specific scopes when available

## Troubleshooting

### "API token not set" error

Make sure you've either:
- Set the token via `distropack-cli config set-token <token>`
- Set the `DISTROPACK_API_TOKEN` environment variable

### "Upload failed" error

- Verify the file exists and is readable
- Check that the package ID and reference ID are correct
- Ensure you have an active subscription

### "Build request failed" error

- Verify all required files have been uploaded
- Check that the version is newer than the last successful build
- Ensure at least one build target is enabled

## License

MIT License - see LICENSE file for details

## Support

- [Documentation](https://docs.distropack.com)
- [GitHub Issues](https://github.com/distropack/distropack-cli/issues)
- [Discord Community](https://discord.gg/distropack)

