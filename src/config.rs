use anyhow::{Context, Result};
use dirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_token: Option<String>,
    pub base_url: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            return Ok(Config {
                api_token: None,
                base_url: None,
            });
        }

        let content = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;

        let config: Config =
            toml::from_str(&content).with_context(|| "Failed to parse config file")?;

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        let config_dir = config_path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Invalid config path"))?;

        fs::create_dir_all(config_dir).with_context(|| {
            format!(
                "Failed to create config directory: {}",
                config_dir.display()
            )
        })?;

        let content = toml::to_string_pretty(self).context("Failed to serialize config")?;

        fs::write(&config_path, content)
            .with_context(|| format!("Failed to write config file: {}", config_path.display()))?;

        Ok(())
    }

    pub fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
        Ok(config_dir.join("distropack").join("config.toml"))
    }

    pub fn api_token(&self) -> Result<String> {
        // Check environment variable first
        if let Ok(token) = std::env::var("DISTROPACK_API_TOKEN") {
            return Ok(token);
        }

        // Fall back to config file
        self.api_token.clone()
            .ok_or_else(|| anyhow::anyhow!("API token not set. Use 'distropack-cli config set-token <token>' or set DISTROPACK_API_TOKEN environment variable"))
    }

    pub fn base_url(&self) -> String {
        // Check environment variable first
        if let Ok(url) = std::env::var("DISTROPACK_API_URL") {
            return url;
        }

        // Fall back to config file or default
        self.base_url
            .clone()
            .unwrap_or_else(|| "https://distropack.dev".to_string())
    }
}
