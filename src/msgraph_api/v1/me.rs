use async_trait::async_trait;
use reqwest::header::HeaderMap;
use reqwest::Error as ReqwestError;
use serde_json::Value;

use super::super::me::MeApi;
use super::super::ApiClient;
use super::client::ApiClientV1;

#[async_trait(?Send)]
impl MeApi for ApiClientV1 {
    async fn get_me(
        &self,
        params: Option<Vec<(&str,String)>>,
    ) -> Result<Value, ReqwestError> {
        let mut headers = HeaderMap::new();
        let mut params = params.unwrap_or_default();
        let mut has_select = false;
        for (key, _) in params.iter() {
            if key == &"$select" {
                has_select = true;
            }
        }
        if !has_select {
            params.append(&mut vec![(
                "$select",
                "id,businessPhones,displayName,givenName,\
            jobTitle,mail,mobilePhone,officeLocation,surname,userPrincipalName,\
            onPremisesDistinguishedName,onPremisesDomainName,onPremisesLastSyncDateTime,\
            onPremisesSecurityIdentifier,onPremisesSamAccountName,onPremisesSyncEnabled,\
            onPremisesUserPrincipalName,passwordPolicies".to_owned(),
            )]);
        }
        headers.insert(
            "Authorization",
            format!("Bearer {}", self.get_token()).parse().unwrap(),
        );
        let url = format!("{}/me", self.get_base_path());
        self.get_client()
            .get(url.as_str())
            .headers(headers)
            .query(&params)
            .send()
            .await?
            .json::<Value>()
            .await
    }
}
