use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

use crate::pages::my_nfts::_id::i18n::SendPopupTranslate;

#[component]
pub fn SendPopup(
    lang: Language,
    onsend: EventHandler<String>,
    oncancel: EventHandler<MouseEvent>,
) -> Element {
    let tr: SendPopupTranslate = translate(&lang);
    let mut address = use_signal(|| "".to_string());

    rsx! {
        div { class: "flex flex-col w-[300px] md:w-[800px] max-w-[800px] h-[300px] justify-center items-center gap-[40px]",
            div { class: "font-semibold text-[20px] md:text-[30px] text-black", "NFT 전송" }
            div { class: "flex flex-col w-full h-full justify-between items-center",
                div { class: "flex flex-col w-full justify-center items-center gap-[10px]",
                    div { class: "font-medium text-[16px] md:text-[16px] text-center md:text-left whitespace-pre-line md:whitespace-nowrap text-[#16775d]",
                        "{tr.title}"
                    }
                    input {
                        class: "w-full h-[55px] px-[16px] text-[#1d1820] bg-transparent border border-[#79747e] font-medium focus:outline-none rounded-[10px]",
                        placeholder: "{tr.input_address_hint}",
                        oninput: move |e| {
                            address.set(e.value());
                        },
                    }
                }

                div { class: "flex flex-row w-full h-[50px] justify-start items-start gap-[10px]",
                    div {
                        class: "cursor-pointer flex flex-row flex-1 justify-center items-center h-full bg-gray-500 rounded-[10px] font-medium text-white text-[16px]",
                        onclick: move |_| {
                            onsend.call(address());
                        },
                        "{tr.transfer}"
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
