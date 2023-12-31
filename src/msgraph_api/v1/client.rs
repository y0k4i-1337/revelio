use crate::{helpers::QueryConfig, msgraph_api::ApiClient};

pub struct ApiClientV1 {
    client: reqwest::Client,
    base_path: String,
    token: String,
}

impl ApiClientV1 {
    pub fn new(token: String, proxy: Option<String>, nossl: bool) -> Self {
        let client = match proxy {
            Some(proxy) => reqwest::Client::builder()
                .proxy(reqwest::Proxy::all(proxy).unwrap())
                .danger_accept_invalid_certs(nossl)
                .build()
                .expect("Failed to create reqwest client"),
            None => reqwest::Client::builder()
                .danger_accept_invalid_certs(nossl)
                .build()
                .expect("Failed to create reqwest client"),
        };
        let base_path = "https://graph.microsoft.com/v1.0".to_owned();
        ApiClientV1 {
            client,
            base_path,
            token,
        }
    }

    pub async fn check_access_token_validity(&self) -> Result<bool, reqwest::Error> {
        let response = self
            .client
            .get(format!("{}/me", self.base_path))
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?;

        if response.status().is_success() {
            Ok(true) // Access token is valid
        } else {
            Ok(false) // Access token is not valid
        }
    }
}

impl ApiClient for ApiClientV1 {
    fn get_token(&self) -> &str {
        &self.token
    }
    fn get_client(&self) -> &reqwest::Client {
        &self.client
    }
    fn get_base_path(&self) -> &str {
        &self.base_path
    }
    fn query_config_to_params(&self, query_config: &QueryConfig) -> Vec<(&str, String)> {
        let mut query_vec: Vec<(&str, String)> = Vec::new();
        if let Some(select) = &query_config.select {
            query_vec.push(("$select", select.clone()));
        }
        if let Some(skiptoken) = &query_config.skiptoken {
            query_vec.push(("$skiptoken", skiptoken.clone()));
        }
        query_vec.push(("$top", query_config.top.to_string()));
        query_vec
    }
}
