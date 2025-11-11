#!/bin/bash
set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# CLI binary name
CLI_BINARY="distropack-cli"
CLI_CACHE_DIR="$HOME/.cache/distropack-cli"
CLI_BINARY_PATH="$CLI_CACHE_DIR/$CLI_BINARY"

# GitHub repository (adjust if needed)
REPO_OWNER="distropack"
REPO_NAME="distropack-cli"

# Function to print colored messages
info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

# Function to check if binary exists and is executable
check_binary() {
    if [ -f "$CLI_BINARY_PATH" ] && [ -x "$CLI_BINARY_PATH" ]; then
        return 0
    fi
    return 1
}

# Function to download binary from GitHub releases
download_binary() {
    local version="${CLI_VERSION:-latest}"
    local arch=""
    local os=""
    
    # Detect OS and architecture
    case "$(uname -m)" in
        x86_64|amd64)
            arch="x86_64"
            ;;
        aarch64|arm64)
            arch="aarch64"
            ;;
        *)
            warn "Unsupported architecture: $(uname -m). Will build from source."
            return 1
            ;;
    esac
    
    case "$(uname -s)" in
        Linux)
            os="linux"
            ;;
        Darwin)
            os="macos"
            ;;
        *)
            warn "Unsupported OS: $(uname -s). Will build from source."
            return 1
            ;;
    esac
    
    # Determine download URL
    local url=""
    if [ "$version" = "latest" ]; then
        url="https://api.github.com/repos/$REPO_OWNER/$REPO_NAME/releases/latest"
    else
        url="https://api.github.com/repos/$REPO_OWNER/$REPO_NAME/releases/tags/$version"
    fi
    
    # Get release info
    info "Fetching release information..."
    local release_info
    release_info=$(curl -sL "$url" || echo "")
    
    if [ -z "$release_info" ]; then
        warn "Could not fetch release information. Will build from source."
        return 1
    fi
    
    # Extract download URL for the binary
    local download_url
    download_url=$(echo "$release_info" | grep -o "\"browser_download_url\": \"[^\"]*${os}-${arch}[^\"]*\"" | head -1 | cut -d'"' -f4 || echo "")
    
    if [ -z "$download_url" ]; then
        warn "No pre-built binary found for ${os}-${arch}. Will build from source."
        return 1
    fi
    
    # Create cache directory
    mkdir -p "$CLI_CACHE_DIR"
    
    # Download binary
    info "Downloading DistroPack CLI from GitHub releases..."
    if curl -sL "$download_url" -o "$CLI_BINARY_PATH"; then
        chmod +x "$CLI_BINARY_PATH"
        info "Successfully downloaded DistroPack CLI"
        return 0
    else
        warn "Download failed. Will build from source."
        rm -f "$CLI_BINARY_PATH"
        return 1
    fi
}

# Function to build CLI from source
build_from_source() {
    info "Building DistroPack CLI from source..."
    
    # Check if Rust is installed
    if ! command -v cargo &> /dev/null; then
        error "Cargo not found. Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    fi
    
    # Clone repository if not already present
    local repo_dir="$CLI_CACHE_DIR/repo"
    if [ ! -d "$repo_dir" ]; then
        info "Cloning DistroPack CLI repository..."
        git clone --depth 1 "https://github.com/$REPO_OWNER/$REPO_NAME.git" "$repo_dir"
    fi
    
    # Build CLI
    info "Building CLI (this may take a few minutes)..."
    cd "$repo_dir"
    cargo build --release --locked
    
    # Copy binary to cache location
    mkdir -p "$CLI_CACHE_DIR"
    cp "target/release/$CLI_BINARY" "$CLI_BINARY_PATH"
    chmod +x "$CLI_BINARY_PATH"
    
    info "Successfully built DistroPack CLI"
}

# Function to install CLI
install_cli() {
    # Check if binary already exists (cached)
    if check_binary; then
        info "Using cached DistroPack CLI"
        return 0
    fi
    
    # Try downloading from releases
    if download_binary; then
        return 0
    fi
    
    # Fall back to building from source
    build_from_source
}

# Function to parse JSON files input and upload files
upload_files() {
    local files_json="$FILES"
    
    if [ -z "$files_json" ]; then
        error "FILES input is required"
        exit 1
    fi
    
    # Validate JSON format
    if ! echo "$files_json" | jq empty 2>/dev/null; then
        error "Invalid JSON format in files input: $files_json"
        exit 1
    fi
    
    # Extract file mappings from JSON
    local ref_ids
    ref_ids=$(echo "$files_json" | jq -r 'keys[]' 2>/dev/null)
    
    if [ -z "$ref_ids" ]; then
        error "No file mappings found in files input"
        exit 1
    fi
    
    # Upload each file
    while IFS= read -r ref_id; do
        local file_path
        file_path=$(echo "$files_json" | jq -r ".[\"$ref_id\"]" 2>/dev/null)
        
        if [ -z "$file_path" ] || [ "$file_path" = "null" ]; then
            error "Invalid file path for ref-id: $ref_id"
            exit 1
        fi
        
        # Check if file exists
        if [ ! -f "$file_path" ]; then
            error "File not found: $file_path (ref-id: $ref_id)"
            exit 1
        fi
        
        info "Uploading file: $file_path (ref-id: $ref_id)"
        "$CLI_BINARY_PATH" upload \
            --package-id "$PACKAGE_ID" \
            --ref-id "$ref_id" \
            --file "$file_path"
        
        if [ $? -ne 0 ]; then
            error "Failed to upload file: $file_path"
            exit 1
        fi
        
        info "Successfully uploaded: $file_path"
    done <<< "$ref_ids"
}

# Function to trigger build
trigger_build() {
    info "Triggering build for package $PACKAGE_ID version $VERSION"
    
    local build_cmd=("$CLI_BINARY_PATH" build \
        --package-id "$PACKAGE_ID" \
        --version "$VERSION")
    
    if [ -n "${TARGET:-}" ]; then
        build_cmd+=(--target "$TARGET")
    fi
    
    "${build_cmd[@]}"
    
    if [ $? -ne 0 ]; then
        error "Failed to trigger build"
        exit 1
    fi
    
    info "Build triggered successfully"
}

# Function to execute main workflow
execute_workflow() {
    # Validate required environment variables
    if [ -z "${PACKAGE_ID:-}" ]; then
        error "PACKAGE_ID is required"
        exit 1
    fi
    
    if [ -z "${VERSION:-}" ]; then
        error "VERSION is required"
        exit 1
    fi
    
    if [ -z "${FILES:-}" ]; then
        error "FILES is required"
        exit 1
    fi
    
    # Set API URL if provided
    if [ -n "${DISTROPACK_API_URL:-}" ]; then
        export DISTROPACK_API_URL
    fi
    
    # Set API token if provided
    if [ -n "${DISTROPACK_API_TOKEN:-}" ]; then
        export DISTROPACK_API_TOKEN
    else
        error "API token not provided. Set api-token input or DISTROPACK_API_TOKEN env var."
        exit 1
    fi
    
    # Upload files
    upload_files
    
    # Trigger build
    trigger_build
    
    info "DistroPack action completed successfully"
}

# Main entry point
case "${1:-}" in
    install)
        install_cli
        ;;
    execute)
        execute_workflow
        ;;
    *)
        error "Unknown command: ${1:-}. Use 'install' or 'execute'."
        exit 1
        ;;
esac

