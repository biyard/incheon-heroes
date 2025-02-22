#![allow(non_snake_case)]
use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn MyProfilePage(lang: Language) -> Element {
    let mut _ctrl = Controller::new()?;
    let tr: MyProfileTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: "{tr.title}" }

        div { id: "my-page", "{tr.title} PAGE" } // end of this page
    }
}
