#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_popup::PopupService;
use dioxus_translate::*;
use dto::{Content, ContentCreateRequest};

use crate::{config, pages::new::i18n::ConsentPopupTranslate, route::Route};

#[component]
pub fn ConsentPopup(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    contents: ReadOnlySignal<Vec<ContentCreateRequest>>,
    lang: Language,
) -> Element {
    let tr: ConsentPopupTranslate = translate(&lang);
    let mut popup: PopupService = use_context();
    let nav = use_navigator();

    rsx! {
        div {..attributes,
            div {
                class: "w-full flex flex-col justify-start items-center gap-[30px] p-4",
                style: "background-color: #ffffff; ",
                p { class: "text-[24px] text-[#191919] font-bold leading-[45px] text-center",
                    "{tr.title}"
                }

                p { class: "text-[16px] text-[#191919] leading-[24px] text-center",
                    "{tr.minting_consent}"
                }

                div { class: "w-full grid grid-cols-2 gap-[20px]",

                    button {
                        class: "col-span-1 w-full h-[40px] bg-[#F2F2F2] text-[#191919] font-bold text-[16px] leading-[24px] rounded-[5px] hover:bg-[#E5E5E5] hover:text-[#191919]",
                        onclick: move |_| {
                            btracing::error!("{}", dto::Error::MustAgreeToTerms.translate(& lang));
                            popup.close();
                        },
                        "{tr.decline}"
                    }

                    button {
                        class: "col-span-1 w-full h-[40px] bg-[#24B28C] text-white font-bold text-[16px] leading-[24px] rounded-[5px] hover:bg-[#1D8E6D] hover:text-white",
                        onclick: move |_| async move {
                            let endpoint = config::get().new_api_endpoint;
                            if let Err(e) = Content::get_client(endpoint).create_bulk(contents()).await {
                                tracing::error!("Failed to create content: {:?}", e);
                                btracing::error!("{}", e.translate(& lang));
                            } else {
                                nav.replace(Route::ContentsPage { lang });
                            }
                            popup.close();
                        },
                        "{tr.accept}"
                    }
                }
            }
        }
    }
}
