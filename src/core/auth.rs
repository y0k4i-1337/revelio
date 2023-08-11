use crate::helpers::ClientConfig;
use chrono::Utc;
use colored::Colorize;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AccessToken, AuthUrl, ClientId, ClientSecret, DeviceAuthorizationUrl, Scope,
    StandardDeviceAuthorizationResponse, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResult {
    pub credential: ClientConfig,
    pub auth_token: Option<String>,
}

impl AuthResult {
    pub fn new(credential: ClientConfig, auth_token: Option<String>) -> Self {
        AuthResult {
            credential,
            auth_token,
        }
    }
}

pub async fn authenticate_credential(config: &ClientConfig) -> AuthResult {
    let client = create_oauth_client(config);
    let auth_url_details = generate_auth_url(&client, config).await;
    println!(
        "\nUse the code {} at {} to authenticate your account\n",
        auth_url_details.user_code().secret().bold().green(),
        auth_url_details.verification_uri().blue()
    );
    println!("Waiting for authentication...");

    let token_result = client
        .exchange_device_access_token(&auth_url_details)
        .request_async(async_http_client, tokio::time::sleep, None)
        .await;

    eprintln!("Token:{:?}", token_result);
    let access_token = token_result.as_ref().unwrap().access_token();
    eprintln!("Access token:{:?}", access_token);

    process_raw_auth_result(config, access_token)
}

/// Create an OAuth2 client according to the given configuration.
fn create_oauth_client(config: &ClientConfig) -> BasicClient {
    let auth_url = AuthUrl::new(format!(
        "https://login.microsoftonline.com/{}/oauth2/v2.0/authorize",
        config.tenant_id
    ))
    .expect("Invalid authorization URL");
    let token_url = TokenUrl::new(format!(
        "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
        config.tenant_id
    ))
    .expect("Invalid token URL");
    let device_auth_url = DeviceAuthorizationUrl::new(format!(
        "https://login.microsoftonline.com/{}/oauth2/v2.0/devicecode",
        config.tenant_id
    ))
    .expect("Invalid device authorization URL");

    BasicClient::new(
        ClientId::new(config.client_id.to_string()),
        Some(ClientSecret::new(config.client_secret.to_string())),
        auth_url,
        Some(token_url),
    )
    // Set the device authorization URL
    .set_device_authorization_url(device_auth_url)
    .set_auth_type(oauth2::AuthType::RequestBody)
}

/// Generate the authorization URL and the user code.
async fn generate_auth_url(
    client: &BasicClient,
    config: &ClientConfig,
) -> StandardDeviceAuthorizationResponse {
    let scopes = config
        .scopes
        .split(',')
        .map(|s| Scope::new(s.trim().to_string()))
        .collect::<Vec<_>>();
    // Generate the URL where the user will be redirected to authorize the client.
    let details: StandardDeviceAuthorizationResponse = client
        .exchange_device_code()
        .expect("Failed to exchange device code")
        .add_scopes(scopes)
        .request_async(async_http_client)
        .await
        .expect("Failed to create auth URL");
    details
}

fn process_raw_auth_result(credential: &ClientConfig, token_result: &AccessToken) -> AuthResult {
    // Implement the processing logic here
    AuthResult::new(credential.clone(), Some(token_result.secret().to_string()))
}

pub fn export_auth_results(auth_results: &[AuthResult]) {
    let export_file = format!(
        "revelio_result_{}.json",
        Utc::now().format("%Y-%m-%d_%H-%M-%S")
    );

    let json_execution_plan =
        serde_json::to_string(auth_results).expect("JSON serialization error");

    let mut file = File::create(&export_file).expect("Failed to create export file");
    file.write_all(json_execution_plan.as_bytes())
        .expect("Failed to write to export file");

    println!("Authentication results saved to file '{}'", export_file);
}
