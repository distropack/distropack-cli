# CircleCI Integration

Complete guide for integrating DistroPack CLI with CircleCI.

## Quick Start

1. **Add environment variables:**
   - Go to Project Settings → Environment Variables
   - Add `DISTROPACK_API_TOKEN`
   - Add `DISTROPACK_PACKAGE_ID` (optional)

2. **Create `.circleci/config.yml`:**

```yaml
version: 2.1

jobs:
  build-and-publish:
    docker:
      - image: cimg/rust:1.75
    steps:
      - checkout
      
      - run:
          name: Install DistroPack CLI
          command: cargo install --path distropack-cli --locked

      - run:
          name: Determine version
          command: |
            if [ -n "$CIRCLE_TAG" ]; then
              VERSION=${CIRCLE_TAG#v}
            else
              VERSION=${CIRCLE_BRANCH}-${CIRCLE_SHA1:0:7}
            fi
            echo "export VERSION=$VERSION" >> $BASH_ENV

      - run:
          name: Upload files
          command: |
            distropack-cli upload --package-id $DISTROPACK_PACKAGE_ID --ref-id source-tarball --file dist/myapp-$VERSION.tar.gz

      - run:
          name: Trigger build
          command: |
            distropack-cli build --package-id $DISTROPACK_PACKAGE_ID --version $VERSION

workflows:
  version: 2
  build:
    jobs:
      - build-and-publish:
          filters:
            branches:
              only: main
            tags:
              only: /^v.*/
```

## Advanced Configuration

### Using Custom Docker Image

```yaml
jobs:
  build:
    docker:
      - image: ubuntu:22.04
    steps:
      - run:
          name: Install Rust
          command: |
            apt-get update
            apt-get install -y curl
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
            source $HOME/.cargo/env
```

### Parallel Jobs

```yaml
workflows:
  version: 2
  build:
    jobs:
      - build-deb:
          filters:
            tags:
              only: /^v.*/
      - build-rpm:
          filters:
            tags:
              only: /^v.*/
```

## Best Practices

1. **Use filters** to limit when builds run
2. **Cache Rust dependencies** for faster builds
3. **Use appropriate images** - cimg/rust for Rust projects

## Troubleshooting

### Environment variables not available

- Verify variables are set in project settings
- Check variable names match exactly
- Ensure variables are not restricted to specific contexts


