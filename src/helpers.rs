use crate::{core::constants::{DEFAULT_SCOPES, USER_AGENTS_KEYS}, msgraph_api::ApiVersion};
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
    /// Tenant ID to use for API requests
    #[clap(short = 't', long, default_value = "common")]
    pub tenant_id: String,
    /// Comma-separated list of scopes to use for API requests
    #[clap(short = 'S', long, default_value = DEFAULT_SCOPES)]
    pub scopes: String,
    /// Set access token to use for API requests
    #[clap(short = 'k', long, env = "REVELIO_TOKEN")]
    pub access_token: Option<String>,
    /// API version to use for API requests
    #[clap(short = 'v', long, default_value = "v1")]
    pub api_version: ApiVersion,
    /// User-agent to use for API requests
    #[clap(short = 'U', long, value_parser = USER_AGENTS_KEYS, default_value = "win_chrome_win10")]
    pub user_agent: String,
    /// Set proxy to use for API requests (except for authentication)
    #[clap(short = 'x', long)]
    pub proxy: Option<String>,
    /// Ignore SSL certificate verification
    #[clap(short = 'i', long)]
    pub ignore_ssl: bool,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Get resources in a tenant
    Get { resource: Resource },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Resource {
    /// Get the profile of the current user
    Me,
    /// Get the list of users in the tenant
    Users,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    pub client_id: String,
    pub client_secret: String,
    pub tenant_id: String,
    pub access_token: Option<String>,
    pub scopes: String,
    pub user_agent: String,
}

impl ClientConfig {
    pub fn new(
        client_id: String,
        client_secret: String,
        tenant_id: String,
        access_token: Option<String>,
        scopes: String,
        user_agent: String,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            tenant_id,
            access_token,
            scopes,
            user_agent,
        }
    }
}
