use clap::Parser;
use revelio::core::auth::authenticate_credential;
use revelio::core::constants::DEFAULT_CLIENT_ID;
use revelio::helpers::{Cli, ClientConfig, Commands, Resource};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let mut config = ClientConfig::new(
        cli.client_id
            .unwrap_or_else(|| DEFAULT_CLIENT_ID.to_string()),
        cli.client_secret.unwrap_or_default(),
        cli.tenant_id,
        cli.access_token,
        cli.scopes,
        cli.user_agent,
    );

    // Authenticate user if no access token is provided
    match &config.access_token {
        Some(_) => {}
        None => {
            let auth_result = authenticate_credential(&config).await;
            config.access_token = auth_result.auth_token.to_owned();
        }
    }

    // Exit if authentication failed
    if config.access_token.is_none() {
        eprintln!("Authentication failed");
        std::process::exit(1);
    }

    match cli.command {
        Commands::Get { resource } => match resource {
            Resource::Me => {
                println!("Me");
                eprintln!("{:?}", config)
            }
            Resource::Users => {
                println!("Users");
            }
        },
    }

    Ok(())
}
