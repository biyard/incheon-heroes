#![allow(dead_code)]
use crate::config;
use crate::models::user_wallet::{create_evm_wallet, create_identity, EvmWallet, UserWallet};
use crate::route::Route;
use crate::services::backend_api::BackendApi;
use crate::services::user_service::UserService;
use by_macros::*;
use dioxus::prelude::*;
use dioxus_translate::Language;
use dto::User;
use ethers::utils::keccak256;
use ic_agent::Identity;

use super::models::LoginProvider;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    pub lang: Language,
    pub id: Signal<String>,
    pub provider: LoginProvider,
    pub hint: Signal<String>,
    pub address: Signal<Option<String>>,
    pub password: Signal<String>,
    pub email: Signal<String>,
    pub picture: Signal<String>,
    pub backend_api: BackendApi,
    pub user_wallet: UserService,
    pub nav: Navigator,
}

impl Controller {
    pub fn new(
        lang: Language,
        id: String,
        provider: LoginProvider,
        hint: String,
        address: String,
        email: String,
        picture: String,
    ) -> std::result::Result<Self, RenderError> {
        tracing::debug!("Hint: {:?}", hint);
        let nav = use_navigator();
        let priv_hint = hint.clone();

        use_effect(move || {
            tracing::debug!("Hint: {:?}", priv_hint);

            if priv_hint.is_empty() {
                tracing::error!("hint is empty");
                nav.replace(Route::ConnectPage { lang });
            }
        });

        let ctrl = Self {
            lang,
            provider,
            nav: use_navigator(),
            id: use_signal(move || id),
            hint: use_signal(move || hint),
            address: use_signal(move || {
                if address.is_empty() {
                    None
                } else {
                    Some(address)
                }
            }),
            password: use_signal(|| "".to_string()),
            email: use_signal(move || email),
            picture: use_signal(move || picture),
            backend_api: use_context(),
            user_wallet: use_context(),
        };

        Ok(ctrl)
    }

    pub async fn signup_handler(&self, wallet: &EvmWallet) {
        if let Err(e) = self
            .backend_api
            .notify_address(&self.id(), &wallet.address)
            .await
        {
            tracing::error!("Failed to notify address: {:?}", e);
        }

        // TODO: Implement singup handler
        match self.provider {
            LoginProvider::Kakao => self.backup_kakao().await,
            LoginProvider::Google => self.backup_google().await,
            LoginProvider::Kaia => {}
        }
    }

    pub async fn handle_login(&mut self) {
        let seed = self.create_seed(self.password());
        tracing::debug!("Seed: {:?}", hex::encode(&seed));
        let wallet = create_evm_wallet(&seed).unwrap();
        tracing::debug!("Wallet: {:?}", wallet);

        if let Some(address) = self.address() {
            if address != wallet.address {
                btracing::error!("Check your password");
                return;
            }
        }

        let icp_wallet = create_identity(&wallet.seed);

        if self.address().is_none() {
            self.signup_handler(&wallet).await;
        }

        self.user_wallet
            .set_wallet(UserWallet::SocialWallet {
                private_key: wallet.private_key,
                seed: wallet.seed,
                checksum_address: wallet.checksum_address.clone(),
                principal: icp_wallet.sender().unwrap().to_text(),
            })
            .await;

        let endpoint = config::get().new_api_endpoint;
        match User::get_client(endpoint)
            .signup_or_login(
                wallet.checksum_address,
                self.email(),
                self.id(),
                self.picture(),
                self.provider.into(),
            )
            .await
        {
            Ok(user) => {
                self.user_wallet.set_user(user);
                self.user_wallet.account_activities.restart();
                self.user_wallet.account_exp.restart();
            }
            Err(e) => {
                btracing::error!("Failed to get user: {:?}", e);
            }
        }

        if self.nav.can_go_back() {
            self.nav.go_back();
        } else {
            self.nav.replace(Route::HomePage { lang: self.lang });
        }
    }

    pub fn create_seed(&self, password: String) -> [u8; 32] {
        let h = keccak256(password.as_bytes());
        tracing::debug!("Hash: {:?}", hex::encode(h));
        tracing::debug!("Hint: {:?}", self.hint());
        let hint = hex::decode(self.hint().trim_start_matches("0x")).unwrap();
        let input = [&h[..], &hint[..]].concat();

        tracing::debug!("Input: {:?}", hex::encode(&input));

        keccak256(input)
    }

    pub async fn backup_kakao(&self) {
        // TODO: send a message to kakao
    }

    pub async fn backup_google(&self) {
        // TODO: save the google account
    }
}
