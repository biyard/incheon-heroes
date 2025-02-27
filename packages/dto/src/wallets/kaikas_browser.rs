use crate::Result;
use wasm_bindgen::prelude::*;
use web_sys::js_sys::{Function, Promise};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(thread_local_v2, js_name = klaytn)]
    static KLAYTN: JsValue;
}

// https://docs.kaiawallet.io/api_reference/klaytn_provider#klaytnrequestoptions
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "Object")]
    pub type Klaytn;

    #[wasm_bindgen(method)]
    pub fn enable(this: &Klaytn) -> Promise;

    #[wasm_bindgen(method, getter = selectedAddress)]
    pub fn selected_address(this: &Klaytn) -> Option<String>;

    #[wasm_bindgen(method)]
    pub fn on(this: &Klaytn, event_name: &str, callback: &Function) -> Promise;

    #[wasm_bindgen(method)]
    pub fn request(this: &Klaytn, option: &JsValue) -> Promise;
}

pub fn klaytn() -> Result<Klaytn> {
    let window =
        web_sys::window().ok_or(crate::Error::Unknown("failed to get window".to_string()))?;
    match window.get("klaytn") {
        Some(k) => Ok(k.dyn_into().unwrap()),
        _ => Err(crate::Error::NoKaikasWallet),
    }
}
