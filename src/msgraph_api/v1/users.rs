use async_trait::async_trait;
use colored::Colorize;
use reqwest::header::HeaderMap;
use reqwest::Url;
use serde_json::Value;

use super::client::ApiClientV1;
use crate::msgraph_api::users::UsersApi;
use crate::msgraph_api::ApiClient;

#[async_trait(?Send)]
impl UsersApi for ApiClientV1 {
    async fn get_users_count(
        &self,
        params: Option<Vec<(&str, String)>>,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        if !self.check_access_token_validity().await? {
            return Err("Access token is not valid".into());
        }
        let mut headers = HeaderMap::new();
        let params = params.unwrap_or_default();
        headers.insert(
            "Authorization",
            format!("Bearer {}", self.get_token()).parse().unwrap(),
        );
        headers.insert("ConsistencyLevel", "eventual".parse().unwrap());
        let url = format!("{}/users/$count", self.get_base_path());
        match self
            .get_client()
            .get(url.as_str())
            .headers(headers)
            .query(&params)
            .send()
            .await?
            .text()
            .await
        {
            Ok(text) => {
                return Ok(text.parse::<usize>().unwrap());
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    async fn get_users(
        &self,
        params: Option<Vec<(&str, String)>>,
        pages: u16,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        if !self.check_access_token_validity().await? {
            return Err("Access token is not valid".into());
        }
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
        let mut skiptoken = String::new();
        loop {
            let response = self
                .get_client()
                .get(url.clone())
                .headers(headers.clone())
                .send()
                .await?;
            // Token has possibly expired
            if !response.status().is_success() && !skiptoken.is_empty() {
                eprintln!(
                    "Access token has probably expired. Latest skiptoken: {}",
                    skiptoken.blue()
                );
                break;
            }

            let response_json = match response.json::<Value>().await {
                Ok(response_json) => response_json,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    break;
                }
            };
            match response_json["value"] {
                Value::Array(ref users) => {
                    if users.is_empty() {
                        break;
                    } else {
                        all_users.extend(users.iter().cloned());
                    }
                }
                _ => {
                    break;
                }
            }

            page_count += 1;
            if pages > 0 && page_count >= pages {
                break;
            }

            if let Some(next_link) = response_json["@odata.nextLink"].as_str() {
                url = Url::parse(next_link).expect("Failed to parse url");
                // Extract skiptoken parameter from next_link
                for (key, value) in url.query_pairs() {
                    if key == "$skiptoken" {
                        skiptoken = value.to_string();
                        break;
                    }
                }
            } else {
                break;
            }
        }
        Ok(Value::Array(all_users))
    }
}
