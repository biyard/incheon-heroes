#![allow(non_snake_case)]
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn StoriesPage(lang: Language) -> Element {
    let tr: StoriesTranslate = translate(&lang);

    rsx! {
        div { id: "stories", " page" }
    }
}
