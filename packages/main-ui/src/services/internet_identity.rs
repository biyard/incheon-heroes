#![allow(non_snake_case)]
use by_macros::DioxusController;
use dioxus::prelude::*;
use ic_agent::{Agent, Identity as _, identity::BasicIdentity};
use std::io::Cursor;

#[derive(Clone, Copy, DioxusController)]
pub struct InternetIdentityService {
    pub agent: Signal<Agent>,
    pub identity: Signal<Option<BasicIdentity>>,
}

impl InternetIdentityService {
    pub fn init() {
        let conf = &crate::config::get().icp;
        let agent = Agent::builder()
            .with_url(conf.endpoint)
            .build()
            .expect("failed to build default agent");

        let srv = Self {
            agent: use_signal(move || agent),
            identity: use_signal(|| None),
        };

        use_context_provider(move || srv);
    }

    pub fn instance() -> Self {
        use_context()
    }

    pub async fn login(&mut self) -> Result<String, String> {
        // Create an anonymous identity first
        let empty_pem = Cursor::new(Vec::new());
        let identity = BasicIdentity::from_pem(empty_pem).map_err(|e| e.to_string())?;
        let principal = identity.sender().map_err(|e| e.to_string())?.to_text();

        // Store the identity first
        self.identity.set(Some(identity));

        // Then set it on the agent (this will move the identity)
        if let Some(identity) = self.identity.take() {
            self.agent().set_identity(identity);
        }

        Ok(principal)
    }

    pub fn is_logged_in(&self) -> bool {
        self.identity().is_some()
    }

    pub fn principal(&self) -> Option<String> {
        self.identity()
            .as_ref()
            .and_then(|id| id.sender().ok().map(|p| p.to_text()))
    }

    pub async fn logout(&mut self) {
        self.identity.set(None);
        // Reset to anonymous identity
        let empty_pem = Cursor::new(Vec::new());
        self.agent()
            .set_identity(BasicIdentity::from_pem(empty_pem).unwrap());
    }
}
