use by_macros::DioxusController;
use dioxus::prelude::*;
use ic_agent::{Agent, Identity};
use web_sys::Url;

use crate::config;

#[derive(Clone, Copy, DioxusController)]
pub struct InternetIdentityService {
    agent: Signal<Agent>,
}

impl InternetIdentityService {
    pub fn init() {
        let conf = &config::get().icp;
        let agent = Agent::builder()
            .with_url(conf.identity_provider)
            .build()
            .expect("failed to build default agent");

        let srv = Self {
            agent: use_signal(move || agent),
        };

        use_context_provider(move || srv);
    }

    pub async fn login(&self) -> Result<String, String> {
        let mut auth_client = ic_auth_client::AuthClient::new()
            .await
            .map_err(|e| format!("Failed to create auth client: {}", e))?;

        let identity_provider_url = Url::new(config::get().icp.identity_provider)
            .map_err(|e| format!("Failed to parse identity provider URL: {:?}", e))?;

        let login_options = ic_auth_client::AuthClientLoginOptions::builder()
            .identity_provider(identity_provider_url)
            .on_success(|response| {
                tracing::debug!("Login success: {:?}", response);
            })
            .on_error(|error| {
                tracing::error!("Login error: {:?}", error);
            })
            .build();

        auth_client.login_with_options(login_options);

        let identity = auth_client.identity();

        let principal = identity.sender().unwrap().to_string();

        Ok(principal)
    }

    pub fn get_agent(&self) -> Agent {
        self.agent()
    }
}
