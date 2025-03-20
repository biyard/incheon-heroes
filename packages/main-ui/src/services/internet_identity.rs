use crate::config;
use by_macros::DioxusController;
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use ic_agent::Agent;
use ic_agent::Identity;
use ic_agent::Signature;
use ic_agent::agent::EnvelopeContent;
use ic_agent::export::Principal;
use ic_agent::identity::BasicIdentity;
use ic_agent::identity::Delegation;
use std::sync::Arc;

pub const INTERNET_IDENTITY_KEY: &str = "internet_identity";

#[derive(Clone)]
pub struct ArcIdentity(Arc<BasicIdentity>);

impl Identity for ArcIdentity {
    fn sender(&self) -> Result<Principal, String> {
        self.0.sender()
    }

    fn public_key(&self) -> Option<Vec<u8>> {
        self.0.public_key()
    }

    fn sign(&self, content: &EnvelopeContent) -> Result<Signature, String> {
        self.0.sign(content)
    }

    fn sign_delegation(&self, content: &Delegation) -> Result<Signature, String> {
        self.0.sign_delegation(content)
    }

    fn sign_arbitrary(&self, content: &[u8]) -> Result<Signature, String> {
        self.0.sign_arbitrary(content)
    }
}

#[derive(Clone, Copy, DioxusController)]
pub struct InternetIdentityService {
    agent: Signal<Option<Agent>>,
    identity: Signal<Option<ArcIdentity>>,
}

impl InternetIdentityService {
    pub fn init() {
        let srv = Self {
            agent: use_signal(|| None),
            identity: use_signal(|| None),
        };
        use_context_provider(move || srv);
    }

    pub async fn login(&mut self, identity: BasicIdentity) {
        let identity = ArcIdentity(Arc::new(identity));
        let principal = identity.sender().unwrap().to_text();

        let agent = Agent::builder()
            .with_url(config::get().icp.endpoint)
            .with_identity(identity.clone())
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

    pub fn get_identity(&self) -> Option<ArcIdentity> {
        self.identity()
    }

    pub fn get_agent(&self) -> Option<Agent> {
        self.agent()
    }
}
