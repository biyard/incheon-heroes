#![allow(non_snake_case)]
use crate::components::icons::complete::CompleteIcon;
use crate::components::icons::heart::HeartIcon;
use crate::components::icons::send::SendIcon;
use crate::pages::ColGridCards;

use super::controller::*;
use super::i18n::*;
#[allow(unused_imports)]
use by_components::icons;
use dioxus::prelude::*;
use dioxus_popup::PopupService;
use dioxus_translate::*;
use dto::Content;
use dto::UserContents;

#[component]
pub fn ContentsByIdPage(id: i64, lang: Language) -> Element {
    let ctrl = Controller::new(lang, id)?;
    let tr: ContentsByIdTranslate = translate(&lang);
    let (content, user) = ctrl.rsc()?;

    rsx! {
        by_components::meta::MetaPage { title: "{tr.title}" }

        NFTDescription {
            content: content.clone(),
            user: user.clone(),
            lang,
            opensea_url: ctrl.opensea_url(),
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

#[component]
pub fn NFTDescription(
    content: Content,
    user: UserContents,
    lang: Language,
    opensea_url: String,
) -> Element {
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
                    a { href: "{opensea_url}", "OpenSea Icon" }
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
    let mut is_step0 = use_signal(|| 1);
    let mut popup: PopupService = use_context();

    if is_step0() == 1 {
        popup //open popup
        .open(rsx! {
            div {
                class: "flex flex-col justify-center items-center",
                style: "background-color: #ffffff; ",
                div { class: "pt-[20px] pb-[20px] w-[420px] text-start text-[15px] tracking-wide",
                    span { "{tr.description}" }
                    div { class: "flex justify-start items-center mt-[20px]",
                        input { r#type: "checkbox", class: "mr-2" }
                        label { class: "mr-2 text-[14px] font-bold", "{tr.agreement}" }
                    }
                }
                button {
                    onclick: move |_event| {
                        tracing::debug!("Confirm button");
                        is_step0.set(2);
                    },
                    // TODO: If button clicked, send to after Confirm page function need.
                    div {
                        class: "flex justify-center items-center rounded-[12px] w-[440px] h-[50px] font-normal text-[18px]",
                        style: "background-color: #24B28C; color: #ffffff; ",
                        "{tr.confirm_text}"
                    }
                }
            }
        })
        .with_id("created_popup")
        .with_title(tr.title);
    } else if is_step0() == 2 {
        tracing::debug!("button");
        popup //loading popup
            .open(rsx! {
                div {
                    class: "mb-[10px] flex flex-col justify-between items-center popup-custom",
                    style: "background-color: #ffffff",
                    img {
                        src: "packages/main-ui/public/images/loading.png",
                        class: "w-[124px] h-[124px]",
                    }
                    div { class: "mt-[35px] w-[400px] text-center text-[16px] tracking-wide text-black",
                        span { "{tr.loading_text}" }
                    }
                }
            })
            .with_id("loading_popup")
            .with_title(tr.title)
            .without_close();
    } else if is_step0() == 3 {
        popup //complete popup
        .open(rsx! {
            div {
                class: "flex flex-col justify-center items-center",
                style: "background-color: #ffffff; ",
                CompleteIcon {}
                div { class: "pt-[20px] pb-[20px] w-[400px] text-center text-[15px] tracking-wide",
                    span { "{tr.complete_text}" }
                }
                button {
                    onclick: move |_event| {
                        tracing::debug!("Confirm button");
                    },
                    // TODO: If button clicked, send to after Confirm page function need.
                    div {
                        class: "flex justify-center items-center rounded-[12px] w-[440px] h-[50px] font-normal text-[18px]",
                        style: "background-color: #24B28C; color: #ffffff; ",
                        "{tr.confirm_text}"
                    }
                }
            }
        })
        .with_id("created_popup");
    }
    rsx! {
        div {
            class: "flex justify-center items-center w-[564px] h-[46px] rounded-[12px] align-middle mb-[16px]",
            style: "background-color: #ffffff",
            button {
                div {
                    class: "font-bold text-[18px]",
                    onclick: move |_| {
                        is_step0.set(1);
                    },
                    "{tr.button_text}"
                }
            }
        }
    }
}
