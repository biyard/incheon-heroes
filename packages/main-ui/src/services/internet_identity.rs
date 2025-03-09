use by_macros::DioxusController;
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use ic_agent::Agent;
use ic_agent::Identity;
use ic_agent::export::Principal;
use ic_agent::identity::BasicIdentity;
// use std::sync::Arc;
use crate::config;

const INTERNET_IDENTITY_KEY: &str = "internet_identity";

#[derive(Clone, Copy, DioxusController)]
pub struct InternetIdentityService {
    agent: Signal<Option<Agent>>,
    identity: Signal<Option<BasicIdentity>>,
}

impl InternetIdentityService {
    pub fn new() -> Self {
        let agent = use_signal(|| None);
        let identity = use_signal(|| None);

        Self { agent, identity }
    }

    pub fn init() {
        use_context_provider(|| Self::new());
    }

    pub async fn login(&mut self, identity: BasicIdentity) {
        let principal = identity.sender().unwrap().to_text();

        let agent = Agent::builder()
            .with_url(config::get().icp.endpoint)
            .with_identity(identity)
            .build()
            .unwrap();

        self.agent.set(Some(agent));
        self.identity.set(Some(identity));

        LocalStorage::set(INTERNET_IDENTITY_KEY, principal).unwrap();
    }

    pub async fn logout(&mut self) {
        self.agent.set(None);
        self.identity.set(None);
        LocalStorage::delete(INTERNET_IDENTITY_KEY);
    }

    pub fn is_logged_in(&self) -> bool {
        self.agent().is_some()
    }

    pub fn get_principal(&self) -> Option<Principal> {
        self.identity().map(|identity| identity.sender().unwrap())
    }

    pub fn get_identity(&self) -> Option<BasicIdentity> {
        self.identity()
    }

    pub fn get_agent(&self) -> Option<Agent> {
        self.agent()
    }
}
