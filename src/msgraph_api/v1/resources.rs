use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct UserProfile {
    pub businessPhones: Vec<String>,
    pub displayName: String,
    pub givenName: String,
    pub jobTitle: String,
    pub mail: String,
    pub mobilePhone: Option<String>,
    pub officeLocation: String,
    pub preferredLanguage: Option<String>,
    pub surname: String,
    pub userPrincipalName: String,
    pub id: String,
}
