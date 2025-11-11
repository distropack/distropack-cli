# GitLab CI Integration

Complete guide for integrating DistroPack CLI with GitLab CI.

## Quick Start

1. **Add CI/CD variables:**
   - Go to Settings → CI/CD → Variables
   - Add `DISTROPACK_API_TOKEN` (masked, protected)
   - Optionally add `DISTROPACK_PACKAGE_ID`

2. **Create `.gitlab-ci.yml`:**

```yaml
variables:
  DISTROPACK_PACKAGE_ID: "${DISTROPACK_PACKAGE_ID}"

stages:
  - build

build-package:
  stage: build
  image: rust:1.75
  before_script:
    - cargo install --path distropack-cli --locked
  script:
    - |
      if [ -n "$CI_COMMIT_TAG" ]; then
        VERSION=${CI_COMMIT_TAG#v}
      else
        VERSION=${CI_COMMIT_REF_SLUG}
      fi
      
      distropack-cli upload --package-id $DISTROPACK_PACKAGE_ID --ref-id source-tarball --file dist/myapp-$VERSION.tar.gz
      distropack-cli build --package-id $DISTROPACK_PACKAGE_ID --version $VERSION
  only:
    - tags
    - main
```

## Configuration

### Using Custom Image

If you need a different base image:

```yaml
build-package:
  image: ubuntu:22.04
  before_script:
    - apt-get update && apt-get install -y curl
    - curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    - source $HOME/.cargo/env
    - cargo install --path distropack-cli --locked
```

### Protected Variables

Mark sensitive variables as protected:

1. Go to Settings → CI/CD → Variables
2. Edit the variable
3. Check "Protect variable"
4. Select protected branches/tags

## Advanced Examples

### Multi-Stage Pipeline

```yaml
stages:
  - prepare
  - upload
  - build

prepare:
  stage: prepare
  script:
    - echo "Preparing build artifacts..."

upload-files:
  stage: upload
  script:
    - distropack-cli upload --package-id $DISTROPACK_PACKAGE_ID --ref-id source-tarball --file dist/app.tar.gz

trigger-build:
  stage: build
  script:
    - distropack-cli build --package-id $DISTROPACK_PACKAGE_ID --version $VERSION
  needs:
    - upload-files
```

### Parallel Builds

```yaml
build-deb:
  script:
    - distropack-cli build --package-id $DISTROPACK_PACKAGE_ID --version $VERSION --target deb

build-rpm:
  script:
    - distropack-cli build --package-id $DISTROPACK_PACKAGE_ID --version $VERSION --target rpm
```

## Best Practices

1. **Use protected variables** for sensitive data
2. **Limit builds** to tags and main branch
3. **Use appropriate images** - Rust image for faster builds
4. **Cache dependencies** if building from source frequently

## Troubleshooting

### Variable not found

- Verify variable is set in CI/CD settings
- Check variable name matches exactly
- Ensure variable is not protected if building on unprotected branches

### Build fails

- Check job logs for specific error messages
- Verify all required files are uploaded
- Ensure version format is correct


