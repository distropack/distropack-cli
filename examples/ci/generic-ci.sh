#!/bin/bash
# Generic CI script for DistroPack integration
# Works with any CI platform that supports environment variables

set -e

# Required environment variables
# DISTROPACK_PACKAGE_ID - Your package ID from DistroPack dashboard
# DISTROPACK_API_TOKEN - Your API token (set as secret in CI)
# DISTROPACK_API_URL - Optional, defaults to https://distropack.dev
# VERSION - Package version (or set from CI variables)

# Install DistroPack CLI
install_distropack_cli() {
    echo "Installing DistroPack CLI..."
    
    # Option 1: Download pre-built binary (when available)
    # PLATFORM=$(uname -m)-unknown-linux-gnu
    # curl -L "https://github.com/distropack/distropack-cli/releases/latest/download/distropack-cli-${PLATFORM}" -o distropack-cli
    # chmod +x distropack-cli
    # export PATH=$PWD:$PATH
    
    # Option 2: Build from source
    if ! command -v cargo &> /dev/null; then
        echo "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source $HOME/.cargo/env
    fi
    
    cargo install --path distropack-cli --locked || {
        echo "Failed to install CLI. Make sure distropack-cli directory is available."
        exit 1
    }
}

# Determine version from CI environment
determine_version() {
    if [ -n "$VERSION" ]; then
        echo "$VERSION"
    elif [ -n "$CI_COMMIT_TAG" ]; then
        # GitLab CI
        echo "${CI_COMMIT_TAG#v}"
    elif [ -n "$GITHUB_REF" ] && [[ "$GITHUB_REF" == refs/tags/* ]]; then
        # GitHub Actions
        echo "${GITHUB_REF#refs/tags/v}"
    elif [ -n "$CIRCLE_TAG" ]; then
        # CircleCI
        echo "${CIRCLE_TAG#v}"
    elif [ -n "$TRAVIS_TAG" ]; then
        # Travis CI
        echo "${TRAVIS_TAG#v}"
    else
        echo "Version not specified. Set VERSION environment variable."
        exit 1
    fi
}

# Main execution
main() {
    # Validate required variables
    if [ -z "$DISTROPACK_PACKAGE_ID" ]; then
        echo "Error: DISTROPACK_PACKAGE_ID is not set"
        exit 1
    fi
    
    if [ -z "$DISTROPACK_API_TOKEN" ]; then
        echo "Error: DISTROPACK_API_TOKEN is not set"
        exit 1
    fi
    
    # Install CLI
    install_distropack_cli
    
    # Determine version
    VERSION=$(determine_version)
    echo "Building version: $VERSION"
    
    # Upload files (adjust file paths and ref-ids as needed)
    echo "Uploading files..."
    distropack-cli upload --package-id "$DISTROPACK_PACKAGE_ID" --ref-id source-tarball --file "dist/myapp-${VERSION}.tar.gz"
    distropack-cli upload --package-id "$DISTROPACK_PACKAGE_ID" --ref-id changelog --file "CHANGELOG.md"
    
    # Trigger builds
    echo "Triggering builds..."
    distropack-cli build --package-id "$DISTROPACK_PACKAGE_ID" --version "$VERSION"
    
    echo "Build triggered successfully!"
}

main "$@"

