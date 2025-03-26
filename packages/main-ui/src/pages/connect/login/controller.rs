#![allow(dead_code)]
use crate::config;
use crate::models::user_wallet::{EvmWallet, UserWallet, create_evm_wallet, create_identity};
use crate::route::Route;
use crate::services::backend_api::BackendApi;
use crate::services::google_service::GoogleService;
use crate::services::kakao_service::KakaoService;
use crate::services::user_service::UserService;
use by_macros::*;
use dioxus::prelude::*;
use dioxus_translate::Language;
use dto::{User, UserResponse};
use ethers::utils::keccak256;
use google_wallet::drive_api::DriveApi;
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
    pub google: GoogleService,
    pub kakao: KakaoService,
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
        let nav = use_navigator();

        let ctrl = Self {
            lang,
            nav,
            provider,
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
            google: use_context(),
            kakao: use_context(),
        };

        use_effect(move || {
            let logged_in = match provider {
                LoginProvider::Google => ctrl.google.logged_in(),
                LoginProvider::Kakao => ctrl.kakao.logged_in(),
                LoginProvider::Kaia => true,
            };

            if !logged_in {
                nav.replace(Route::ConnectPage { lang });
            }
        });

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

        let address = wallet.checksum_address.clone();
        let seed = wallet.seed.clone();

        match self.provider {
            LoginProvider::Kakao => self.backup_kakao(seed).await,
            LoginProvider::Google => self.backup_google(address, seed).await,
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

        let endpoint = config::get().new_api_endpoint;
        match User::get_client(endpoint)
            .signup_or_login(
                wallet.checksum_address.clone(),
                self.email(),
                self.id(),
                self.picture(),
                self.provider.into(),
            )
            .await
        {
            Ok(UserResponse { user, action }) => {
                self.user_wallet.set_user(user);

                let _ = action;

                // NOTE: Usually, google login does not be inflowed this.
                //       Inflowing google login in here indicates google drive storage is not available or crashed.
                if action == dto::UserResponseType::SignUp || self.provider == LoginProvider::Google
                {
                    self.signup_handler(&wallet).await;
                }

                self.user_wallet
                    .set_wallet(UserWallet::SocialWallet {
                        private_key: wallet.private_key,
                        seed: wallet.seed,
                        checksum_address: wallet.checksum_address,
                        principal: icp_wallet.sender().unwrap().to_text(),
                    })
                    .await;
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

    pub async fn backup_kakao(&self, seed: String) {
        if let Err(e) = self.kakao.send_message(&self.id(), &seed).await {
            btracing::error!("Failed to send message: {:?}", e)
        }
    }

    pub async fn backup_google(&self, address: String, seed: String) {
        let cli = DriveApi::new(self.google.access_token());

        let content = format!("{}:{}", address, seed);
        match cli.upload_file(&content).await {
            Ok(_) => {
                tracing::debug!("Backup success");
            }
            Err(e) => {
                btracing::error!("Failed to save backup key: {}", e);
            }
        }
    }
}
