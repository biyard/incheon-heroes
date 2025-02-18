#![allow(non_snake_case)]
use crate::components::headings::Heading1;
use crate::pages::RankingBoards;

use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn ContributorsPage(lang: Language) -> Element {
    let mut ctrl = Controller::new()?;
    let tr: ContributorsTranslate = translate(&lang);

    rsx! {
        div {
            id: "contributors",
            class: "w-full flex flex-col items-center justify-center gap-[50px]",
            Heading1 { lang, "{tr.title}" }
            div { class: "w-full max-w-[730px] flex flex-col gap-[20px] items-center justify-center",
                ExpandableContainer {
                    lang,
                    title: "{tr.ss2_title}",
                    description: "{tr.ss2_desc}",
                    RankingBoards {
                        lang,
                        onchange: move |t| ctrl.handle_select_s2_type(t),
                        data: (ctrl.s2_data)(),
                    }
                }
                ExpandableContainer {
                    lang,
                    title: "{tr.song_title}",
                    description: "{tr.song_desc}",
                }
                ExpandableContainer {
                    lang,
                    title: "{tr.ss1_title}",
                    description: "{tr.ss1_desc}",
                    RankingBoards {
                        lang,
                        onchange: move |t| ctrl.handle_select_s1_type(t),
                        data: ctrl.s1_data(),
                    }
                }
                ExpandableContainer {
                    lang,
                    title: "{tr.holder_title}",
                    description: "{tr.holder_desc}",
                }
            }
        }
    }
}

#[component]
pub fn ExpandableContainer(
    lang: Language,
    title: String,
    description: String,
    children: Element,
) -> Element {
    let badge = asset!("/public/images/badge.png");
    let mut expanded = use_signal(|| false);
    let tr: ExpandableContainerTranslate = translate(&lang);

    rsx! {
        div { class: "transition-all w-full flex flex-col gap-[12px] px-[20px] py-[15px] items-center justify-center bg-[#FAFAFA]/60 rounded-[12px] text-[#666666]",
            div { class: "w-full flex items-center justify-center text-[20px] font-semibold py-[8.5px]",
                img { src: "{badge}" }
                p { class: "text-[#16775D] text-[16px] font-bold", "{title}" }
            }
            if expanded() {
                div {
                    class: "text-[15px] font-semibold",
                    dangerous_inner_html: "{description}",
                }
                div { class: "w-full flex flex-col gap-[20px] items-center justify-center overflow-hidden",
                    {children}
                }
            } else {
                div { class: "w-full flex flex py-[10px] cursor-pointer items-center justify-center",
                    button {
                        class: "text-[16px] font-semibold text-[#666666]",
                        onclick: move |_| expanded.set(true),
                        "{tr.btn}"
                    }
                }
            }
        }
    }
}
