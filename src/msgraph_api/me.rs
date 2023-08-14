use async_trait::async_trait;
use reqwest::header::HeaderMap;
use reqwest::Error as ReqwestError;
use serde_json::Value;

#[async_trait(?Send)]
pub trait MeApi {
    /// Get profile of the current user
    async fn get_me(
        &self,
        headers: Option<HeaderMap>,
        params: Option<Vec<(&str, &str)>>,
    ) -> Result<Value, ReqwestError>;
}
