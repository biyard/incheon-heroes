#![allow(non_snake_case)]
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn NoticesPage(lang: Language) -> Element {
    let tr: NoticesTranslate = translate(&lang);
    rsx! {
        div { id: "notices", "{tr.title} page" }
    }
}
