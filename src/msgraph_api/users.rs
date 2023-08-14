use async_trait::async_trait;
use reqwest::Error as ReqwestError;
use serde_json::Value;
#[async_trait(?Send)]
pub trait UsersApi {
    /// Get the number of users in the tenant
    async fn get_users_count(
        &self,
        params: Option<Vec<(&str, String)>>,
    ) -> Result<usize, ReqwestError>;
    /// Get the list of users in the tenant
    async fn get_users(
        &self,
        params: Option<Vec<(&str, String)>>,
        pages: u16,
    ) -> Result<Value, ReqwestError>;
}
