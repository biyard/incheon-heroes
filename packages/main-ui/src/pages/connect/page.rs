#![allow(non_snake_case)]
use crate::components::headings::Heading1;

use super::controller::*;
use super::i18n::*;
use by_components::icons;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn ConnectPage(lang: Language) -> Element {
    let ctrl = Controller::new(lang)?;
    let tr: ConnectTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: "{tr.title}", description: "{tr.desc}" }

        div {
            id: "connect",
            class: "w-full flex flex-col gap-[40px] mx-[10px] items-center",
            div { class: "flex flex-col gap-[20px]",
                Heading1 { lang, "{tr.title}" }
                p { class: "text-[14px] text-center", "{tr.desc}" }
            }

            div {
                class: "flex flex-col w-full max-w-[550px] rounded-[20px]",
                background: "rgba(233, 242, 236, 0.2)",
                box_shadow: "0px 0px 12px #16775D",

                button {
                    class: "w-full flex flex-row h-[75px] items-center justify-start px-[40px] text-[20px] font-normal gap-[40px] hover:bg-white/30",
                    onclick: move |_| async move {
                        ctrl.handle_google().await;
                    },
                    icons::logo::Google { size: 35 }
                    p { "{tr.google}" }
                }

                button {
                    class: "w-full flex flex-row h-[75px] items-center justify-start px-[40px] text-[20px] font-normal gap-[40px] hover:bg-white/30",
                    onclick: move |_| async move {
                        ctrl.handle_kakao().await;
                    },
                    icons::logo::Kakao { size: 35 }
                    p { "{tr.kakao}" }
                }
                button {
                    class: "w-full flex flex-row h-[75px] items-center justify-start px-[40px] text-[20px] font-normal gap-[40px] hover:bg-white/30",
                    onclick: move |_| async move {
                        ctrl.handle_kaia().await;
                    },
                    icons::logo::Kaia { size: 35 }
                    p { "{tr.kaia}" }
                }
                button {
                    class: "w-full flex flex-row h-[75px] items-center justify-start px-[40px] text-[20px] font-normal gap-[40px] hover:bg-white/30",
                    onclick: move |_| async move {
                        ctrl.handle_internet_identity().await;
                    },
                    icons::logo::InternetIdentity { size: 35 }
                    p { "{tr.ii}" }
                }
            
            }
        } // end of this page
    }
}
