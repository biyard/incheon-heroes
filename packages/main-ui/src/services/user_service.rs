#![allow(non_snake_case)]
use by_macros::DioxusController;
use dioxus::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum UserWallet {
    SocialWallet {
        private_key: String,
        seed: String,
        checksum_address: String,
    },
    #[default]
    None,
}

#[derive(Clone, Copy, DioxusController)]
pub struct UserService {
    pub wallet: Signal<UserWallet>,
}

impl UserService {
    pub fn init() {
        let srv = Self {
            wallet: use_signal(|| UserWallet::None),
        };
        use_context_provider(move || srv);
    }

    pub fn is_logined(&self) -> bool {
        match self.wallet() {
            UserWallet::None => false,
            _ => true,
        }
    }
}
