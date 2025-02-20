use by_macros::DioxusController;
use dioxus::prelude::*;

use crate::services::{klaytn::Klaytn, shop_contract::ShopItem};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    pub items: Resource<Vec<ShopItem>>,
    pub klaytn: Klaytn,
}

impl Controller {
    pub fn new() -> std::result::Result<Self, RenderError> {
        let klaytn: Klaytn = use_context();

        let items = use_server_future(move || async move {
            match klaytn.shop().list_items(0, 1000).await {
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
        let ctrl = Self { items, klaytn };
        use_context_provider(|| ctrl);

        Ok(ctrl)
    }

    pub fn handle_buy(&self, i: usize) {
        let item_id = self.items().unwrap()[i].id;
        tracing::debug!("buying item: {:?}", item_id);
        // TODO: implement buying shop item
    }

    pub fn handle_like(&self, i: usize) {
        let item_id = self.items().unwrap()[i].id;
        tracing::debug!("liking item: {:?}", item_id);
        // TODO: implement liking shop item
    }
}
