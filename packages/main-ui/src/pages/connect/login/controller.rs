#![allow(dead_code)]
use by_macros::*;
use dioxus::prelude::*;
use dioxus_translate::Language;

use crate::services::backend_api::AccountHint;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    pub lang: Language,
    pub hint: Signal<AccountHint>,
}

impl Controller {
    pub fn new(lang: Language, hint: AccountHint) -> std::result::Result<Self, RenderError> {
        let ctrl = Self {
            lang,
            hint: use_signal(move || hint),
        };

        Ok(ctrl)
    }

    pub async fn handle_login(&self) {}
}
