#![allow(non_snake_case)]
use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn MyNftsPage(lang: Language) -> Element {
    let mut _ctrl = Controller::new()?;
    let tr: MyNftsTranslate = translate(&lang);

    rsx! {
        div { id: "my-nfts", "{tr.title} PAGE" }
    }
}
