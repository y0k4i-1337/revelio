use clap::Parser;
use revelio::core::auth::authenticate_credential;
use revelio::core::constants::{AUTH_URL, DEFAULT_CLIENT_ID, REDIRECT_URL, TOKEN_URL};
use revelio::helpers::{Cli, ClientConfig};

fn main() {
    let cli = Cli::parse();

    let config = ClientConfig::new(
        cli.client_id
            .unwrap_or_else(|| DEFAULT_CLIENT_ID.to_string()),
        cli.client_secret.unwrap_or_default(),
        cli.user_agent,
    );

    // println!("{:?}", config);

    // Authenticate the user
    let auth_result = authenticate_credential(&config);
    println!("{:?}", auth_result)
}
