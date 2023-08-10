use crate::core::constants::USER_AGENTS_KEYS;
use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Custom client ID to use for API requests
    #[clap(short = 'c', long)]
    pub client_id: Option<String>,
    /// Custom client secret to use for API requests
    #[clap(short = 's', long)]
    pub client_secret: Option<String>,
    /// User-agent to use for API requests
    #[clap(short = 'U', long, default_value = "revelio", value_parser = USER_AGENTS_KEYS, default_value = "win_chrome_win10")]
    pub user_agent: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get resources in a tenant
    Get { resource: Resource },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Resource {
    /// Get the profile of the current user
    Me,
    /// Get the list of users in the tenant
    Users,
    /// Get the list of groups in the tenant
    Groups,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    pub client_id: String,
    pub client_secret: String,
    pub user_agent: String,
}

impl ClientConfig {
    pub fn new(client_id: String, client_secret: String, user_agent: String) -> Self {
        Self {
            client_id,
            client_secret,
            user_agent,
        }
    }
}
