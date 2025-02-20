use std::sync::Arc;

use by_macros::DioxusController;
use ethers::providers::Http;
use ethers::providers::Provider;

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = biyard)]
//     pub fn initialize(config: &JsValue) -> JsValue;

//     #[wasm_bindgen(js_namespace = biyard)]
//     pub fn klaytn() -> KlaytnContracts;

// }

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_name = "Object")]
//     pub type KlaytnContracts;

//     #[wasm_bindgen(method, getter)]
//     pub fn shop(this: &KlaytnContracts) -> ShopContract;
// }

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_name = "Object")]
//     pub type ShopContract;

//     #[wasm_bindgen(method, getter)]
//     pub fn methods(this: &ShopContract) -> ShopMethods;
// }

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_name = "Object")]
//     pub type ShopMethods;

//     #[wasm_bindgen(catch, method, js_name = listItems)]
//     pub async fn list_items(this: &ShopMethods, page: u128, size: u128)
//         -> Result<JsValue, JsValue>;
// }

use dioxus::prelude::*;

use crate::config;

use super::shop_contract::ShopContract;

#[derive(Clone, Copy, DioxusController)]
pub struct Klaytn {
    pub shop: Signal<ShopContract>,
}

impl Klaytn {
    pub fn init() {
        let conf = config::get();
        let provider = Provider::<Http>::try_from(conf.klaytn.endpoint).unwrap();
        let provider = Arc::new(provider);
        let shop = ShopContract::new(conf.contracts.shop, provider.clone());
        let srv = Self {
            shop: use_signal(move || shop),
        };

        use_context_provider(move || srv);
    }
}
