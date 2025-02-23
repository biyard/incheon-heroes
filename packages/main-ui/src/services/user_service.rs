#![allow(non_snake_case)]
use std::sync::Arc;

use by_macros::DioxusController;
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use ic_agent::identity::BasicIdentity;

use crate::models::user_wallet::{create_identity, UserWallet};

const USER_WALLET_KEY: &str = "user_wallet";

#[derive(Clone, Copy, DioxusController)]
pub struct UserService {
    wallet: Signal<UserWallet>,
    icp_wallet: Signal<Option<Arc<BasicIdentity>>>,
}

impl UserService {
    pub fn init() {
        let srv = Self {
            wallet: use_signal(|| UserWallet::None),
            icp_wallet: use_signal(|| None),
        };
        #[cfg(feature = "web")]
        use_effect(move || {
            let mut srv = srv;
            srv.load_wallet_from_storage();
        });

        use_context_provider(move || srv);
    }

    pub fn is_logined(&self) -> bool {
        match self.wallet() {
            UserWallet::None => false,
            _ => true,
        }
    }

    pub fn load_wallet_from_storage(&mut self) {
        if let Ok(wallet) = LocalStorage::get::<UserWallet>(USER_WALLET_KEY) {
            tracing::debug!("Loaded wallet from storage: {wallet}");
            if let Some(seed_hex) = wallet.seed() {
                let icp_wallet = create_identity(&seed_hex);
                self.icp_wallet.set(Some(Arc::new(icp_wallet)));
            }
            self.wallet.set(wallet);
        }
    }

    pub fn set_wallet(&mut self, wallet: UserWallet, icp_wallet: BasicIdentity) {
        if let Err(e) = LocalStorage::set(USER_WALLET_KEY, &wallet) {
            tracing::warn!("Failed to save wallet to storage: {e}");
        };

        self.wallet.set(wallet);
        self.icp_wallet.set(Some(Arc::new(icp_wallet)));
    }

    pub fn evm_address(&self) -> Option<String> {
        match self.wallet() {
            UserWallet::SocialWallet {
                checksum_address, ..
            } => Some(checksum_address),
            UserWallet::None => None,
        }
    }

    pub fn icp_address(&self) -> Option<String> {
        match self.wallet() {
            UserWallet::SocialWallet { principal, .. } => Some(principal),
            UserWallet::None => None,
        }
    }
}
