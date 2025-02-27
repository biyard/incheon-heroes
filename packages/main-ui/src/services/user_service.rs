#![allow(non_snake_case)]
use async_trait::async_trait;
use std::sync::Arc;

use by_macros::DioxusController;
use dioxus::prelude::*;
use dioxus_oauth::prelude::FirebaseService;
use dto::{
    contracts::klaytn_transaction::KlaytnTransaction,
    wallets::{wallet::KaiaLocalWallet, KaiaWallet},
    User,
};
use ethers::{
    providers::{Http, Provider},
    types::Signature,
};
use gloo_storage::{LocalStorage, Storage};
use ic_agent::{identity::BasicIdentity, Identity};

use crate::{
    config,
    models::{
        nft_metadata::NftMetadata,
        user_wallet::{create_identity, UserWallet},
    },
};

use super::{icp_canister::IcpCanister, klaytn::Klaytn};

const USER_WALLET_KEY: &str = "user_wallet";

unsafe impl Sync for UserService {}
unsafe impl Send for UserService {}

#[derive(Clone, Copy, DioxusController)]
pub struct UserService {
    wallet: Signal<UserWallet>,
    icp_wallet: Signal<Option<Arc<BasicIdentity>>>,
    evm_nfts: Resource<Vec<(u64, NftMetadata)>>,
    sbts: Resource<Vec<(u64, NftMetadata)>>,
    icp_nfts: Resource<Vec<(u64, NftMetadata)>>,
    icp_canister: IcpCanister,
    user: Signal<Option<User>>,
    klaytn: Klaytn,
}

impl UserService {
    pub fn user_id(&self) -> Option<i64> {
        match self.user() {
            Some(user) => Some(user.id),
            None => None,
        }
    }

    pub fn init() {
        let firebase = FirebaseService::new(
            config::get().firebase.api_key.clone(),
            config::get().firebase.auth_domain.clone(),
            config::get().firebase.project_id.clone(),
            config::get().firebase.storage_bucket.clone(),
            config::get().firebase.messaging_sender_id.clone(),
            config::get().firebase.app_id.clone(),
            config::get().firebase.measurement_id.clone(),
        );

        use_context_provider(move || firebase);

        let wallet = use_signal(|| UserWallet::None);
        let mut klaytn: Klaytn = use_context();

        let sbts = use_resource(move || async move {
            let w = wallet();
            let address = match w.evm_address() {
                Some(address) => address,
                None => return vec![],
            };
            let sbt = (klaytn.sbt)();

            match sbt.list_token_ids_by_address(address).await {
                Ok(ids) => {
                    tracing::debug!("Token ids: {}", ids.len());
                    let mut nfts = vec![];
                    for id in ids {
                        let uri = match sbt.get_token_uri(id).await {
                            Ok(uri) => uri,
                            Err(e) => {
                                tracing::error!("Failed to fetch nft({id}) uri: {e:?}");
                                continue;
                            }
                        };

                        tracing::debug!("Token uri: {uri}");
                        match NftMetadata::fetch_by_uri(&uri).await {
                            Ok(nft) => nfts.push((id.as_u64(), nft)),
                            Err(e) => {
                                tracing::error!("Failed to fetch nft({id}) metadata: {e:?}");
                            }
                        }
                    }

                    nfts
                }
                Err(e) => {
                    tracing::error!("Failed to get token ids: {e:?}");
                    vec![]
                }
            }
        });

        let evm_nfts = use_resource(move || async move {
            let w = wallet();
            let address = match w.evm_address() {
                Some(address) => address,
                None => return vec![],
            };

            match (klaytn.holder)().list_token_ids_by_address(address).await {
                Ok(ids) => {
                    let mut nfts = vec![];
                    for id in ids {
                        match NftMetadata::fetch(id.as_u64().try_into().unwrap()).await {
                            Ok(nft) => nfts.push((id.as_u64(), nft)),
                            Err(e) => {
                                tracing::error!("Failed to fetch nft({id}) metadata: {e:?}");
                            }
                        }
                    }

                    nfts
                }
                Err(e) => {
                    tracing::error!("Failed to get token ids: {e:?}");
                    vec![]
                }
            }
        });

        let icp_canister: IcpCanister = use_context();
        let icp_wallet: Signal<Option<Arc<BasicIdentity>>> = use_signal(|| None);

        let icp_nfts = use_resource(move || async move {
            let principal = match icp_wallet() {
                Some(identity) => identity.sender().unwrap().to_text(),
                None => return vec![],
            };

            match icp_canister.list_nfts_by_address(principal).await {
                Ok(nfts) => nfts
                    .into_iter()
                    .map(|nft| (nft.id, NftMetadata::from(nft)))
                    .collect(),
                Err(e) => {
                    tracing::error!("Failed to get token ids: {e:?}");
                    vec![]
                }
            }
        });

        let srv = Self {
            user: use_signal(|| None),
            wallet,
            icp_wallet,
            icp_canister,
            evm_nfts,
            sbts,
            icp_nfts,
            klaytn: use_context(),
        };

        klaytn.set_signer(srv);

        #[cfg(feature = "web")]
        use_effect(move || {
            spawn(async move {
                let mut srv = srv;
                srv.load_wallet_from_storage().await;
            });
        });

        use_context_provider(move || srv);
    }

    pub fn get_evm_nfts(&self) -> Vec<(u64, NftMetadata)> {
        let nfts = match self.evm_nfts.value()() {
            Some(v) => v,
            None => vec![],
        };

        nfts
    }

    pub fn get_icp_nfts(&self) -> Vec<(u64, NftMetadata)> {
        let nfts = match self.icp_nfts.value()() {
            Some(v) => v,
            None => vec![],
        };

        nfts
    }

    pub fn is_logined(&self) -> bool {
        match self.user() {
            None => false,
            _ => true,
        }
    }

    pub async fn load_wallet_from_storage(&mut self) {
        if let Ok(wallet) = LocalStorage::get::<UserWallet>(USER_WALLET_KEY) {
            tracing::debug!("Loaded wallet from storage: {wallet}");
            if let Some(seed_hex) = wallet.seed() {
                let icp_wallet = create_identity(&seed_hex);
                self.icp_wallet.set(Some(Arc::new(icp_wallet)));
            }
            self.wallet.set(wallet);

            let addr = self.evm_address().unwrap();
            let endpoint = config::get().new_api_endpoint;
            match User::get_client(endpoint).get_user_by_address(addr).await {
                Ok(user) => {
                    self.user.set(Some(user));
                }
                Err(e) => {
                    tracing::error!("Failed to get user by address: {e}");
                }
            }
        }
    }

    pub fn set_user(&mut self, user: User) {
        self.user.set(Some(user));
    }

    pub async fn set_wallet(&mut self, wallet: UserWallet) {
        LocalStorage::delete(USER_WALLET_KEY);
        if wallet.can_cached() {
            if let Err(e) = LocalStorage::set(USER_WALLET_KEY, &wallet) {
                tracing::warn!("Failed to save wallet to storage: {e}");
            }
        }

        if let Some(wallet) = wallet.icp_identity() {
            self.icp_canister.agent().set_identity(wallet);
        }

        if let Some(wallet) = wallet.icp_identity() {
            self.icp_wallet.set(Some(Arc::new(wallet)));
        }

        tracing::debug!("Set wallet: {wallet:?}");
        self.wallet.set(wallet);
    }

    pub fn evm_address(&self) -> Option<String> {
        self.wallet().evm_address()
    }

    pub fn icp_address(&self) -> Option<String> {
        self.wallet().principal()
    }
}

#[cfg_attr(not(feature = "server"), async_trait(?Send))]
#[cfg_attr(feature = "server", async_trait)]
impl KaiaWallet for UserService {
    fn address(&self) -> ethers::types::H160 {
        self.evm_address().unwrap_or_default().parse().unwrap()
    }

    async fn sign_transaction(&self, tx: &KlaytnTransaction) -> dto::Result<Signature> {
        tracing::debug!("Sign transaction: {tx:?}");

        match self.wallet() {
            UserWallet::SocialWallet {
                ref private_key, ..
            } => {
                let conf = config::get();
                let provider = Provider::<Http>::try_from(conf.klaytn.endpoint).unwrap();
                let provider: Arc<Provider<Http>> = Arc::new(provider);

                let w = KaiaLocalWallet::new(private_key, provider).await?;
                w.sign_transaction(tx).await
            }
            UserWallet::KaiaWallet(ref wallet) => wallet.sign_transaction(tx).await,
            UserWallet::None => Err(dto::Error::WalletNotInitialized),
        }
    }
}
