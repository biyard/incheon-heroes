#![allow(dead_code)]
use crate::route::Route;
use crate::services::backend_api::BackendApi;
use crate::services::user_service::{UserService, UserWallet};
use by_macros::*;
use dioxus::prelude::*;
use dioxus_translate::Language;
use ethers::prelude::*;
use ethers::utils::{keccak256, to_checksum};

use super::models::LoginProvider;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    pub lang: Language,
    pub id: Signal<String>,
    pub provider: LoginProvider,
    pub hint: Signal<String>,
    pub address: Signal<Option<String>>,
    pub password: Signal<String>,
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
            backend_api: use_context(),
            user_wallet: use_context(),
        };

        Ok(ctrl)
    }

    pub async fn signup_handler(&self, wallet: &Wallet) {
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
        }
    }

    pub async fn handle_login(&mut self) {
        let seed = self.create_seed(self.password());
        tracing::debug!("Seed: {:?}", hex::encode(&seed));
        let wallet = create_wallet_from_seed(&seed).unwrap();
        tracing::debug!("Wallet: {:?}", wallet);

        if let Some(address) = self.address() {
            if address != wallet.address {
                tracing::error!("Address mismatch {} != {}", address, wallet.address);
                return;
            }
        }

        if self.address().is_none() {
            self.signup_handler(&wallet).await;
        }

        self.user_wallet.wallet.set(UserWallet::SocialWallet {
            private_key: wallet.private_key,
            seed: wallet.seed,
            checksum_address: wallet.checksum_address,
        });

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

#[derive(Debug)]
pub struct Wallet {
    pub private_key: String,
    pub seed: String,
    pub checksum_address: String,
    pub address: String,
}

/// Creates a wallet from a seed by performing BIP32 derivation and then computing
/// the associated ECDSA key pair and Ethereum address. The derivation path used is "m/44'/60'/0'/0/0".
pub fn create_wallet_from_seed(seed: &[u8]) -> Result<Wallet, Box<dyn std::error::Error>> {
    use bip32::{DerivationPath, XPrv};
    use std::str::FromStr;

    // Create a BIP32 root key from the seed.
    // Derive the child key using the Ethereum derivation path.
    let derivation_path = DerivationPath::from_str("m/44'/60'/0'/0/0")?;
    let child_xprv = XPrv::derive_from_path(seed, &derivation_path)?;

    // Get the 32-byte private key.
    let private_key = hex::encode(child_xprv.private_key().to_bytes());
    let wallet: LocalWallet = private_key.parse().unwrap();

    let address = wallet.address();
    let checksum_address = to_checksum(&address, None);

    Ok(Wallet {
        private_key,
        seed: format!("0x{}", hex::encode(seed)),
        address: checksum_address.to_lowercase(),
        checksum_address,
    })
}
