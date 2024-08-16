use crate::{
    core::{auth, constants::{DEFAULT_SCOPES, USER_AGENTS_KEYS}},
    msgraph_api::ApiVersion,
};
use clap::{Args, Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{ops::RangeInclusive, path::PathBuf};

// Function to save JSON response to a file in pretty format
pub fn save_json_to_file(
    output_dir: &str,
    file_name: &str,
    json_value: &Value,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a PathBuf for the output directory
    let mut output_path = PathBuf::from(output_dir);

    // Add the filename to the output path
    output_path.push(file_name);

    // Write the JSON response to the output file in pretty format
    std::fs::write(&output_path, serde_json::to_string_pretty(json_value)?)?;

    eprintln!("JSON response saved to: {:?}", output_path);

    Ok(())
}

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
    /// Tenant ID to use for API requests (e.g. organizations, common, consumers,
    /// tenant ID or domain)
    #[clap(short = 't', long, default_value = "organizations")]
    pub tenant_id: String,
    /// Comma-separated list of scopes to use for API requests
    #[clap(short = 'S', long, default_value = DEFAULT_SCOPES)]
    pub scopes: String,
    /// Authentication flow to use for API requests
    #[clap(value_enum, short = 'f', long, default_value_t = AuthFlow::Device)]
    pub flow: AuthFlow,
    /// Set access token to use for API requests
    #[clap(short = 'k', long, env = "REVELIO_TOKEN")]
    pub access_token: Option<String>,
    /// Username to use for password authentication flow
    #[clap(short = 'u', long)]
    pub username: Option<String>,
    /// Password to use for password authentication flow
    #[clap(short = 'p', long)]
    pub password: Option<String>,
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
    /// Output directory (only used when retrieving large amounts of data)
    #[clap(short = 'o', long, default_value = ".")]
    pub out_dir: String,
    #[command(subcommand)]
    pub command: Commands,
}

/// Enum for authentication flow
#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum AuthFlow {
    /// Device code flow
    Device,
    /// Authorization code flow
    Code,
    /// Client credentials flow
    Client,
    /// Resource owner password credentials flow
    Password,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Get resources in a tenant
    Get(GetArgs),
}

#[derive(Args)]
pub struct GetArgs {
    /// Custom select query parameter (properties to return)
    #[clap(long)]
    pub select: Option<String>,
    /// Custom top query parameter (page size of results)
    #[clap(long, value_parser = top_in_range, default_value = "500")]
    pub top: u16,
    /// Set skiptoken to continue from a previous request
    #[clap(long)]
    pub skiptoken: Option<String>,
    /// Maximum number of pages to return (0 for all pages)
    #[clap(long, value_parser = clap::value_parser!(u16).range(0..), default_value = "0")]
    pub pages: u16,
    /// Resource to get
    pub resource: Resource,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Resource {
    /// Get the profile of the current user
    Me,
    /// Get the list of users in the tenant
    Users,
    /// Get the number of users in the tenant
    UsersCount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    pub client_id: String,
    pub client_secret: String,
    pub tenant_id: String,
    pub auth_flow: AuthFlow,
    pub access_token: Option<String>,
    pub scopes: String,
    pub user_agent: String,
}

impl ClientConfig {
    pub fn new(
        client_id: String,
        client_secret: String,
        tenant_id: String,
        auth_flow: AuthFlow,
        access_token: Option<String>,
        scopes: String,
        user_agent: String,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            tenant_id,
            auth_flow,
            access_token,
            scopes,
            user_agent,
        }
    }
}

pub struct QueryConfig {
    pub select: Option<String>,
    pub skiptoken: Option<String>,
    pub top: u16,
}

impl QueryConfig {
    pub fn new(select: Option<String>, skiptoken: Option<String>, top: u16) -> Self {
        Self {
            select,
            skiptoken,
            top,
        }
    }
}

const TOP_RANGE: RangeInclusive<usize> = 1..=999;

fn top_in_range(s: &str) -> Result<u16, String> {
    let top: usize = s
        .parse()
        .map_err(|_| format!("`{s}` isn't a valid top value"))?;
    if TOP_RANGE.contains(&top) {
        Ok(top as u16)
    } else {
        Err(format!(
            "top not in range {}-{}",
            TOP_RANGE.start(),
            TOP_RANGE.end()
        ))
    }
}
