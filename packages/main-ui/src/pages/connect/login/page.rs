#![allow(non_snake_case)]
use crate::components::headings::Heading1;
use crate::services::backend_api::AccountHint;

use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn LoginPage(lang: Language, hint: AccountHint) -> Element {
    let ctrl = Controller::new(lang, hint)?;
    let tr: LoginTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: "{tr.title}" }

        div { id: "login", class: "flex flex-col gap-[50px] items-center",
            Heading1 { lang, "{tr.title}" }
            div { class: "flex flex-col gap-[20px] items-center w-full",
                p { class: "text-[14px] font-semibold", "{tr.description}" }
                input {
                    r#type: "password",
                    class: "w-full max-w-[500px] h-[60px] border-[1px] border-black/40 rounded-[5px] p-[10px] flex flex-row justify-start items-center bg-[#E9F2EC]",
                    placeholder: "{tr.placeholder}",
                }
                button {
                    class: "w-full max-w-[500px] h-[60px] bg-[#2196F3] text-white text-[20px] font-bold rounded-[5px]",
                    onclick: move |_| async move {
                        ctrl.handle_login().await;
                    },
                    "{tr.btn}"
                }
            }
        } // end of this page
    }
}
