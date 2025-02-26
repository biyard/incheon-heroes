use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

use crate::pages::my_nfts::_id::i18n::ExchangePopupTranslate;

#[component]
pub fn ExchangePopup(
    lang: Language,
    onexchange: EventHandler<MouseEvent>,
    oncancel: EventHandler<MouseEvent>,
) -> Element {
    let tr: ExchangePopupTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-[300px] md:w-[800px] max-w-[800px] h-[300px] justify-center items-center gap-[40px]",
            div { class: "font-semibold text-[20px] md:text-[30px] text-black", "{tr.title}" }
            div { class: "flex flex-col w-full h-full justify-between items-center",
                div { class: "font-medium text-[16px] md:text-[16px] text-center md:text-left whitespace-pre-line md:whitespace-nowrap text-[#16775d]",
                    "{tr.description}"
                }

                div { class: "flex flex-row w-full h-[50px] justify-start items-start gap-[10px]",
                    div {
                        class: "cursor-pointer flex flex-row flex-1 justify-center items-center h-full bg-gray-500 rounded-[10px] font-medium text-white text-[16px]",
                        onclick: move |e: Event<MouseData>| {
                            onexchange.call(e);
                        },
                        "{tr.exchange}"
                    }
                    div {
                        class: "cursor-pointer flex flex-row flex-1 justify-center items-center h-full bg-[#16775d] rounded-[10px] font-medium text-white text-[16px]",
                        onclick: move |e: Event<MouseData>| {
                            oncancel.call(e);
                        },
                        "{tr.cancel}"
                    }
                }
            }
        }
    }
}
