use async_trait::async_trait;
use serde_json::Value;

#[async_trait(?Send)]
pub trait MeApi {
    /// Get profile of the current user
    async fn get_me(
        &self,
        params: Option<Vec<(&str, String)>>,
    ) -> Result<Value, Box<dyn std::error::Error>>;
}
