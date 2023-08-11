use clap::Parser;
use revelio::core::auth::authenticate_credential;
use revelio::core::constants::DEFAULT_CLIENT_ID;
use revelio::helpers::{Cli, ClientConfig};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let config = ClientConfig::new(
        cli.client_id
            .unwrap_or_else(|| DEFAULT_CLIENT_ID.to_string()),
        cli.client_secret.unwrap_or_default(),
        cli.tenant_id,
        cli.scopes,
        cli.user_agent,
    );

    println!("{:?}", config);

    // Authenticate the user
    let auth_result = authenticate_credential(&config).await;
    println!("{:?}", auth_result);
    Ok(())
}
