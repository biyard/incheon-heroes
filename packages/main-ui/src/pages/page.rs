#![allow(non_snake_case)]
use crate::components::headings::Heading1;

use super::i18n::*;
use crate::assets::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn HomePage(lang: Language) -> Element {
    let tr: MainTextTranslate = translate(&lang);

    rsx! {
        div { id: "home-page", class: "flex flex-col items-center gap-[45px]",
            VideoSection {}

            Heading1 { with_symbol: false, "INCHEON UNIVERSE" }
            p { class: "text-[16px] font-bold text-center", "{tr.main_text}" }
            LoginButton { lang }
            LeaderBoard { lang }
        }
    }
}

#[component]
pub fn LeaderBoard(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    lang: Language,
) -> Element {
    let tr: LeaderBoardTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col items-center gap-[30px]", ..attributes,
            Heading1 { "{tr.title}" }
            div { class: "w-full flex flex-col items-end gap-[5px] px-[20px] py-[10px] rounded-[12px]" }


        }
    }
}

#[component]
pub fn VideoSection() -> Element {
    rsx! {
        div { id: "videosection", class: "flex justify-center items-center p-4",
            video {
                class: "w-full max-w-6xl",
                autoplay: "true",
                r#loop: "true",
                muted: "true",
                src: "{VIDEO}",
            }
        }
    }
}

#[component]
pub fn LoginButton(
    #[props(default ="login_button".to_string())] id: String,
    #[props(default ="".to_string())] class: String,

    lang: Language,
) -> Element {
    let tr: LoginButtonTranslate = translate(&lang);

    rsx! {
        div { class: "flex justify-center items-center p-4",
            button {
                onclick: |_| println!("Button clicked!"),
                class: "px-4 py-2 bg-gray-500 text-white rounded-full hover:bg-gray-600 text-xl font-bold",
                style: "width: 250px; height: 60px; padding-10 px",
                "{tr.button_text}"
            }
        }
    }
}
