#![allow(non_snake_case)]
use crate::components::icons::heart::HeartIcon;
use crate::components::icons::send::SendIcon;
use crate::pages::ColGridCards;

use super::controller::*;
use super::i18n::*;
#[allow(unused_imports)]
use by_components::icons;
use dioxus::prelude::*;
use dioxus_translate::*;
use dto::Content;
use dto::UserContents;

#[derive(PartialEq, Eq, Clone, Copy)]
enum PopupState {
    Closed,
    Confirm,
    Loading,
    Completed,
}

#[component]
pub fn ContentsByIdPage(id: i64, lang: Language) -> Element {
    let ctrl = Controller::new(lang, id)?;
    let tr: ContentsByIdTranslate = translate(&lang);
    let (content, user) = ctrl.rsc()?;

    rsx! {
        by_components::meta::MetaPage { title: "{tr.title}" }

        NFTDescription { content: content.clone(), user: user.clone(), lang }

        p {
            {
                format!(
                    "{} {} {}",
                    tr.more_contents_lead,
                    user.evm_address,
                    tr.more_contents_tail,
                )
            }
        }
        ColGridCards { lang, contents: user.contents.clone() }
    } // end of this page
}

#[component]
pub fn NFTDescription(content: Content, user: UserContents, lang: Language) -> Element {
    let title = content.title;
    let thumbnail_image = content.thumbnail_image;
    let description = content.description;

    rsx! {
        div { class: "w-[1200px] h-full flex flex-row justify-center items-start gap-[48px] mb-[80px]",
            //image section
            div {
                img {
                    class: "w-[588px] h-[588px] flex justify-center items-center rounded-[12px]",
                    src: "{thumbnail_image}",
                }
            }

            // TODO: connect to image data

            //description section
            div { class: "w-[564px] flex flex-col justify-center",

                div { class: "w-[336px] h-full",
                    //minting status
                    div { class: "w-[336px] h-[26px] mb-[10px] font-bold text-[15px]",
                        "Minting Now"
                    }
                    //NFT title
                    div { class: "w-[336px] h-[38px] mb-[20px] font-normal text-[32px]",
                        "{title}"
                    }
                    //creator profile image and name
                    div { class: "w-full h-full flex flex-row justify-start items-center gap-1 mb-[30px]",
                        div { class: "font-semibold text-[15px]" }
                        "by"
                        div { class: "w-8 h-8 bg-[#d9d9d9] rounded-2xl" }
                        div { class: "font-semibold text-[#16775D] text-[15px]", "{user.id}" }
                                        // TODO: need to connect'Creator Name' data
                    }
                }

                div {
                    //description
                    // TODO: connect to description data
                    class: "mb-[30px]",
                    "{description}"
                }

                div { class: "mb-[60px] flex flex-row justify-start gap-[20px]",
                    //like
                    button { class: "w-[37px] h-[24px] flex flex-row gap-[4px]",
                        HeartIcon {}
                        "0"
                                        // TODO: need data matching
                    }
                    //share
                    button { class: "w-[70px] h-[24px] flex flex-row gap-[4px]",
                        SendIcon {}
                        "Share"
                                        // TODO: need data matching
                    }
                }

                //Mint now button
                MintNowButton { lang }

                div { class: "w-full flex flex-row justify-start items-center gap-[4px]",
                    //mint count
                    p { class: "font-bold", "350" }
                    // TODO: connect to count data
                    p { "Minted" }
                }
            }
        }
    }
}

#[component]
pub fn MintNowButton(lang: Language) -> Element {
    let tr: MintNowButtonTranslate = translate(&lang);
    let mut popup_state = use_signal(|| PopupState::Closed);

    rsx! {
        div {
            button {
                class: "w-full h-[46px] mb-[16px] bg-white rounded-[12px] text-[16px] flex justify-center items-center",
                onclick: move |_| {
                    popup_state.set(PopupState::Confirm);
                },
                "{tr.button_text}"
            }

            match popup_state() {
                PopupState::Confirm => {
                    rsx! {
                        div { class: "popup confirm",
                            button {
                                onclick: move |_| {
                                    popup_state.set(PopupState::Loading);
                                    popup_state.set(PopupState::Completed);
                                },
                                "{tr.confirm_text}"
                            }
                        }
                    }
                }
                PopupState::Loading => {
                    rsx! {
                        div { class: "popup loading",
                            p { "{tr.loading_text}" }
                        }
                    }
                }
                PopupState::Completed => {
                    rsx! {
                        div { class: "popup completed",
                            p { "{tr.complete_text}" }
                            button {
                                onclick: move |_| {
                                    popup_state.set(PopupState::Closed);
                                },
                                "{tr.confirm_text}"
                            }
                        }
                    }
                }
                _ => rsx! {},
            }
        }
    }
}
