#![allow(non_snake_case)]
use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn ShopByIdPage(id: String, lang: Language) -> Element {
    let mut _ctrl = Controller::new()?;
    let tr: ShopByIdTranslate = translate(&lang);

    rsx! {
        div { id: "shop-by-id", "{tr.title} PAGE" }
    }
}
