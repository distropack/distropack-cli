use anyhow::{Context, Result};
use std::time::Duration;
use tokio::time::sleep;
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
    
    let job_ids = client.build_package(package_id, version, target)
        .await
        .context("Failed to trigger build")?;
    
    if job_ids.is_empty() {
        anyhow::bail!("No build jobs were created");
    }
    
    println!("Waiting for {} build job(s) to complete...", job_ids.len());
    
    // Poll for build status
    let poll_interval = Duration::from_secs(5);
    let mut iteration = 0;
    let mut seen_statuses: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    
    loop {
        let status = client.check_build_status(&job_ids)
            .await
            .context("Failed to check build status")?;
        
        // Track which jobs we've seen status for
        let found_job_ids: std::collections::HashSet<_> = status.jobs.iter()
            .map(|j| &j.job_id)
            .collect();
        
        // Print status for jobs (only when status changes or periodically for running jobs)
        for job in &status.jobs {
            let previous_status = seen_statuses.get(&job.job_id);
            let status_changed = previous_status.map(|s| s != &job.status).unwrap_or(true);
            
            match job.status.as_str() {
                "running" => {
                    if status_changed || iteration % 3 == 0 {
                        println!("  {}: Building...", job.name);
                    }
                }
                "finished" => {
                    if status_changed {
                        println!("  {}: ✓ Completed", job.name);
                    }
                }
                "failed" => {
                    if status_changed {
                        println!("  {}: ✗ Failed", job.name);
                        if let Some(ref msg) = job.fail_message {
                            println!("    Error: {}", msg);
                        }
                    }
                }
                _ => {}
            }
            
            seen_statuses.insert(job.job_id.clone(), job.status.clone());
        }
        
        // If some jobs aren't in the response yet, they're still initializing
        let missing_count = job_ids.len() - found_job_ids.len();
        if missing_count > 0 && iteration % 3 == 0 {
            println!("  {} job(s) still initializing...", missing_count);
        }
        
        if status.all_finished {
            println!("\nBuild completed!");
            
            if status.any_failed {
                // Print all failure messages
                for job in &status.jobs {
                    if job.status == "failed" {
                        if let Some(ref msg) = job.fail_message {
                            eprintln!("  {} failed: {}", job.name, msg);
                        }
                        if let Some(ref tech) = job.technical_error {
                            eprintln!("    Technical details: {}", tech);
                        }
                    }
                }
                std::process::exit(1);
            } else {
                println!("All builds completed successfully!");
                return Ok(());
            }
        }
        
        iteration += 1;
        sleep(poll_interval).await;
    }
}


