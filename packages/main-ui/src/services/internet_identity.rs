use std::sync::Arc;

use async_trait::async_trait;
use candid::{encode_args, utils::ArgumentEncoder, CandidType, Decode, Principal, Deserialize};
use ic_agent::{Agent, identity::BasicIdentity};
use serde::Serialize;

use crate::{
    config,
    models::user_wallet::UserWallet,
    services::user_service::UserService,
    Result,
};

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct Delegation {
    pub pubkey: Vec<u8>,
    pub expiration: u64,
    pub targets: Option<Vec<Principal>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct SignedDelegation {
    pub delegation: Delegation,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct PrepareLoginResponse {
    pub message: String,
    pub session_key: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct LoginResponse {
    pub user_canister_pubkey: Vec<u8>,
    pub expiration: u64,
}

pub struct InternetIdentityService {
    agent: Agent,
    auth_client: AuthClient,
}

impl InternetIdentityService {
    pub fn init() {
        let conf = &config::get().icp;
        let auth_client = AuthClient::new(conf.endpoint, Principal::from_text(conf.canister_id).unwrap());
        let agent = Agent::builder()
            .with_url(conf.endpoint)
            .build()
            .expect("failed to build default agent");

        let srv = Self {
            auth_client,
            agent,
        };

        use_context_provider(move || srv);
    }

    pub async fn authorize(&self) -> Result<String> {
        let auth_client = AuthClient::new(self.agent.clone(), self.canister_id).await?;
        let principal = auth_client.login().await?;
        Ok(principal.to_text())
    }

    pub async fn prepare_login(&self, address: String) -> Result<PrepareLoginResponse> {
        self.query("siwe_prepare_login", (address,)).await
    }

    pub async fn login(
        &self,
        signature: String,
        address: String,
        session_key: Vec<u8>,
    ) -> Result<LoginResponse> {
        self.update("siwe_login", (signature, address, session_key)).await
    }

    pub async fn get_delegation(
        &self,
        address: String,
        session_key: Vec<u8>,
        expiration: u64,
    ) -> Result<SignedDelegation> {
        self.query(
            "siwe_get_delegation",
            (address, session_key, expiration),
        ).await
    }

    async fn query<'a, Tuple: ArgumentEncoder, R>(
        &self,
        method: &'a str,
        args: Tuple,
    ) -> Result<R>
    where
        R: 'static + Deserialize<'a> + CandidType + std::fmt::Debug + ToOwned<Owned = R>,
    {
        let inter_output = self
            .agent
            .query(&self.canister_id, method)
            .with_arg(encode_args(args).map_err(|e| Error::CandidError(e.to_string()))?)
            .call()
            .await
            .map_err(|e| Error::AgentError(e.to_string()))?;
        
        candid::Decode!(&inter_output, R).map_err(|e| Error::CandidError(e.to_string()))
    }

    async fn update<'a, Tuple: ArgumentEncoder, R>(
        &self,
        method: &'a str,
        args: Tuple,
    ) -> Result<R>
    where
        R: 'static + Deserialize<'a> + CandidType + std::fmt::Debug + ToOwned<Owned = R>,
    {
        let inter_output = self
            .agent
            .update(&self.canister_id, method)
            .with_arg(encode_args(args).map_err(|e| Error::CandidError(e.to_string()))?)
            .call_and_wait()
            .await
            .map_err(|e| Error::AgentError(e.to_string()))?;
        
        candid::Decode!(&inter_output, R).map_err(|e| Error::CandidError(e.to_string()))
    }
}

struct AuthClient {
    agent: Agent,
    canister_id: Principal,
}

impl AuthClient {
    async fn new(agent: Agent, canister_id: Principal) -> Result<Self> {
        Ok(Self { agent, canister_id })
    }

    async fn login(&self) -> Result<Principal> {
        let identity = BasicIdentity::from_phrase(&[0; 32])?; // Dummy identity for now
        self.agent.set_identity(identity);
        
        
        Ok(self.agent.get_principal()?)
    }
}