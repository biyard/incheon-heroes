#![allow(non_snake_case)]
use crate::components::icons::heart::HeartIcon;
use crate::components::icons::send::SendIcon;
use crate::pages::ColGridCards;

use super::controller::*;
use super::i18n::*;
#[allow(unused_imports)]
use by_components::icons;
use dioxus::prelude::*;
// use dioxus_popup::PopupService;
use dioxus_translate::*;
// enum PopupState {
//     Closed,
//     Confirm,
//     Loading,
//     Completed,
// }

#[component]
pub fn ContentsByIdPage(id: i64, lang: Language) -> Element {
    let mut ctrl = Controller::new(lang, id)?;
    tracing::debug!("log");
    let tr: ContentsByIdTranslate = translate(&lang);
    let (content, user) = ctrl.get_nft_data();

    rsx! {
        by_components::meta::MetaPage { title: "{tr.title}" }

        NFTDescription {}

        div { id: "contents-by-id", class: "flex flex-col gap-[40px]",
            p { "{content:?}" }
            p { "{user:?}" }

            div { class: "flex flex-row gap-[30px]",
                img {
                    src: "{content.thumbnail_image}",
                    alt: "{content.title}",
                    class: "w-full h-[300px] object-cover",
                }
            }

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
}

#[component]
pub fn NFTDescription() -> Element {
    rsx! {
        div { class: "w-[1200px] h-full flex flex-row justify-center items-start gap-[48px]",
            //image section
            div { class: "w-[588px] h-[588px] flex justify-center items-center" }

            // TODO: connect to image data

            //description section
            div { class: "w-[564px] h-full flex-col justify-start items-center",

                div { class: "w-[336px] h-full",
                    //minting status
                    div { class: "w-[336px] h-[26px] mb-[10px] font-bold text-[15px]",
                        "Minting Now"
                    }
                    //NFT title
                    div { class: "w-[336px] h-[38px] mb-[20px] font-normal text-[32px]",
                        "NFT Title"
                    }
                    //creator profile image and name
                    div { class: "w-full h-full flex flex-row justify-start items-center gap-1 mb-[30px]",
                        div { class: "font-semibold text-[15px]" }
                        "by"
                        div { class: "w-8 h-8 bg-[#d9d9d9] rounded-2xl" }
                        div { class: "font-semibold text-[#16775D] text-[15px]", "Creator Name" }
                    }
                }

                div {
                    //description
                    // TODO: connect to description data
                    class: "mb-[30px]",
                    "The Roots of Knowledge NFT is the entry point to the bot. fun ecosystem and the upcoming $BOT launch, powered by Proof of Knowledge. The Roots of Knowledge NFT is the entry point to the bot. fun ecosystem and the upcoming $BOT launch, powered by Proof of Knowledge."
                }

                div { class: "mb-[60px] flex flex-row justify-start gap-[20px]",
                    //like
                    div { class: "w-[37px] h-[24px] flex flex-row gap-[4px]",
                        HeartIcon {}
                        "0"
                    }
                    //share
                    div { class: "w-[70px] h-[24px] flex flex-row gap-[4px]",
                        SendIcon {}
                        "Share"
                    }
                }

                button {
                    //Mint now button
                    class: "w-full h-[46px] mb-[16px] bg-white rounded-[12px] text-[16px] flex justify-center items-center",
                    "Mint now"
                }

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

// #[component]
// pub fn MintNowButton(lang: Language) -> Element {
//     let tr: MintNowButtonTranslate = translate(&lang);
//     let mut popup_state = use_signal(|| PopupState::Closed);
//     let mut popup: PopupService = use_context();

//     // 팝업이 열려 있을 경우 UI 변경
//     if popup_state() != PopupState::Closed {
//         popup.open(rsx! {
//             div { class: "flex flex-col justify-between items-center",
//                 match popup_state() {
//                     PopupState::Confirm => rsx! {
//                         div { class: "mt-[35px] mb-[35px] w-[400px] text-center text-[16px] tracking-wide",
//                             span { "{tr.description}" }
//                         }
//                         button {
//                             onclick: move |_| {
//                                 popup_state.set(PopupState::Loading);
//                                 let popup_state = popup_state.clone();
//                                     popup_state.set(PopupState::Completed);
//                                 });
//                             },
//                             div {
//                                 class: "flex justify-center items-center rounded-[12px] w-[400px] h-[57px] font-extrabold text-[18px]",
//                                 style: "background-color: #74789E; color: #212231;",
//                             }
//                         }
//                     },
//                     PopupState::Loading => rsx! {
//                         div { class: "mt-[35px] w-[400px] text-center text-[16px] tracking-wide",
//                             span { class: "font-bold text-[#B5AB65]", "Loading..." }
//                         }
//                     },
//                     PopupState::Completed => rsx! {
//                         div { class: "mt-[35px] mb-[35px] w-[400px] text-center text-[16px] tracking-wide" }
//                         button {
//                             onclick: move |_| {
//                                 popup_state.set(PopupState::Closed);
//                             },
//                             div {
//                                 class: "flex justify-center items-center rounded-[12px] w-[400px] h-[57px] font-extrabold text-[18px]",
//                                 style: "background-color: #B5AB65; color: #212231;",
//                             }
//                         }
//                     },
//                     _ => rsx! {},
//                 }
//             }
//         })
//         .with_id("popup")
//         .with_title(tr.title)
//         .without_close();
//     }

//     rsx! {
//         div { class: "flex justify-center gap-[30px] mt-[50px]",
//             div {
//                 class: "flex justify-center items-center w-[400px] h-[57px] rounded-[12px] align-middle",
//                 style: "background-color: #74789E",
//                 button {
//                     onclick: move |_event| {
//                         tracing::debug!("Cancel button");
//                     },
//                     div {
//                         class: "font-bold text-[18px]",
//                         style: "color: #212231;",
//                         "Mint Now"
//                     }
//                 }
//             }
//         }
//     }
// }
