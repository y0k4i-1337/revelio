use reqwest::Error as ReqwestError;
use serde_json::Value;
pub trait UsersApi {
    /// Get the number of users in the tenant
    fn get_users_count(&self) -> Result<usize, ReqwestError>;
    /// Get the list of users in the tenant
    fn get_users(&self) -> Result<Value, ReqwestError>;
}
