use reqwest::Error as ReqwestError;
use reqwest::header::HeaderMap;
use serde_json::Value;

pub trait MeApi {
    /// Get profile of the current user
    fn get_me(&self, headers: Option<HeaderMap>, params: Option<Vec<(&str, &str)>>) -> Result<Value, ReqwestError>;
}
