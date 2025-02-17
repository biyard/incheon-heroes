#![allow(non_snake_case)]
use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn FaqPage(lang: Language) -> Element {
    let mut _ctrl = Controller::new()?;
    let tr: FaqTranslate = translate(&lang);

    rsx! {
        div { id: "faq", "{tr.title} PAGE" }
    }
}
