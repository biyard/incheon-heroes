#![allow(non_snake_case)]
use super::controller::*;
use super::i18n::*;
#[allow(unused_imports)]
use by_components::icons;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn ContentsByIdPage(id: i64, lang: Language) -> Element {
    let mut _ctrl = Controller::new(lang)?;
    let tr: ContentsByIdTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: "{tr.title}" }

        div { id: "contents-by-id", "{tr.title} PAGE" } // end of this page
    }
}
