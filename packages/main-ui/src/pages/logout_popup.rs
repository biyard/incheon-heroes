#![allow(non_snake_case)]
use crate::Route;
use crate::{pages::i18n::LogoutPopupTranslate, services::user_service::UserService};
use dioxus::prelude::*;
use dioxus_popup::PopupService;
use dioxus_translate::*;

#[component]
pub fn LogoutPopup(
    #[props(default = "logout_popup".to_string())] id: String,
    lang: Language,
) -> Element {
    let mut popup: PopupService = use_context();
    let mut user_wallet: UserService = use_context();
    let tr = translate::<LogoutPopupTranslate>(&lang);
    let nav = use_navigator();
    rsx! {
        div { class: "popup-custom bg-white w-[450px] h-[170px] mt-[30px]rounded-lg flex flex-col items-center gap-[30px]",
            p { class: "font-bold text-lg", "{tr.title}" }
            p { class: "text-md mb-4", "{tr.description}" }
            div { class: "flex justify-center gap-4 mb-[50px]",
                button {
                    class: "w-[200px] h-[40px] bg-black text-white rounded-[24px]",
                    onclick: move |_| {
                        tracing::debug!("Logout button clicked");
                        spawn(async move {
                            user_wallet.logout().await;
                            nav.push(Route::HomePage { lang });
                            popup.close();
                        });
                    },
                    // TODO: need function work
                    "{tr.logout_button}"
                }
                button {
                    class: "w-[200px] h-[40px] bg-black text-white rounded-[24px]",
                    onclick: move |_| {
                        popup.close();
                    },
                    "{tr.cancel_button}"
                }
            }
        }
    }
}
