# CI/CD Integration Guide

This guide shows how to integrate DistroPack CLI into your CI/CD pipelines.

## Overview

The DistroPack CLI is designed to work seamlessly in CI/CD environments. It supports:
- Environment variable configuration
- Non-interactive operation
- Proper exit codes for pipeline control
- Clear error messages

## Required Setup

For any CI/CD platform, you'll need:

1. **API Token** - Create one in the [DistroPack Dashboard](https://distropack.com/Dashboard/ApiTokens)
2. **Package ID** - Found in your package's URL or dashboard
3. **File Reference IDs** - The "access names" you set when creating package files

## Using the GitHub Action

**Recommended for GitHub Actions users** - The easiest way to use DistroPack in GitHub Actions is with our official action. It handles CLI installation automatically and provides a simple interface for uploading files and triggering builds.

**Example:**
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

**Benefits:**
- ✅ No manual CLI installation required
- ✅ Automatic binary caching for faster runs
- ✅ Single step for uploads and builds
- ✅ Simple JSON-based file mapping

See the [GitHub Action documentation](../../.github/actions/distropack-action/README.md) for complete usage examples and the [GitHub Actions guide](ci/github-actions.md) for detailed integration instructions.

## Platform-Specific Guides

- [GitHub Actions](ci/github-actions.md) - Includes both action and manual installation methods
- [GitLab CI](ci/gitlab-ci.md)
- [CircleCI](ci/circleci.md)
- [Generic CI Platforms](ci/generic.md)

## Common Workflow

Most CI/CD integrations follow this pattern:

1. **Install CLI** - Download or build the DistroPack CLI (or use the GitHub Action which handles this automatically)
2. **Configure** - Set API token and base URL via environment variables
3. **Upload Files** - Upload required files by reference ID
4. **Trigger Build** - Start the package build process

**Note:** If using the GitHub Action, steps 1-4 are handled in a single action step.

## Environment Variables

Set these in your CI/CD platform's secrets/environment configuration:

- `DISTROPACK_API_TOKEN` - Your API token (required)
- `DISTROPACK_API_URL` - API base URL (optional, defaults to https://distropack.dev)
- `DISTROPACK_PACKAGE_ID` - Your package ID (can be hardcoded or from secrets)

## Example: Basic Workflow

### Using GitHub Action (Recommended)

```yaml
# GitHub Actions example using the official action
steps:
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

### Manual Installation (Other Platforms)

```yaml
# Generic YAML example
steps:
  - name: Install DistroPack CLI
    run: |
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
      source $HOME/.cargo/env
      cargo install --path distropack-cli --locked

  - name: Upload files
    env:
      DISTROPACK_API_TOKEN: ${{ secrets.DISTROPACK_API_TOKEN }}
    run: |
      distropack-cli upload --package-id $PACKAGE_ID --ref-id source-tarball --file dist/app.tar.gz

  - name: Trigger build
    env:
      DISTROPACK_API_TOKEN: ${{ secrets.DISTROPACK_API_TOKEN }}
    run: |
      distropack-cli build --package-id $PACKAGE_ID --version $VERSION
```

## Version Handling

Different CI platforms provide version information differently:

- **GitHub Actions**: Extract from `$GITHUB_REF` (tags) or use workflow inputs
- **GitLab CI**: Use `$CI_COMMIT_TAG` or `$CI_COMMIT_REF_SLUG`
- **CircleCI**: Use `$CIRCLE_TAG` or build from branch/SHA
- **Generic**: Set `VERSION` environment variable manually

## Error Handling

The CLI returns proper exit codes:
- `0` - Success
- `1` - Error (authentication, network, validation)

Your CI/CD platform should handle these appropriately. Most platforms will fail the build on non-zero exit codes.

## Best Practices

1. **Use Secrets Management**
   - Never hardcode tokens
   - Use your platform's secrets management
   - Rotate tokens regularly

2. **Conditional Builds**
   - Only trigger builds on tags or specific branches
   - Avoid building on every commit

3. **Error Notifications**
   - Set up notifications for failed builds
   - Monitor token usage

4. **Version Validation**
   - Validate version format before triggering builds
   - Use semantic versioning

5. **Parallel Operations**
   - Upload files in parallel when possible
   - Don't wait for builds to complete (they run asynchronously)

## Troubleshooting

### CLI Installation Fails

- Ensure Rust toolchain is available
- Check network connectivity
- Verify sufficient disk space

### Authentication Errors

- Verify token is set correctly
- Check token hasn't been revoked
- Ensure account has active subscription

### Upload Failures

- Verify file paths are correct
- Check file permissions
- Ensure reference IDs match your package configuration

### Build Failures

- Check that all required files are uploaded
- Verify version is newer than last successful build
- Ensure at least one build target is enabled

## Next Steps

- See platform-specific guides for detailed examples
- Check the [API Reference](api-reference.md) for advanced usage
- Review [Examples](examples.md) for common patterns

