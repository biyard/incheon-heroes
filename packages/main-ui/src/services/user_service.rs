#![allow(non_snake_case)]
use std::sync::Arc;

use by_macros::DioxusController;
use dioxus::prelude::*;
use dto::User;
use gloo_storage::{LocalStorage, Storage};
use ic_agent::{identity::BasicIdentity, Identity};

use crate::{
    config,
    models::{
        nft_metadata::NftMetadata,
        user_wallet::{self, create_identity, UserWallet},
    },
};

use super::{icp_canister::IcpCanister, klaytn::Klaytn};

const USER_WALLET_KEY: &str = "user_wallet";

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

    pub async fn logout(&mut self) {
        LocalStorage::delete("user_wallet");
    }

    pub fn init() {
        let wallet = use_signal(|| UserWallet::None);
        let klaytn: Klaytn = use_context();

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
                    self.set_contract_config().await;
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
        if let Err(e) = LocalStorage::set(USER_WALLET_KEY, &wallet) {
            tracing::warn!("Failed to save wallet to storage: {e}");
        };

        if let Some(wallet) = wallet.icp_identity() {
            self.icp_canister.agent().set_identity(wallet);
        }

        if let Some(wallet) = wallet.icp_identity() {
            self.icp_wallet.set(Some(Arc::new(wallet)));
        }

        self.wallet.set(wallet);
        self.set_contract_config().await;
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

    async fn set_contract_config(&mut self) {
        match self.wallet() {
            UserWallet::SocialWallet {
                ref private_key, ..
            } => {
                self.klaytn.set_wallet_provider(private_key).await;
            }
            _ => {}
        }
    }
}
