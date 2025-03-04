use by_macros::DioxusController;
use dioxus::prelude::*;

use crate::services::{klaytn::Klaytn, shop_contract::ShopItem, user_service::UserService};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    pub items: Resource<Vec<ShopItem>>,
    pub liked_items: Resource<Vec<u64>>,
    #[allow(dead_code)]
    pub klaytn: Klaytn,
    #[allow(dead_code)]
    pub user: UserService,
    pub account_exp: Resource<u64>,
}

impl Controller {
    pub fn new() -> std::result::Result<Self, RenderError> {
        let klaytn: Klaytn = use_context();
        let user: UserService = use_context();

        let items = use_server_future(move || async move {
            match (klaytn.shop)().list_items(0, 1000).await {
                Ok(res) => {
                    tracing::debug!("{:?}", res);
                    res
                }
                Err(e) => {
                    tracing::error!("{:?}", e);
                    vec![]
                }
            }
        })?;

        let account_exp = use_resource(move || async move {
            let w = user.wallet();
            let address = match w.evm_address() {
                Some(address) => address,
                None => return 0,
            };

            match (klaytn.account)().get_account_exp(address).await {
                Ok(exp) => exp.as_u64(),
                Err(e) => {
                    tracing::error!("Failed to get exp: {e:?}");
                    0
                }
            }
        });

        let liked_items = use_server_future(move || {
            let address = user.evm_address();

            async move {
                if address.is_none() {
                    return vec![];
                }
                let address = address.unwrap();
                match (klaytn.shop)().list_likes_by_address(&address).await {
                    Ok(res) => {
                        tracing::debug!("{:?}", res);
                        res.into_iter().map(|v| v.as_u64()).collect()
                    }
                    Err(e) => {
                        tracing::error!("{:?}", e);
                        vec![]
                    }
                }
            }
        })?;
        let ctrl = Self {
            items,
            klaytn,
            account_exp,
            liked_items,
            user,
        };
        use_context_provider(|| ctrl);

        Ok(ctrl)
    }

    pub async fn handle_buy(&self, i: usize) {
        let klaytn: Klaytn = use_context();

        let shop = klaytn.shop.cloned();
        let account_exp = self.account_exp().unwrap_or_default();
        let item = (self.items)().unwrap()[i].clone();

        if account_exp < item.price.as_u64() {
            btracing::error!("not enough exp");
            return;
        }
        let item_id = item.id;

        btracing::debug!("buying item: {:?}", item_id);

        match shop.buy_item(item_id).await {
            Ok(v) => {
                btracing::debug!("transaction tx: {v}");
            }
            Err(e) => {
                btracing::error!("send transaction failed: {e:?}");
            }
        }
    }

    pub async fn handle_like(&self, i: usize) {
        let klaytn: Klaytn = use_context();
        let shop = klaytn.shop.cloned();

        let item_id = self.items().unwrap()[i].id;
        btracing::debug!("liking item: {:?}", item_id);

        match shop.like_item(item_id).await {
            Ok(v) => {
                btracing::debug!("transaction tx: {v}");
            }
            Err(e) => {
                btracing::error!("send transaction failed: {e}");
            }
        }
    }
}
