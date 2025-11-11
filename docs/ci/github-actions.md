# GitHub Actions Integration

Complete guide for integrating DistroPack CLI with GitHub Actions.

## Quick Start

### Using the GitHub Action (Recommended)

The easiest way to use DistroPack in GitHub Actions is with our official action. It handles CLI installation automatically and provides a simple interface.

1. **Add secrets to your repository:**
   - Go to Settings → Secrets and variables → Actions
   - Add `DISTROPACK_API_TOKEN` with your API token
   - Optionally add `DISTROPACK_PACKAGE_ID` if you want to hardcode it

2. **Create workflow file** `.github/workflows/distropack.yml`:

```yaml
name: DistroPack Build

on:
  push:
    tags:
      - 'v*'
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
      
      - name: Determine version
        id: version
        run: |
          if [ "${{ github.event_name }}" == "workflow_dispatch" ]; then
            echo "version=${{ github.event.inputs.version }}" >> $GITHUB_OUTPUT
          else
            VERSION=${GITHUB_REF#refs/tags/v}
            echo "version=$VERSION" >> $GITHUB_OUTPUT
          fi
      
      - name: Build source tarball
        run: |
          mkdir -p dist
          tar czf dist/myapp-${{ steps.version.outputs.version }}.tar.gz src/
      
      - name: DistroPack
        uses: distropack/distropack-action@v1.0
        with:
          api-token: ${{ secrets.DISTROPACK_API_TOKEN }}
          package-id: ${{ secrets.DISTROPACK_PACKAGE_ID }}
          version: ${{ steps.version.outputs.version }}
          files: |
            {
              "source-tarball": "dist/myapp-${{ steps.version.outputs.version }}.tar.gz",
              "changelog": "CHANGELOG.md"
            }
```

See the [GitHub Action documentation](../../.github/actions/distropack-action/README.md) for complete usage examples.

### Manual Installation (Alternative)

If you prefer to install the CLI manually or need more control over the installation process:

```yaml
name: DistroPack Build

on:
  push:
    tags:
      - 'v*'
  workflow_dispatch:
    inputs:
      version:
        description: 'Package version'
        required: true

env:
  DISTROPACK_PACKAGE_ID: ${{ secrets.DISTROPACK_PACKAGE_ID }}
  DISTROPACK_API_TOKEN: ${{ secrets.DISTROPACK_API_TOKEN }}

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install DistroPack CLI
        run: |
          curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
          source $HOME/.cargo/env
          cargo install --path distropack-cli --locked
      
      - name: Determine version
        id: version
        run: |
          if [ "${{ github.event_name }}" == "workflow_dispatch" ]; then
            echo "version=${{ github.event.inputs.version }}" >> $GITHUB_OUTPUT
          else
            VERSION=${GITHUB_REF#refs/tags/v}
            echo "version=$VERSION" >> $GITHUB_OUTPUT
          fi
      
      - name: Upload files
        run: |
          distropack-cli upload --package-id $DISTROPACK_PACKAGE_ID --ref-id source-tarball --file dist/myapp-${{ steps.version.outputs.version }}.tar.gz
      
      - name: Trigger build
        run: |
          distropack-cli build --package-id $DISTROPACK_PACKAGE_ID --version ${{ steps.version.outputs.version }}
```

## Workflow Triggers

### On Tag Push

Automatically build when you push a version tag:

```yaml
on:
  push:
    tags:
      - 'v*'  # Matches v1.0.0, v2.1.3, etc.
```

### Manual Trigger

Allow manual builds with version input:

```yaml
on:
  workflow_dispatch:
    inputs:
      version:
        description: 'Package version'
        required: true
```

### On Release

Build when a release is published:

```yaml
on:
  release:
    types: [published]
```

## Advanced Examples

### Multi-Target Builds with Action

Build for specific distributions using the action:

```yaml
strategy:
  matrix:
    target: [deb, rpm, pacman]
steps:
  - name: DistroPack
    uses: distropack/distropack-action@v1.0
    with:
      api-token: ${{ secrets.DISTROPACK_API_TOKEN }}
      package-id: ${{ secrets.DISTROPACK_PACKAGE_ID }}
      version: ${{ steps.version.outputs.version }}
      target: ${{ matrix.target }}
      files: |
        {
          "source-tarball": "dist/myapp-${{ steps.version.outputs.version }}.tar.gz"
        }
```

### Multi-Target Builds (Manual Installation)

Build for specific distributions with manual installation:

```yaml
- name: Build Debian package
  run: |
    distropack-cli build --package-id $DISTROPACK_PACKAGE_ID --version ${{ steps.version.outputs.version }} --target deb

- name: Build RPM package
  run: |
    distropack-cli build --package-id $DISTROPACK_PACKAGE_ID --version ${{ steps.version.outputs.version }} --target rpm
```

### Conditional Builds

Only build on main branch:

```yaml
jobs:
  build:
    if: github.ref == 'refs/heads/main'
    # ... rest of workflow
```

### Matrix Builds with Action

Build multiple packages using the action:

```yaml
strategy:
  matrix:
    package: [123, 456, 789]
steps:
  - name: DistroPack
    uses: distropack/distropack-action@v1.0
    with:
      api-token: ${{ secrets.DISTROPACK_API_TOKEN }}
      package-id: ${{ matrix.package }}
      version: ${{ steps.version.outputs.version }}
      files: |
        {
          "source-tarball": "dist/myapp-${{ steps.version.outputs.version }}.tar.gz"
        }
```

### Matrix Builds (Manual Installation)

Build multiple packages with manual installation:

```yaml
strategy:
  matrix:
    package: [123, 456, 789]
steps:
  - name: Build package ${{ matrix.package }}
    run: |
      distropack-cli build --package-id ${{ matrix.package }} --version $VERSION
```

## Best Practices

1. **Use Secrets**: Never hardcode tokens in workflow files
2. **Tag-based builds**: Only build on version tags to avoid unnecessary builds
3. **Error handling**: GitHub Actions will fail on non-zero exit codes
4. **Notifications**: Set up notifications for failed builds

## Troubleshooting

### "API token not set" error

- Verify `DISTROPACK_API_TOKEN` is set in repository secrets
- Check that the secret name matches exactly (case-sensitive)

### Workflow not triggering

- Verify tag format matches the pattern (e.g., `v1.0.0`)
- Check workflow file is in `.github/workflows/` directory
- Ensure workflow file has valid YAML syntax

### Build fails

- Check that all required files are uploaded before triggering build
- Verify package ID is correct
- Ensure version is newer than last successful build


