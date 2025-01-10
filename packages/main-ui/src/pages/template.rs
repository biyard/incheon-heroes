#![allow(non_snake_case)]
use super::i18n::HeaderTranslate;
use crate::assets::*;
use crate::route::Route;
use crate::theme::ColorTheme;
use dioxus::prelude::*;
use dioxus_popup::PopupZone;
use dioxus_translate::*;

#[component]
pub fn RootLayout(lang: Language) -> Element {
    let theme: ColorTheme = use_context();

    rsx! {
        div {
            class: "flex flex-col w-full items-center justify-start w-full min-h-[100vh] text-white",
            style: "background: {theme.background}; color: {theme.primary_text}",
            Header {
                class: "w-full h-[70px] flex items-center justify-center max-w-[1440px]",
                lang,
            }
            div { class: "w-full max-w-[1440px] my-[70px]", Outlet::<Route> {} }
        }
        PopupZone {}
    }
}

#[component]
pub fn Header(
    #[props(default ="header".to_string())] id: String,
    #[props(default ="".to_string())] class: String,

    lang: Language,
) -> Element {
    let tr: HeaderTranslate = translate(&lang);

    rsx! {
        div { id, class,
            Link {
                class: "flex items-center justify-center",
                to: Route::HomePage { lang },
                img { src: "{LOGO}", class: "w-[145px] h-[50px]" }
            }
            div {
                id: "menu",
                class: "grow grid grid-cols-5 items-center justify-center",
                Menu { value: "{tr.history}", to: Route::HomePage { lang } }
                Menu { value: "{tr.event}", to: Route::HomePage { lang } }
                Menu { value: "{tr.shop}", to: Route::HomePage { lang } }
                Menu { value: "{tr.dao}", to: Route::HomePage { lang } }
                Menu { value: "{tr.info}", to: Route::HomePage { lang } }
            }
        }
    }
}

#[component]
pub fn Menu(value: String, #[props(into)] to: NavigationTarget) -> Element {
    rsx! {
        Link {
            id: "menu-{value}",
            to,
            class: "flex items-center justify-center text-[16px] font-bold col-span-1",
            "{value}"
        }
    }
}
