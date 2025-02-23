#![allow(non_snake_case)]
use by_macros::DioxusController;
use dioxus::prelude::*;

use candid::{encode_args, utils::ArgumentEncoder, CandidType, Decode, Principal};
use ic_agent::{Agent, Identity};
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

    pub fn set_identity<I>(&mut self, id: I)
    where
        I: 'static + Identity,
    {
        self.agent().set_identity(id);
    }

    pub async fn update<'a, Tuple: ArgumentEncoder, I, R>(
        &self,
        method: &'a str,
        args: Tuple,
        inter_output: &'a mut Vec<u8>,
    ) -> Result<R, String>
    where
        R: 'static + Deserialize<'a> + CandidType + std::fmt::Debug + ToOwned<Owned = R>,
    {
        *inter_output = match self
            .agent()
            .update(&self.canister_id, method.to_string())
            .with_arg(match encode_args(args) {
                Ok(args) => args,
                Err(e) => {
                    return Err(e.to_string());
                }
            })
            .call_and_wait()
            .await
        {
            Ok(blob) => blob,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        match candid::Decode!(inter_output, R) {
            Ok(obj) => Ok(obj),
            Err(e) => return Err(e.to_string()),
        }
    }

    pub async fn query<'a, Tuple: ArgumentEncoder, I, R>(
        &self,
        method: &'a str,
        args: Tuple,
        inter_output: &'a mut Vec<u8>,
    ) -> Result<R, String>
    where
        R: 'static + Deserialize<'a> + CandidType + std::fmt::Debug + ToOwned<Owned = R>,
    {
        *inter_output = match self
            .agent()
            .query(&self.canister_id, method.to_string())
            .with_arg(match encode_args(args) {
                Ok(args) => args,
                Err(e) => {
                    return Err(e.to_string());
                }
            })
            .call()
            .await
        {
            Ok(blob) => blob,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        match candid::Decode!(inter_output, R) {
            Ok(obj) => Ok(obj),
            Err(e) => return Err(e.to_string()),
        }
    }
}
