use async_trait::async_trait;
use reqwest::header::HeaderMap;
use reqwest::{Error as ReqwestError, Url};
use serde_json::Value;

use super::client::ApiClientV1;
use crate::msgraph_api::users::UsersApi;
use crate::msgraph_api::ApiClient;

#[async_trait(?Send)]
impl UsersApi for ApiClientV1 {
    async fn get_users_count(
        &self,
        params: Option<Vec<(&str, String)>>,
    ) -> Result<usize, ReqwestError> {
        let mut headers = HeaderMap::new();
        let params = params.unwrap_or_default();
        headers.insert(
            "Authorization",
            format!("Bearer {}", self.get_token()).parse().unwrap(),
        );
        headers.insert("ConsistencyLevel", "eventual".parse().unwrap());
        let url = format!("{}/users/$count", self.get_base_path());
        self.get_client()
            .get(url.as_str())
            .headers(headers)
            .query(&params)
            .send()
            .await?
            .text()
            .await
            .map(|text| text.parse::<usize>().unwrap())
    }

    async fn get_users(
        &self,
        params: Option<Vec<(&str, String)>>,
        pages: u16,
    ) -> Result<Value, ReqwestError> {
        let mut params = params.unwrap_or_default();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", self.get_token()).parse().unwrap(),
        );
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
            onPremisesUserPrincipalName,passwordPolicies"
                    .to_owned(),
            )]);
        }
        let mut all_users: Vec<Value> = Vec::new();
        let mut url = Url::parse(format!("{}/users", self.get_base_path()).as_str())
            .expect("Failed to parse url");
        url.query_pairs_mut().extend_pairs(params);

        let mut page_count: u16 = 0;
        loop {
            let response = self
                .get_client()
                .get(url.clone())
                .headers(headers.clone())
                .send()
                .await?
                .json::<Value>()
                .await?;
            let users = response["value"].as_array().unwrap();
            all_users.extend(users.iter().cloned());

            page_count += 1;
            if pages > 0 && page_count >= pages {
                break;
            }

            if let Some(next_link) = response["@odata.nextLink"].as_str() {
                url = Url::parse(next_link).expect("Failed to parse url");
            } else {
                break;
            }
        }
        Ok(Value::Array(all_users))
    }
}
