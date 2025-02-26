#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_popup::PopupService;
use dioxus_translate::{translate, Language};

use crate::{
    components::icons::complete::CompleteIcon, pages::contents::_id::i18n::MintingPopupTranslate,
};

#[component]
pub fn MintingPopup(lang: Language) -> Element {
    let tr: MintingPopupTranslate = translate(&lang);
    let mut popup: PopupService = use_context();

    use_effect(move || {
        popup.with_title(&tr.title);
    });

    rsx! {
        div { class: "carousel w-[480px]",
            div { class: "carousel-item relative w-full", id: "slide1",
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
                    a {
                        href: "#slide2",
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

            }
            div { class: "carousel-item relative w-full", id: "slide2",
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
            }
            div { class: "carousel-item relative w-full", id: "slide3",
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
            }
            div { class: "carousel-item relative w-full", id: "slide4" }
        }
    }
}
