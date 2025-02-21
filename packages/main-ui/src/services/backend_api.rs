#![allow(non_snake_case)]
use by_macros::DioxusController;
use dioxus::prelude::*;
use dto::*;

use crate::config;

#[derive(Clone, Copy, Default, DioxusController)]
pub struct BackendApi {
    pub endpoint: &'static str,
}

impl BackendApi {
    pub fn init() {
        let endpoint = config::get().main_api_endpoint;
        let srv = Self { endpoint };
        use_context_provider(move || srv);
    }

    pub async fn get_account_hint(
        &self,
        provider: &str,
        code: &str,
        redirect_uri: &str,
    ) -> Result<AccountHint> {
        rest_api::get(&format!(
            "{}/v1/auth?provider={}&code={}&redirect-uri={}",
            self.endpoint, provider, code, redirect_uri
        ))
        .await
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccountHint {
    pub address_hint: String,
    pub private_key_hint: String,
    pub restore_key: String,
    pub id: String,
    pub address: Option<String>,
}

impl AccountHint {
    pub fn seed(&self) -> Vec<u8> {
        hex::decode(&self.private_key_hint).unwrap()
    }

    pub fn is_registered(&self) -> bool {
        self.address.is_some()
    }
}
