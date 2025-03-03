#![allow(non_snake_case)]
use by_macros::DioxusController;
use dioxus::prelude::*;

#[derive(Clone, Copy, DioxusController)]
pub struct GoogleService {
    pub access_token: Signal<String>,
}

impl GoogleService {
    pub fn init() {
        let srv = Self {
            access_token: use_signal(|| "".to_string()),
        };
        use_context_provider(move || srv);
    }

    pub fn logged_in(&self) -> bool {
        !self.access_token().is_empty()
    }
}
