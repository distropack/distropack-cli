# DistroPack GitHub Action

Official GitHub Action for DistroPack - upload files and trigger Linux package builds directly from your CI/CD pipelines.

## Features

- 🔐 Secure API token authentication
- 📤 Upload multiple files by reference ID
- 🏗️ Trigger package builds for multiple distributions
- ⚡ Automatic CLI installation (no manual setup required)
- 🚀 Fast execution with binary caching

## Usage

### Basic Example

```yaml
name: Build Packages

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Build source tarball
        run: |
          tar czf dist/myapp-${{ github.ref_name }}.tar.gz src/
      
      - name: Upload and build packages
        uses: distropack/distropack-cli/.github/actions/distropack-action@v1
        with:
          api-token: ${{ secrets.DISTROPACK_API_TOKEN }}
          package-id: 123
          version: ${{ github.ref_name }}
          files: |
            {
              "source-tarball": "dist/myapp-${{ github.ref_name }}.tar.gz",
              "changelog": "CHANGELOG.md"
            }
```

### With Specific Target

```yaml
- name: Build Debian package
  uses: distropack/distropack-cli/.github/actions/distropack-action@v1
  with:
    api-token: ${{ secrets.DISTROPACK_API_TOKEN }}
    package-id: 123
    version: 1.0.0
    target: deb
    files: |
      {
        "source-tarball": "dist/app.tar.gz"
      }
```

### Manual Workflow Dispatch

```yaml
name: Build Packages

on:
  workflow_dispatch:
    inputs:
      version:
        description: 'Package version'
        required: true

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Upload and build
        uses: distropack/distropack-cli/.github/actions/distropack-action@v1
        with:
          api-token: ${{ secrets.DISTROPACK_API_TOKEN }}
          package-id: ${{ secrets.DISTROPACK_PACKAGE_ID }}
          version: ${{ inputs.version }}
          files: |
            {
              "source-tarball": "dist/myapp-${{ inputs.version }}.tar.gz"
            }
```

## Inputs

| Input | Description | Required | Default |
|-------|-------------|----------|---------|
| `api-token` | API token (or use `DISTROPACK_API_TOKEN` env var) | No | - |
| `package-id` | Package ID to build | Yes | - |
| `version` | Version string for the build | Yes | - |
| `files` | JSON object mapping ref-ids to file paths | Yes | - |
| `target` | Target distribution (deb, rpm, pacman). Omit to build all enabled targets. | No | - |
| `api-url` | Base API URL | No | `https://distropack.dev` |
| `cli-version` | Specific CLI version to use | No | `latest` |

### Files Input Format

The `files` input must be a JSON object mapping reference IDs (access names) to file paths:

```json
{
  "source-tarball": "dist/app.tar.gz",
  "changelog": "CHANGELOG.md",
  "license": "LICENSE"
}
```

Each key is a reference ID (access name) configured in your DistroPack package, and each value is the path to the file relative to the repository root.

## Environment Variables

You can also use environment variables instead of inputs:

- `DISTROPACK_API_TOKEN` - API token (overrides `api-token` input)
- `DISTROPACK_API_URL` - Base API URL (overrides `api-url` input)

## Setup

### 1. Get Your API Token

1. Go to [DistroPack Dashboard](https://distropack.com/Dashboard/ApiTokens)
2. Create a new API token
3. Copy the token (you'll only see it once)

### 2. Add Secret to GitHub

1. Go to your repository Settings → Secrets and variables → Actions
2. Click "New repository secret"
3. Name: `DISTROPACK_API_TOKEN`
4. Value: Your API token
5. Click "Add secret"

### 3. Create Workflow File

Create `.github/workflows/distropack.yml` in your repository with the action usage.

## How It Works

1. **CLI Installation**: The action automatically installs the DistroPack CLI:
   - First checks for cached binary
   - Tries downloading from GitHub releases
   - Falls back to building from source if needed

2. **File Upload**: Parses the `files` JSON input and uploads each file by reference ID

3. **Build Trigger**: Triggers the package build with the specified version and target

## Examples

### Multiple Files

```yaml
files: |
  {
    "source-tarball": "dist/myapp-1.0.0.tar.gz",
    "changelog": "CHANGELOG.md",
    "license": "LICENSE",
    "readme": "README.md"
  }
```

### Matrix Builds

```yaml
strategy:
  matrix:
    target: [deb, rpm, pacman]
steps:
  - name: Build ${{ matrix.target }}
    uses: distropack/distropack-cli/.github/actions/distropack-action@v1
    with:
      api-token: ${{ secrets.DISTROPACK_API_TOKEN }}
      package-id: 123
      version: 1.0.0
      target: ${{ matrix.target }}
      files: |
        {
          "source-tarball": "dist/app.tar.gz"
        }
```

### Conditional Builds

```yaml
- name: Build on main branch
  if: github.ref == 'refs/heads/main'
  uses: distropack/distropack-cli/.github/actions/distropack-action@v1
  with:
    api-token: ${{ secrets.DISTROPACK_API_TOKEN }}
    package-id: 123
    version: ${{ github.sha }}
    files: |
      {
        "source-tarball": "dist/app.tar.gz"
      }
```

## Troubleshooting

### "API token not set" error

- Verify `DISTROPACK_API_TOKEN` is set in repository secrets
- Check that the secret name matches exactly (case-sensitive)
- Ensure you're passing the token via `api-token` input or `DISTROPACK_API_TOKEN` env var

### "File not found" error

- Verify file paths are correct relative to repository root
- Check that files are created in previous workflow steps
- Ensure file paths don't contain typos

### "Invalid JSON format" error

- Verify the `files` input is valid JSON
- Check that JSON keys and values are properly quoted
- Use a JSON validator if needed

### CLI installation fails

- The action will automatically fall back to building from source
- Ensure the runner has sufficient disk space
- Check network connectivity for downloading releases

### Build fails

- Verify all required files are uploaded before triggering build
- Check that package ID is correct
- Ensure version is newer than last successful build
- Verify at least one build target is enabled for the package

## Publishing to GitHub Marketplace

To publish this action to the GitHub Marketplace:

1. **Ensure proper branding** in `action.yml`:
   ```yaml
   branding:
     icon: 'package'
     color: 'blue'
   ```

2. **Create a release**:
   - Tag the release (e.g., `v1.0.0`)
   - Add release notes describing the action

3. **Publish to Marketplace**:
   - Go to the repository on GitHub
   - Click "Actions" tab
   - Click "Publish action" button
   - Or visit: https://github.com/marketplace/new
   - Select "GitHub Action"
   - Fill in marketplace listing details

4. **After publishing**:
   - Users can reference as: `distropack/action@v1` (if published to marketplace)
   - Or: `distropack/distropack-cli/.github/actions/distropack-action@v1` (direct reference)

## Versioning

The action uses semantic versioning:
- `@v1` - Latest v1.x.x release
- `@v1.0.0` - Specific version
- `@main` - Latest from main branch (not recommended for production)

## Related Documentation

- [DistroPack CLI Documentation](../../README.md)
- [CI Integration Guide](../../docs/ci-integration.md)
- [GitHub Actions Guide](../../docs/ci/github-actions.md)
- [API Reference](../../docs/api-reference.md)

## Support

- [Documentation](https://docs.distropack.com)
- [GitHub Issues](https://github.com/distropack/distropack-cli/issues)
- [Discord Community](https://discord.gg/distropack)

## License

MIT License - see LICENSE file for details

