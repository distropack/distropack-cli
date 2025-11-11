# Authentication Guide

DistroPack CLI uses API tokens for authentication. This guide explains how to obtain, configure, and use API tokens securely.

## Obtaining an API Token

1. Log in to your [DistroPack Dashboard](https://distropack.com/Dashboard)
2. Navigate to **API Tokens** in the dashboard menu
3. Click **Create Token**
4. Give your token a descriptive name (e.g., "CI/CD Pipeline", "Local Development")
5. **Copy the token immediately** - it's only shown once!

## Configuring the Token

### Method 1: Using the Config Command

```bash
distropack-cli config set-token YOUR_TOKEN_HERE
```

The token is stored securely in your configuration file.

### Method 2: Environment Variable (Recommended for CI/CD)

```bash
export DISTROPACK_API_TOKEN="your-token-here"
```

Environment variables take precedence over the config file, making them ideal for CI/CD environments.

### Method 3: Config File (Advanced)

Edit the config file directly:
- **Linux/macOS**: `~/.config/distropack/config.toml`
- **Windows**: `%APPDATA%\distropack\config.toml`

```toml
api_token = "your-token-here"
base_url = "https://distropack.dev"
```

## Verifying Authentication

Check your configuration:
```bash
distropack-cli config show
```

This will display your base URL and a masked version of your token.

## Token Management

### Viewing Your Tokens

Visit the [API Tokens page](https://distropack.com/Dashboard/ApiTokens) in the dashboard to see:
- Token names
- Creation dates
- Last used timestamps

### Revoking Tokens

1. Go to the API Tokens page in the dashboard
2. Click **Delete** next to the token you want to revoke
3. Confirm the deletion

**Note:** Revoking a token immediately invalidates it. Any scripts or CI/CD pipelines using that token will fail.

### Rotating Tokens

For security best practices, rotate your tokens regularly:

1. Create a new token
2. Update your configuration/CI/CD with the new token
3. Verify everything works
4. Delete the old token

## Security Best Practices

1. **Never commit tokens to version control**
   - Use environment variables or secrets management
   - Add config files to `.gitignore`

2. **Use separate tokens for different purposes**
   - One token for CI/CD
   - One token for local development
   - One token for testing

3. **Rotate tokens regularly**
   - Set a reminder to rotate every 90 days
   - Rotate immediately if a token is compromised

4. **Use descriptive token names**
   - Makes it easier to identify and revoke specific tokens
   - Example: "GitHub Actions - Production"

5. **Monitor token usage**
   - Check "Last Used" timestamps regularly
   - Revoke unused tokens

6. **Limit token scope** (when available)
   - Use the most restrictive scope possible
   - Don't grant unnecessary permissions

## Troubleshooting

### "API token not set" error

- Verify the token is set: `distropack-cli config show`
- Check environment variables: `echo $DISTROPACK_API_TOKEN`
- Ensure the config file exists and is readable

### "Unauthorized" error

- Verify the token is correct (no extra spaces or characters)
- Check if the token was revoked in the dashboard
- Ensure your account has an active subscription

### Token appears in process list

If you're concerned about tokens appearing in process lists:
- Use environment variables instead of command-line arguments
- Consider using a secrets manager for CI/CD

