use anyhow::Result;
use crate::api::ApiClient;
use crate::config::Config;

pub async fn build_package(package_id: i32, version: &str, target: Option<&str>) -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::new(&config)?;
    
    if let Some(target) = target {
        println!("Triggering build for package {} version {} (target: {})...", 
            package_id, version, target);
    } else {
        println!("Triggering build for package {} version {} (all targets)...", 
            package_id, version);
    }
    
    client.build_package(package_id, version, target).await?;
    
    Ok(())
}


