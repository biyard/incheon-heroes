#![allow(non_snake_case)]
use super::{template::Footer, LoginButton, Maintext, VideoSection};
use dioxus::prelude::*;
use dioxus_translate::Language;

#[component]
pub fn HomePage(lang: Language) -> Element {
    rsx! {
        div { id: "home-page", class: "flex flex-col min-h-screen",
            VideoSection {}
            Maintext { lang }
            LoginButton { lang }
            Footer { lang }
        }
    }
}
