#![allow(non_snake_case)]
use crate::components::headings::Heading1;

use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn StoriesPage(lang: Language) -> Element {
    let tr: StoriesTranslate = translate(&lang);

    rsx! {
        div {
            id: "stories",

            class: "flex flex-col gap-[0px] items-center justify-center",
            div { class: "flex flex-col gap-[40px] items-center justify-center py-[120px]",
                Heading1 { lang, "{tr.title}" }
                div {
                    class: "text-[16px] font-bold text-center",
                    dangerous_inner_html: "{tr.desc}",
                }
            }
            CharacterStory { lang }
        }
    }
}

#[component]
pub fn CharacterStory(lang: Language) -> Element {
    let tr: CharacterTranslate = translate(&lang);
    let ainy = asset!("/public/images/ainy.png");
    let bumy = asset!("/public/images/bumy.png");
    let comy = asset!("/public/images/comy.png");
    let daery = asset!("/public/images/daeri.png");

    let characters = vec![
        (tr.ainy_name, tr.ainy_role, tr.ainy_description, ainy, false),
        (tr.bumy_name, tr.bumy_role, tr.bumy_description, bumy, true),
        (tr.comy_name, tr.comy_role, tr.comy_description, comy, false),
        (
            tr.daeri_name,
            tr.daeri_role,
            tr.daeri_description,
            daery,
            true,
        ),
    ];

    rsx! {
        div {
            class: "w-screen bg-white flex items-center justify-center",
            background: "linear-gradient(180deg, #FFFFFF -5.25%, #E9F2EC 64.72%)",
            div { class: "w-full max-w-[1440px] flex flex-col py-[120px] gap-[120px] items-center justify-center",
                Heading1 { lang, "{tr.title}" }
                for (name , role , description , image , reverse) in characters {
                    Character {
                        name,
                        role,
                        description,
                        image,
                        reverse,
                        lang,
                    }
                }
            }
        }
    }
}

#[component]
pub fn Character(
    name: String,
    role: String,
    description: String,
    image: String,
    #[props(default = true)] reverse: bool,
    lang: Language,
) -> Element {
    rsx! {
        div { class: "w-full max-w-[1064px] grid grid-cols-5 items-center justify-start",
            if reverse {
                img { class: "col-span-2", src: "{image}" }
            }
            div { class: "col-span-3 flex flex-col gap-[24px]",
                div { class: "h-[6px] bg-black w-[40px]" }
                div { class: "flex flex-row gap-[24px] items-center",
                    Heading1 { with_symbol: false, lang, "{name}" }
                    div { class: "text-[28px] font-black", "{role}" }
                }
                div {
                    class: "text-[16px] font-normal text-start",
                    dangerous_inner_html: "{description}",
                }
            }
            if !reverse {
                img { class: "col-span-2", src: "{image}" }
            }
        }
    }
}
