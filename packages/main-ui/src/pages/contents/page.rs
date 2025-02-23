#![allow(non_snake_case)]
use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn ContentsPage(lang: Language) -> Element {
    let mut _ctrl = Controller::new(lang)?;
    let tr: ContentsTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: "{tr.title}" }

        div { id: "contents", "{tr.title} PAGE" } // end of this page
    }
}
