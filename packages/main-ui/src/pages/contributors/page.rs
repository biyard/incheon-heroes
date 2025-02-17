#![allow(non_snake_case)]
use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn ContributorsPage(lang: Language) -> Element {
    let mut _ctrl = Controller::new()?;
    let tr: ContributorsTranslate = translate(&lang);

    rsx! {
        div { id: "contributors", "{tr.title} PAGE" }
    }
}
