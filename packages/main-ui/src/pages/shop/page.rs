#![allow(non_snake_case)]
use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn ShopPage(lang: Language) -> Element {
    let mut _ctrl = Controller::new()?;
    let tr: ShopTranslate = translate(&lang);

    rsx! {
        div { id: "shop", "{tr.title} PAGE" }
    }
}
