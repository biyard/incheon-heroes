#![allow(non_snake_case)]
use by_components::loaders::radial_spinner::RadialSpinner;
use dioxus::prelude::*;
use dioxus_popup::PopupService;
use dioxus_translate::{translate, Language};
use dto::Content;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::{
    components::icons::complete::CompleteIcon, config,
    pages::contents::_id::i18n::MintingPopupTranslate,
};

#[component]
pub fn MintingPopup(lang: Language, id: ReadOnlySignal<i64>) -> Element {
    tracing::debug!("lang: {}", lang);
    let tr: MintingPopupTranslate = translate(&lang);
    let mut checked = use_signal(|| false);
    let mut popup: PopupService = use_context();

    rsx! {
        div { class: "carousel w-[480px]",

            div { class: "carousel-item relative w-full", id: "slide1",
                div {
                    class: "flex flex-col justify-start items-center gap-[30px]",
                    style: "background-color: #ffffff; ",
                    p { class: "text-[24px] text-[#191919] font-bold leading-[45px]",
                        "{tr.title}"
                    }

                    div { class: "w-full flex flex-col items-start justify-start gap-[20px] text-[#5B5B5B]",

                        div { class: "w-full text-start text-[15px] tracking-wide",
                            span { "{tr.description}" }
                        }
                        div { class: "flex justify-start items-center",
                            div { class: "form-control cursor-pointer",
                                label {
                                    class: "label cursor-pointer flex flex-row gap-[10px] cursor-pointer",
                                    onclick: move |_event| {
                                        checked.set(!checked());
                                    },
                                    input {
                                        checked: checked(),
                                        class: "checkbox [--chkbg:theme(colors.green.500)] [--chkfg:white] border-[#16775D] w-[18px] h-[18px] rounded-[4px]",
                                        r#type: "checkbox",
                                    }
                                    label { class: "mr-2 text-[14px] font-bold cursor-pointer",
                                        "{tr.agreement}"
                                    }
                                }
                            }
                        }

                        a {
                            href: "#slide2",
                            class: "flex justify-center items-center rounded-[12px] w-full h-[50px] font-normal text-[18px] text-white bg-[#24B28C] hover:bg-[#1E9E7A]",
                            onclick: move |evt| {
                                if !checked() {
                                    evt.prevent_default();
                                    evt.stop_propagation();
                                    return;
                                }
                                spawn(async move {
                                    let endpoint = config::get().new_api_endpoint;
                                    match Content::get_client(endpoint).mint(id()).await {
                                        Ok(_) => {
                                            let document = web_sys::window().unwrap().document().unwrap();
                                            let complete_mint = document
                                                .get_element_by_id("complete-mint")
                                                .unwrap();
                                            if let Ok(el) = complete_mint.dyn_into::<HtmlElement>() {
                                                tracing::debug!("click complete mint");
                                                el.click();
                                            }
                                        }
                                        Err(e) => {
                                            tracing::error!("Minting error: {:?}", e);
                                        }
                                    };
                                });
                            },
                            "{tr.confirm_text}"
                        }
                    }
                }
            } // notice


            div { class: "carousel-item relative w-full", id: "slide2",
                div {
                    class: "w-full flex flex-col justify-start items-center gap-[30px]",
                    style: "background-color: #ffffff; ",
                    p { class: "text-[24px] text-[#191919] font-bold leading-[45px]",
                        "{tr.title}"
                    }
                    div { class: "flex flex-col justify-center items-center gap-[20px] text-[#5B5B5B]",
                        div { class: "w-[124px] h-[124px] flex items-center justify-center",
                            RadialSpinner { size: 90 }
                        }
                        span { "{tr.loading_text}" }
                    }
                    a {
                        id: "complete-mint",
                        class: "hidden",
                        href: "#slide3",
                    }
                }
            }

            div { class: "carousel-item relative w-full", id: "slide3",
                div { class: "w-full flex flex-col justify-between items-center gap-[20px] pt-[40px]",
                    CompleteIcon { size: 120 }
                    div { class: "flex flex-col w-full items-center justify-center gap-[20px] text-[#5B5B5B]",

                        p { class: "w-full text-center text-[15px] tracking-wide text-[#5B5B5B] font-bold",
                            "{tr.complete_text}"
                        }
                        button {
                            class: "flex justify-center items-center rounded-[12px] w-[440px] h-[50px] font-normal text-[18px] bg-[#24B28C] hover:bg-[#1E9E7A] text-white",
                            onclick: move |_event| {
                                popup.close();
                            },
                            "{tr.confirm_text}"
                        }

                    }
                }
            }
        }
    }
}
