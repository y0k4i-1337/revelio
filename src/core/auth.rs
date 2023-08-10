use crate::core::constants::{AUTH_URL, REDIRECT_URL, TOKEN_URL};
use crate::helpers::ClientConfig;
use chrono::Utc;
use colored::Colorize;
use dialoguer::Input;
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{
    AccessToken, AuthType, AuthUrl, AuthorizationCode, ClientId, CsrfToken, PkceCodeChallenge,
    RedirectUrl, Scope, TokenResponse, TokenUrl, PkceCodeVerifier,
};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::io::{self, Read};

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

pub fn authenticate_credential(config: &ClientConfig) -> AuthResult {
    let client = create_oauth_client(&config.client_id);
    let (auth_url, pkce_code_verifier) = generate_auth_url(&client);
    println!(
        "{}\n{}\n",
        "Please visit the following URL to authenticate your account:",
        auth_url.blue()
    );

    let auth_redirect_url: String = Input::<String>::new()
        .with_prompt("Paste the URL you were redirected to")
        .interact_text()
        .expect("Failed to read input");

    // Remove newline characters
    let auth_redirect_url = auth_redirect_url.replace('\n', "");

    println!("URL: {}", auth_redirect_url);

    let auth_code = parse_code_from_redirect_url(&auth_redirect_url);
    let token = exchange_code_for_token(&client, &auth_code, pkce_code_verifier);

    process_raw_auth_result(config, &token)
}

fn create_oauth_client(client_id: &str) -> BasicClient {
    let auth_url = AuthUrl::new(AUTH_URL.to_string()).expect("Invalid authorization URL");
    let token_url = TokenUrl::new(TOKEN_URL.to_string()).expect("Invalid token URL");

    BasicClient::new(
        ClientId::new(client_id.to_string()),
        None,
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(RedirectUrl::new(REDIRECT_URL.to_string()).expect("Invalid redirect URL"))
    // Microsoft Graph requires client_id in URL rather than
    // using Basic authentication.
    .set_auth_type(AuthType::RequestBody)
}

fn generate_auth_url(client: &BasicClient) ->(String, PkceCodeVerifier) {
    // Microsoft Graph supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
    // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, _csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        // Utilize the same scope as the Microsoft Graph Explorer.
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("offline_access".to_string()))
        .add_scope(Scope::new("User.Read".to_string()))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    (authorize_url.to_string(), pkce_code_verifier)
}

fn parse_code_from_redirect_url(redirect_url: &str) -> String {
    let url = Url::parse(redirect_url).expect("Failed to parse redirect URL");
    let code = url
        .query_pairs()
        .find(|(param, _)| *param == "code")
        .expect("No code in redirect URL")
        .1
        .to_string();
    code
}

fn exchange_code_for_token(client: &BasicClient, code: &str, pkce_code_verifier: PkceCodeVerifier ) -> AccessToken {
    // Exchange the code with a token.
    let token_result = client
        .exchange_code(AuthorizationCode::new(code.to_string()))
        // Send the PKCE code verifier in the token request
        .set_pkce_verifier(pkce_code_verifier)
        .request(http_client)
        .expect("Failed to create token");

    token_result.access_token().clone()
}

fn process_raw_auth_result(credential: &ClientConfig, token_result: &AccessToken) -> AuthResult {
    // Implement the processing logic here
    AuthResult::new(credential.clone(), Some(token_result.secret().to_string()))
}

pub fn export_auth_results(auth_results: &[AuthResult]) {
    let export_file = format!(
        "spray365_results_{}.json",
        Utc::now().format("%Y-%m-%d_%H-%M-%S")
    );

    let json_execution_plan =
        serde_json::to_string(auth_results).expect("JSON serialization error");

    let mut file = File::create(&export_file).expect("Failed to create export file");
    file.write_all(json_execution_plan.as_bytes())
        .expect("Failed to write to export file");

    println!("Authentication results saved to file '{}'", export_file);
}
