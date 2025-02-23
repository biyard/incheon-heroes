use std::collections::HashSet;

use candid::{CandidType, Principal};
use features::siwe_provider::service::get_address::AddressMap;
use features::siwe_provider::service::init_upgrade::siwe_init;
use ic_cdk::{api, init, post_upgrade, pre_upgrade, println, storage};
use ic_cdk::{export_candid, query};

pub mod types;
pub mod features {
    pub mod api;
    pub mod dao;
    pub mod nft;
    pub mod siwe_provider;
}

pub mod utils {
    pub mod backend_api;
    pub mod klaytn_birdige_api;
}

use dto::dao::*;
use dto::nft::*;
use features::api::*;
use features::dao::*;
use features::nft::*;
use features::siwe_provider::service::init_upgrade::SettingsInput;
use ic_cdk::api::management_canister::http_request::*;
use ic_siwe::delegation::SignedDelegation;
use ic_siwe::login::LoginDetails;
use serde::Deserialize;
use serde_bytes::ByteBuf;
use types::*;

#[derive(CandidType, Deserialize)]
pub struct InitArgs {
    custodians: Option<HashSet<Principal>>,
}

#[init]
fn init(args: InitArgs) {
    features::nft::STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.custodians = args
            .custodians
            .unwrap_or_else(|| HashSet::from_iter([api::caller()]))
            .clone();
    });
}

#[pre_upgrade]
fn pre_upgrade() {
    // NOTE: pre_upgrade will be called the previous version of the canister deployed.
    let v = version();
    println!("pre_upgrade: {}", v);
    let nft_data = features::nft::STATE.with(|state| state.take());

    let dao_data = features::dao::STATE.with(|state| state.take());

    let timer_data = features::dao::TIMERS.with(|state| state.take());

    storage::stable_save((nft_data, dao_data, timer_data)).ok();
}

#[post_upgrade]
fn post_upgrade() {
    println!("post_upgrade: {}", version());
    let data = match storage::stable_restore::<(
        features::nft::State,
        features::dao::State,
        features::dao::TimerState,
    )>() {
        Ok((nft_data, dao_data, timer_data)) => {
            features::nft::STATE.with(|state0| {
                *state0.borrow_mut() = nft_data;
            });

            features::dao::STATE.with(|state0| {
                *state0.borrow_mut() = dao_data;
            });

            // NOTE: set_timer_handle will save the timers into TIMERS STATE of dao
            timer_data
                .timers
                .iter()
                .for_each(|proposal_id| set_timer_handle(*proposal_id, None));
        }
        Err(e) => panic!("Failed to decode {:?}", e),
    };

    siwe_init(SettingsInput {
        domain: env!("DOMAIN").to_string(),
        uri: env!("URI").to_string(),
        salt: env!("SALT").to_string(),
        chain_id: Some(
            option_env!("CHAIN_ID")
                .unwrap_or("8217")
                .to_string()
                .parse::<u32>()
                .unwrap(),
        ),
        scheme: Some(option_env!("SCHEME").unwrap_or("https").to_string()),
        statement: None,
        sign_in_expires_in: Some(
            option_env!("SIGN_IN_EXPIRES_IN")
                .unwrap_or("300000000000")
                .to_string()
                .parse::<u64>()
                .unwrap(),
        ),
        session_expires_in: Some(
            option_env!("SESSION_EXPIRES_IN")
                .unwrap_or("604800000000000")
                .to_string()
                .parse::<u64>()
                .unwrap(),
        ),
        targets: Some(vec![ic_cdk::id().to_string()]),
        runtime_features: None,
    });

    data
}

#[query]
fn version() -> String {
    match option_env!("VERSION") {
        Some(version) => match option_env!("COMMIT") {
            Some(commit) => format!("{}-{}", version, commit),
            None => format!("{}", version),
        },
        None => match option_env!("DATE") {
            Some(date) => date.to_string(),
            None => "unknown".to_string(),
        },
    }
    .to_string()
}

export_candid!();
