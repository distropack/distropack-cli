use anyhow::{Context, Result};
use reqwest::multipart;
use serde::Deserialize;
use std::path::Path;

use crate::config::Config;

pub struct ApiClient {
    client: reqwest::Client,
    base_url: String,
    token: String,
}

impl ApiClient {
    pub fn new(config: &Config) -> Result<Self> {
        let token = config.api_token()?;
        let base_url = config.base_url();

        let client = reqwest::Client::builder()
            .build()
            .context("Failed to create HTTP client")?;

        Ok(ApiClient {
            client,
            base_url,
            token,
        })
    }

    pub async fn upload_file(&self, package_id: i32, ref_id: &str, file_path: &str) -> Result<()> {
        let path = Path::new(file_path);
        if !path.exists() {
            anyhow::bail!("File not found: {}", file_path);
        }

        let file_bytes = tokio::fs::read(path)
            .await
            .with_context(|| format!("Failed to read file: {}", file_path))?;

        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| anyhow::anyhow!("Invalid file name"))?
            .to_string();

        let file_part = multipart::Part::bytes(file_bytes)
            .file_name(file_name)
            .mime_str("application/octet-stream")
            .context("Failed to create file part")?;

        let form = multipart::Form::new().part("file", file_part);

        let url = format!(
            "{}/api/upload_file?packageId={}&accessName={}",
            self.base_url, package_id, ref_id
        );

        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.token)
            .multipart(form)
            .send()
            .await
            .context("Failed to send upload request")?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("Upload failed with status {}: {}", status, text);
        }
    }

    pub async fn build_package(
        &self,
        package_id: i32,
        version: &str,
        target: Option<&str>,
    ) -> Result<Vec<String>> {
        let url = if let Some(target) = target {
            format!(
                "{}/api/build_package_single?packageId={}&distroType={}",
                self.base_url, package_id, target
            )
        } else {
            format!(
                "{}/api/build_package?packageId={}",
                self.base_url, package_id
            )
        };

        let form = reqwest::multipart::Form::new().text("version", version.to_string());

        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.token)
            .multipart(form)
            .send()
            .await
            .context("Failed to send build request")?;

        if response.status().is_success() {
            let build_response: BuildResponse = response
                .json()
                .await
                .context("Failed to parse build response")?;

            println!(
                "Build triggered successfully for package {} version {} ({} job(s))",
                package_id,
                version,
                build_response.job_ids.len()
            );

            Ok(build_response.job_ids)
        } else {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("Build request failed with status {}: {}", status, text);
        }
    }

    pub async fn check_build_status(&self, job_ids: &[String]) -> Result<BuildStatusResponse> {
        let job_ids_str = job_ids.join(",");
        let url = format!("{}/api/build_status?jobIds={}", self.base_url, job_ids_str);

        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.token)
            .send()
            .await
            .context("Failed to check build status")?;

        if response.status().is_success() {
            let status: BuildStatusResponse = response
                .json()
                .await
                .context("Failed to parse status response")?;
            Ok(status)
        } else {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("Status check failed with status {}: {}", status, text);
        }
    }
}

#[derive(Deserialize)]
struct BuildResponse {
    #[serde(rename = "jobIds")]
    job_ids: Vec<String>,
}

#[derive(Deserialize)]
pub struct BuildStatusResponse {
    pub jobs: Vec<JobStatus>,
    #[serde(rename = "allFinished")]
    pub all_finished: bool,
    #[serde(rename = "anyFailed")]
    pub any_failed: bool,
}

#[derive(Deserialize)]
pub struct JobStatus {
    #[serde(rename = "jobId")]
    pub job_id: String,
    pub status: String,
    pub name: String,
    #[serde(rename = "failMessage")]
    pub fail_message: Option<String>,
    #[serde(rename = "technicalError")]
    pub technical_error: Option<String>,
}
