#![allow(non_snake_case)]
use crate::components::buttons::HeartButton;
use crate::components::buttons::ShareButton;
use crate::pages::ColGridCards;

use super::controller::*;
use super::i18n::*;
#[allow(unused_imports)]
use by_components::icons;
use by_components::icons::logo::OpenSea;
use dioxus::prelude::*;
use dioxus_translate::*;
use dto::Content;
use dto::UserContents;

#[component]
pub fn ContentsByIdPage(id: ReadOnlySignal<i64>, lang: Language) -> Element {
    let ctrl = Controller::new(lang, id)?;
    let tr: ContentsByIdTranslate = translate(&lang);
    let (content, user) = ctrl.rsc()?;

    rsx! {
        by_components::meta::MetaPage { title: "{content.title}", description: "{content.description}" }
        div { class: "w-full max-w-[1200px] flex flex-col gap-[80px]",

            NftDescription {
                content: content.clone(),
                user: user.clone(),
                lang,
                opensea_url: ctrl.opensea_url(),
            }

            div { class: "flex flex-col gap-[10px] w-full",
                p { class: "text-[16px] font-bold leading-[55.8px]",
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
            }
        }
    } // end of this page
}

#[component]
pub fn NftDescription(
    content: Content,
    user: UserContents,
    lang: Language,
    opensea_url: String,
) -> Element {
    let title = content.title;
    let thumbnail_image = content.thumbnail_image;
    let description = content.description;
    let tr: NftDescriptionTranslate = translate(&lang);
    let mut ctrl: Controller = use_context();
    let mut icon_color = use_signal(|| "#5B5B5B".to_string());

    rsx! {
        div { class: "w-full h-full grid grid-cols-2 max-[700px]:grid-cols-1 gap-[48px]",
            //image section
            img {
                class: "col-span-1 w-full rounded-[12px] overflow-hidden object-contain bg-white",
                src: "{thumbnail_image}",
            }


            //description section
            div { class: "col-span-1 w-full flex flex-col items-start justify-center gap-[30px]",

                div { class: "w-full flex flex-col gap-[10px]",
                    //minting status
                    div { class: "font-bold text-[15px]", "Minting Now" }

                    div { class: "w-full flex flex-col gap-[20px]",
                        //NFT title
                        p { class: " font-black text-[32px]", "{title}" }
                        //creator profile image and name
                        div { class: "w-full flex flex-row justify-start items-center gap-[10px]",
                            span { class: "font-semibold text-[15px]", "by" }
                            div { class: "flex flex-row items-center justify-center gap-[5px]",
                                img {
                                    class: "w-[25px] h-[25px] bg-[#d9d9d9] rounded-full",
                                    src: "{user.profile_url}",
                                }
                                div { class: "font-semibold text-[#16775D] text-[15px]",
                                    "{user.evm_address}"
                                }
                            }
                        }
                    }
                }

                div { class: "h-full max-h-[200px] text-[#5B5B5B]", "{description}" }

                div { class: "flex flex-row justify-start items-center gap-[10px] text-[#5B5B5B]",
                    //opensea
                    a {
                        class: " hover:bg-[#0086FF] hover:text-[#ffffff] px-[10px] py-[4px] rounded-[12px] flex flex-row items-center justify-center gap-[10px]",
                        target: "_blank",
                        href: "{opensea_url}",
                        onmouseover: move |_| icon_color.set("#ffffff".to_string()),
                        onmouseout: move |_| icon_color.set("#5B5B5B".to_string()),
                        OpenSea { size: 30, color: "{icon_color()}" }
                        "View in OpenSea"
                    }

                    //like
                    HeartButton {
                        onclick: move |_| async move {
                            ctrl.handle_like().await;
                        },
                        liked: content.liked,
                        "{content.likes}"
                    }
                    //share
                    ShareButton {
                        onclick: move |_| async move {
                            ctrl.handle_share().await;
                        },
                    }
                }

                //Mint now button
                div { class: "w-full flex flex-col gap-[16px] items-start justify-center text-[#191919]",
                    button {
                        class: "flex justify-center bg-white items-center w-full h-[46px] rounded-[12px] align-middle hover:text-[#16775D] hover:border-[1px] hover:border-[#16775D] hover:bg-[#E4F4E4] font-semibold text-[18px]",
                        box_shadow: "0px 4px 20px rgba(84, 157, 159, 0.25)",
                        onclick: move |_| {
                            ctrl.open_minting_popup();
                        },
                        "{tr.button_text}"
                    }

                    div { class: "w-full flex flex-row justify-start items-center gap-[4px]",
                        //mint count
                        p { class: "font-bold", "{content.downloads}" }
                        p { "{tr.downloads}" }
                    }
                }
            }
        }
    }
}
