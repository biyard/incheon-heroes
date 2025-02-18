#![allow(non_snake_case)]
use crate::components::headings::Heading1;

use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn NoticesPage(lang: Language) -> Element {
    let tr: NoticesTranslate = translate(&lang);
    let tr = tr.clone();
    let notices = vec![
        (tr.notice_season2_title, tr.notice_season2_detail),
        (tr.notice_song_title, tr.notice_song_detail),
    ];

    rsx! {
        div {
            id: "notices",
            class: "flex flex-col gap-[50px] items-center justify-center",
            Heading1 { lang, "{tr.title}" }
            div { class: "w-full max-w-[700px] flex flex-col gap-[20px] items-center justify-center overflow-hidden",
                for (title , detail) in notices {
                    Notice { title: "{title}", details: "{detail}", lang }
                }
            }
        }
    }
}

#[component]
pub fn Notice(title: String, details: String, lang: Language) -> Element {
    let mut clicked = use_signal(|| false);
    let tr: NoticeTranslate = translate(&lang);

    rsx! {
        div { class: "w-full flex flex-col gap-0 rounded-[10px] overflow-hidden items-center justify-center",
            div { class: "w-full flex items-center justify-center  bg-[#32C564] text-[20px] font-semibold py-[8.5px]",
                p { "{title}" }
            }
            if !clicked() {
                div { class: "transition-all w-full flex flex py-[10px] bg-[#FAFAFA] items-center justify-center",
                    button {
                        class: "text-[16px] font-semibold text-[#666666]",
                        onclick: move |_| clicked.set(true),
                        "{tr.btn}"
                    }
                }
            } else {
                div {
                    class: "transition-all w-full flex flex py-[10px] bg-[#FAFAFA] items-center justify-center",
                    dangerous_inner_html: "{details}",
                }
            }

        }
    }
}
