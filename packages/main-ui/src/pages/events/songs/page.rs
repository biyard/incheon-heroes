#![allow(non_snake_case)]
use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn SongsPage(lang: Language) -> Element {
    let mut _ctrl = Controller::new()?;
    let tr: SongsTranslate = translate(&lang);

    rsx! {
        div { id: "songs", "{tr.title} PAGE" }
    }
}
