use reqwest::Error as ReqwestError;
use serde_json::Value;
pub trait UsersApi {
    /// Get the number of users in the tenant
    fn get_users_count(&self, params: Option<Vec<(&str, String)>>) -> Result<usize, ReqwestError>;
    /// Get the list of users in the tenant
    fn get_users(&self, params: Option<Vec<(&str, String)>>) -> Result<Value, ReqwestError>;
}
