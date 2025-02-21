#![allow(non_snake_case)]
use by_macros::DioxusController;
use dioxus::prelude::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UserService {}

impl UserService {
    pub fn init() {
        let srv = Self {};
        use_context_provider(move || srv);
    }
}
