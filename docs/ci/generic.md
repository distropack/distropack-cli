# Generic CI Platform Integration

Guide for integrating DistroPack CLI with any CI/CD platform.

## Overview

The DistroPack CLI works with any CI platform that supports:
- Environment variables
- Shell script execution
- Network access

## Generic Shell Script

Use the provided `generic-ci.sh` script:

```bash
#!/bin/bash
# Set required variables
export DISTROPACK_PACKAGE_ID=123
export DISTROPACK_API_TOKEN="your-token-here"
export DISTROPACK_API_URL="https://distropack.dev"  # Optional
export VERSION="1.0.0"  # Or extract from CI variables

# Run the generic script
./distropack-cli/examples/ci/generic-ci.sh
```

## Platform-Specific Adaptations

### Travis CI

```yaml
language: rust
env:
  - DISTROPACK_PACKAGE_ID=123
script:
  - cargo install --path distropack-cli --locked
  - |
    if [ -n "$TRAVIS_TAG" ]; then
      VERSION=${TRAVIS_TAG#v}
    else
      VERSION=${TRAVIS_COMMIT:0:7}
    fi
    distropack-cli build --package-id $DISTROPACK_PACKAGE_ID --version $VERSION
```

### Azure Pipelines

```yaml
variables:
  DISTROPACK_PACKAGE_ID: '123'

steps:
  - task: UsePythonVersion@0
    inputs:
      versionSpec: '3.x'
  
  - script: |
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
      source $HOME/.cargo/env
      cargo install --path distropack-cli --locked
    displayName: 'Install DistroPack CLI'
  
  - script: |
      distropack-cli build --package-id $(DISTROPACK_PACKAGE_ID) --version $(Build.BuildNumber)
    env:
      DISTROPACK_API_TOKEN: $(DISTROPACK_API_TOKEN)
    displayName: 'Trigger Build'
```

### Jenkins

See the [Jenkins example](jenkins/Jenkinsfile) for a complete pipeline.

### Drone CI

```yaml
kind: pipeline
type: docker
name: distropack

steps:
  - name: install-cli
    image: rust:1.75
    commands:
      - cargo install --path distropack-cli --locked

  - name: build
    image: rust:1.75
    environment:
      DISTROPACK_API_TOKEN:
        from_secret: distropack_api_token
      DISTROPACK_PACKAGE_ID: 123
    commands:
      - distropack-cli build --package-id $DISTROPACK_PACKAGE_ID --version ${DRONE_TAG#v}
    when:
      event:
        - tag
```

## Common Patterns

### Version Extraction

Most platforms provide version information. Common patterns:

```bash
# From tag
VERSION=${TAG#v}  # Remove 'v' prefix

# From commit SHA
VERSION=${COMMIT:0:7}  # First 7 characters

# From branch name
VERSION=${BRANCH#release-}  # Remove prefix

# Manual
VERSION="1.0.0"
```

### Conditional Execution

Only build on specific conditions:

```bash
if [ -n "$TAG" ] && [[ "$TAG" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    VERSION=${TAG#v}
    distropack-cli build --package-id $PACKAGE_ID --version $VERSION
fi
```

## Best Practices

1. **Use secrets management** - Never hardcode tokens
2. **Extract version consistently** - Use the same pattern across environments
3. **Handle errors** - Check exit codes and fail builds appropriately
4. **Log output** - Helpful for debugging

## Troubleshooting

### Platform-specific issues

- **Windows**: Use PowerShell or WSL for shell scripts
- **Limited network**: Ensure CI platform allows outbound HTTPS
- **Permission errors**: Check file permissions and PATH

### Common errors

- **Token not found**: Verify environment variable names
- **CLI not found**: Ensure PATH includes CLI location
- **Build fails**: Check all prerequisites are met

