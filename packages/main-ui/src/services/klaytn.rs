use std::sync::Arc;

use by_macros::DioxusController;
use ethers::providers::Http;
use ethers::providers::Provider;

use dioxus::prelude::*;

use crate::config;

use super::holder_contract::HolderContract;
use super::sbt_contract::SbtContract;
use super::shop_contract::ShopContract;

#[derive(Clone, Copy, DioxusController)]
pub struct Klaytn {
    pub shop: Signal<ShopContract>,
    pub holder: Signal<HolderContract>,
    pub sbt: Signal<SbtContract>,
}

impl Klaytn {
    pub fn init() {
        let conf = config::get();
        let provider = Provider::<Http>::try_from(conf.klaytn.endpoint).unwrap();
        let provider = Arc::new(provider);
        let shop = ShopContract::new(conf.contracts.shop, provider.clone());
        let holder = HolderContract::new(conf.contracts.holder, provider.clone());
        let sbt = SbtContract::new(conf.contracts.sbt, provider.clone());
        let srv = Self {
            shop: use_signal(move || shop),
            holder: use_signal(move || holder),
            sbt: use_signal(move || sbt),
        };

        use_context_provider(move || srv);
    }
}
