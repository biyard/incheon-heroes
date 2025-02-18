#![allow(non_snake_case)]
use crate::components::headings::Heading1;
use crate::components::icons::arrows::LeftArrow;
use crate::components::icons::arrows::RightArrow;

use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn SongsPage(lang: Language) -> Element {
    let mut ctrl = Controller::new()?;
    let tr: SongsTranslate = translate(&lang);
    let page = ctrl.page();
    let size = 12;
    let songs = ctrl.songs();

    let start = page * size;
    let mut end = start + size;
    if end > songs.len() {
        end = songs.len();
    }

    rsx! {
        div {
            id: "songs",
            class: "w-full flex flex-col gap-[30px] items-center justify-center",
            Heading1 { lang, "{tr.title}" }
            div {
                class: "text-[16px] font-bold text-center",
                dangerous_inner_html: "{tr.description}",
            }

            div { class: "w-full flex flex-row items-center justify-center gap-[20px]",
                div {
                    id: "btn_prev",
                    class: "w-[50px] flex items-center justify-center",
                    onclick: |_| {},
                    LeftArrow {}
                }
                div { class: "grow grid grid-cols-3 gap-[10px]",
                    for i in start..end {
                        SongCard { song: ctrl.songs()[i].clone() }
                    }
                }

                button {
                    id: "btn_prev",
                    class: "w-[50px] flex items-center justify-center",
                    onclick: |_| {},
                    RightArrow {}
                }
            }
        }
    }
}

#[component]
pub fn SongCard(song: Song) -> Element {
    rsx! {
        div {
            class: "col-span-1 flex flex-col rounded-[10px]",
            background: "linear-gradient(180deg, #848484 65.5%, #666666 82%, #313131 100%)",

            div { class: "flex flex-row gap-[10px] p-[10px] items-center",
                img {
                    class: "w-[100px] h-[100px] object-fit rounded-[10px]",
                    src: if let Some(ref url) = song.image_url { url.to_string() } else { "" },
                }

                div { class: "flex flex-col",
                    p { "title" }
                    div { class: "flex flex-row gap-[1px]",
                        "icons"
                        "name"
                    }
                    div { "lyrics" }
                }
            }

            div { class: "flex flex-row gap-[10px] justify-between p-[10px]",
                div { class: "flex flex-row gap-[10px] items-center",
                    "play"
                    "no palys"
                }

                div { class: "flex flex-row gap-[10px] items-center",
                    "like"
                    "no likes"
                }

                div { class: "flex flex-row gap-[10px] items-center", "share icon" }
            }
        }
    }
}
