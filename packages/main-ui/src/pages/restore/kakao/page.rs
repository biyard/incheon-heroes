#![allow(non_snake_case)]
use by_components::loaders::radial_spinner::RadialSpinner;
use dioxus::prelude::*;
use dioxus_translate::*;

use crate::{route::Route, services::user_service::UserService};

#[component]
pub fn KakaoRestorePage(lang: Language, id: String, seed: String) -> Element {
    let nav = use_navigator();
    let mut user: UserService = use_context();
    tracing::debug!("Restoring from Kakao seed: {} for {}", seed, id);

    use_effect(move || {
        let id = id.clone();
        let seed = seed.clone();

        spawn(async move {
            match user.restore_from_seed(&id, &seed).await {
                Ok(_) => {
                    nav.replace(Route::HomePage { lang });
                }
                Err(_) => {
                    nav.replace(Route::ConnectPage { lang });
                }
            }
        });
    });

    rsx! {
        div {
            id: "kakao",
            class: "h-full w-full flex items-center justify-center",
            RadialSpinner { size: 200 }
        } // end of this page
    }
}
