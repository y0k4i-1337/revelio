pub struct ApiClient {
    client: reqwest::blocking::Client,
    base_path: String,
    token: String,
}

impl ApiClient {
    pub fn new(token: String) -> Self {
        let client = reqwest::blocking::Client::new();
        let base_path = "https://graph.microsoft.com/v1.0".to_owned();
        ApiClient {
            client,
            base_path,
            token,
        }
    }
    pub fn get_token(&self) -> &str {
        &self.token
    }
    pub fn get_client(&self) -> &reqwest::blocking::Client {
        &self.client
    }
    pub fn get_base_path(&self) -> &str {
        &self.base_path
    }
}
