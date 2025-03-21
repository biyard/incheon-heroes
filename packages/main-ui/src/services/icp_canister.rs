#![allow(non_snake_case)]

use by_macros::DioxusController;
use dioxus::prelude::*;

use candid::{CandidType, Decode, Principal, encode_args, utils::ArgumentEncoder};
use dto::*;
use ic_agent::Agent;
use nft::Nft;
use serde::Deserialize;

use crate::config;

#[derive(Clone, Copy, DioxusController)]
pub struct IcpCanister {
    pub agent: Signal<Agent>,
    pub canister_id: Principal,
}

impl IcpCanister {
    pub fn init() {
        let conf = &config::get().icp;
        let agent = Agent::builder()
            .with_url(conf.endpoint)
            .build()
            .expect("failed to build default agent");

        // if cfg!(not(prod)) {
        //     agent.fetch_root_key().await.unwrap();
        // }
        let srv = Self {
            canister_id: candid::Principal::from_text(conf.canister_id)
                .expect("invalid canister id"),
            agent: use_signal(move || agent),
        };

        use_context_provider(move || srv);
    }

    pub async fn list_nfts_by_address(&self, principal: String) -> Result<Vec<Nft>> {
        let principal =
            Principal::from_text(&principal).map_err(|e| Error::PrincipalError(e.to_string()))?;
        self.query("list_tokens_by", (principal,)).await
    }

    pub async fn bridge(&self, id: u64, address: String) -> Result<Nft> {
        self.update("bridge", (id, address, "".to_string())).await
    }

    pub async fn update<'a, Tuple: ArgumentEncoder, R>(
        &self,
        method: &'a str,
        args: Tuple,
    ) -> Result<R>
    where
        R: 'static + Deserialize<'a> + CandidType + std::fmt::Debug + ToOwned<Owned = R>,
    {
        let inter_output = self
            .agent()
            .update(&self.canister_id, method.to_string())
            .with_arg(encode_args(args).map_err(|e| Error::CandidError(e.to_string()))?)
            .call_and_wait()
            .await
            .map_err(|e| Error::AgentError(e.to_string()))?;
        let inter_output = Box::leak(Box::new(inter_output));

        candid::Decode!(inter_output, R).map_err(|e| Error::CandidError(e.to_string()))
    }

    pub async fn query<'a, Tuple: ArgumentEncoder, R>(
        &self,
        method: &'a str,
        args: Tuple,
    ) -> Result<R>
    where
        R: 'static + Deserialize<'a> + CandidType + std::fmt::Debug + ToOwned<Owned = R>,
    {
        let inter_output = self
            .agent()
            .query(&self.canister_id, method.to_string())
            .with_arg(encode_args(args).map_err(|e| Error::CandidError(e.to_string()))?)
            .call()
            .await
            .map_err(|e| Error::AgentError(e.to_string()))?;
        let inter_output = Box::leak(Box::new(inter_output));

        candid::Decode!(inter_output, R).map_err(|e| Error::CandidError(e.to_string()))
    }

    pub async fn register_evm_address(&self, address: String) -> Result<()> {
        self.update("register_evm_address", (address,)).await
    }
}
