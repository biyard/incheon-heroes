#![allow(non_snake_case)]
use std::sync::Arc;

use by_macros::DioxusController;
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use ic_agent::identity::BasicIdentity;

use crate::models::{
    nft_metadata::NftMetadata,
    user_wallet::{create_identity, UserWallet},
};

use super::klaytn::Klaytn;

const USER_WALLET_KEY: &str = "user_wallet";

#[derive(Clone, Copy, DioxusController)]
pub struct UserService {
    wallet: Signal<UserWallet>,
    icp_wallet: Signal<Option<Arc<BasicIdentity>>>,
    evm_nfts: Resource<Vec<NftMetadata>>,
    sbts: Resource<Vec<NftMetadata>>,
}

impl UserService {
    pub fn init() {
        let wallet = use_signal(|| UserWallet::None);
        let icp_wallet = use_signal(|| None);
        let klaytn: Klaytn = use_context();

        let sbts = use_resource(move || async move {
            let w = wallet();
            let address = match w.evm_address() {
                Some(address) => address,
                None => return vec![],
            };
            let sbt = klaytn.sbt();

            match sbt.list_token_ids_by_address(address).await {
                Ok(ids) => {
                    let mut nfts = vec![];
                    for id in ids {
                        let uri = match sbt.get_token_uri(id).await {
                            Ok(uri) => uri,
                            Err(e) => {
                                tracing::error!("Failed to fetch nft({id}) uri: {e:?}");
                                continue;
                            }
                        };

                        match NftMetadata::fetch_by_uri(&uri).await {
                            Ok(nft) => nfts.push(nft),
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

            match klaytn.holder().list_token_ids_by_address(address).await {
                Ok(ids) => {
                    let mut nfts = vec![];
                    for id in ids {
                        match NftMetadata::fetch(id.as_u64().try_into().unwrap()).await {
                            Ok(nft) => nfts.push(nft),
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

        // let _icp_nfts = use_resource(move || async move {
        //     let _w = icp_wallet();
        // });

        let srv = Self {
            wallet,
            icp_wallet,
            evm_nfts,
            sbts,
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
