use by_macros::DioxusController;
use dioxus::prelude::*;

use crate::services::{klaytn::Klaytn, shop_contract::ShopItem};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    pub item: Resource<ShopItem>,
}

impl Controller {
    pub fn new(id: String) -> std::result::Result<Self, RenderError> {
        let klaytn: Klaytn = use_context();

        let item = use_server_future(move || {
            let id = id.clone();
            async move {
                klaytn
                    .shop()
                    .get_item(id.clone().parse().unwrap())
                    .await
                    .unwrap()
            }
        })?;
        let ctrl = Self { item };

        Ok(ctrl)
    }
}
