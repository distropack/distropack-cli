use anyhow::Result;
use crate::config::Config;

pub async fn set_token(token: &str) -> Result<()> {
    let mut config = Config::load()?;
    config.api_token = Some(token.to_string());
    config.save()?;
    println!("API token saved successfully!");
    Ok(())
}

pub async fn set_base_url(url: &str) -> Result<()> {
    let mut config = Config::load()?;
    config.base_url = Some(url.to_string());
    config.save()?;
    println!("Base URL set to: {}", url);
    Ok(())
}

pub async fn show_config() -> Result<()> {
    let config = Config::load()?;
    
    println!("Configuration:");
    println!("  Base URL: {}", config.base_url());
    
    match config.api_token() {
        Ok(token) => {
            let masked = format!("{}...{}", &token[..4], &token[token.len()-4..]);
            println!("  API Token: {}", masked);
        }
        Err(_) => {
            println!("  API Token: Not set");
        }
    }
    
    Ok(())
}


