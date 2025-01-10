#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::Language;

#[component]
pub fn HomePage(lang: Language) -> Element {
    rsx! {
        div { id: "home-page", "HomePage page" }
    }
}
