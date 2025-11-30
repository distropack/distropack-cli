use anyhow::Result;
use crate::api::ApiClient;
use crate::config::Config;

pub async fn upload_file(package_id: &str, ref_id: &str, file_path: &str) -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::new(&config)?;
    
    println!("Uploading file {} to package {} (ref: {})...", file_path, package_id, ref_id);
    client.upload_file(package_id, ref_id, file_path).await?;
    println!("File uploaded successfully!");
    
    Ok(())
}


