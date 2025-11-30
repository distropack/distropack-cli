use anyhow::Result;
use clap::{Parser, Subcommand};

mod api;
mod auth;
mod commands;
mod config;

use commands::{build, config as config_cmd, upload};

#[derive(Parser)]
#[command(name = "distropack-cli")]
#[command(about = "DistroPack CLI - Automate Linux package builds from CI/CD pipelines")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Upload a file to a package by reference ID
    Upload {
        /// Package ID
        #[arg(long)]
        package_id: String,
        /// File reference ID (access name)
        #[arg(long)]
        ref_id: String,
        /// Path to file to upload
        #[arg(long)]
        file: String,
    },
    /// Trigger package build(s)
    Build {
        /// Package ID
        #[arg(long)]
        package_id: String,
        /// Version string
        #[arg(long)]
        version: String,
        /// Target distribution (deb, rpm, pacman). Omit to build all enabled targets.
        #[arg(long)]
        target: Option<String>,
    },
    /// Manage configuration
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Set API token
    SetToken {
        /// API token value
        token: String,
    },
    /// Set API base URL
    SetBaseUrl {
        /// Base URL (e.g., https://distropack.dev)
        url: String,
    },
    /// Show current configuration
    Show,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Upload {
            package_id,
            ref_id,
            file,
        } => upload::upload_file(&package_id, &ref_id, &file).await,
        Commands::Build {
            package_id,
            version,
            target,
        } => build::build_package(&package_id, &version, target.as_deref()).await,
        Commands::Config { command } => match command {
            ConfigCommands::SetToken { token } => config_cmd::set_token(&token).await,
            ConfigCommands::SetBaseUrl { url } => config_cmd::set_base_url(&url).await,
            ConfigCommands::Show => config_cmd::show_config().await,
        },
    }
}
