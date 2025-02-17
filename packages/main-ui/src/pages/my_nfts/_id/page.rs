#![allow(non_snake_case)]
use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn MyNftsByIdPage(id: String, lang: Language) -> Element {
    let mut _ctrl = Controller::new()?;
    let tr: MyNftsByIdTranslate = translate(&lang);

    rsx! {
        div { id: "my-nfts-by-id", "{tr.title} PAGE" }
    }
}
