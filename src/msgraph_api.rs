pub mod me;
pub mod users;
pub mod v1;
use clap::ValueEnum;

use crate::helpers::QueryConfig;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ApiVersion {
    V1,
    Beta,
}

pub trait ApiClient: me::MeApi + users::UsersApi {
    /// Get the access token
    fn get_token(&self) -> &str;
    /// Get the reqwest client
    fn get_client(&self) -> &reqwest::Client;
    /// Get the base path
    fn get_base_path(&self) -> &str;
    /// Convert QueryConfig into reqwest params format
    fn query_config_to_params(&self, query_config: &QueryConfig) -> Vec<(&str, String)>;
}

pub fn create_api_client(
    api_version: ApiVersion,
    token: String,
    proxy: Option<String>,
    nossl: bool,
) -> Box<dyn ApiClient> {
    match api_version {
        ApiVersion::V1 => Box::new(v1::client::ApiClientV1::new(token, proxy, nossl)),
        ApiVersion::Beta => unimplemented!("Beta API not implemented yet"),
    }
}
