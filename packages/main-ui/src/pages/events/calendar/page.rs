#![allow(non_snake_case)]
use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn CalendarPage(lang: Language) -> Element {
    let mut _ctrl = Controller::new()?;
    let tr: CalendarTranslate = translate(&lang);

    rsx! {
        div { id: "calendar", "{tr.title} PAGE" }
    }
}
