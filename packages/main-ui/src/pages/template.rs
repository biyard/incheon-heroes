#![allow(non_snake_case)]
use super::i18n::FooterTranslate;
use super::i18n::HeaderTranslate;
use super::i18n::LoginButtonTranslate;
use super::i18n::MainTextTranslate;
use crate::assets::*;
use crate::route::Route;
use crate::theme::ColorTheme;
use base64::display;
use dioxus::prelude::*;
// use dioxus_oauth::component;
use dioxus_popup::PopupZone;
use dioxus_translate::*;

#[component]
pub fn RootLayout(lang: Language) -> Element {
    let theme: ColorTheme = use_context();

    rsx! {
        div {
            class: "flex flex-col w-full items-center justify-start min-h-[100vh] text-white",
            style: "background: {theme.background}; color: {theme.primary_text}",
            Header {
                class: "w-full h-[70px] flex items-center justify-center max-w-[1440px]",
                lang,
            }
            div { class: "w-full my-[70px]", Outlet::<Route> {} }
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

#[component]
pub fn Footer(
    #[props(default ="footer".to_string())] id: String,
    #[props(default ="".to_string())] class: String,

    lang: Language,
) -> Element {
    let tr: FooterTranslate = translate(&lang);

    rsx! {
        footer { id, class: "bg-gray-700 text-gray-500 w-full p-3",
            div {
                id: "footer",
                class: "w-full flex flex-col justify-start items-start",
                img { src: "{LOGO}", class: "w-[145px] h-[50px]" }
                div { class: "m-1" }
                div { class: "text-left w-full",
                    p { class: "text-xs", style: "white-space: nowrap;", "{tr.address}" }
                    div { class: "m-1" }
                    p { class: "text-xs", "{tr.copyright}" }
                }
                footer {
                    position: "fixed",
                    bottom: "0",
                    width: "100%",
                    padding: "1rem",
                }
            }
        }
    }
}

#[component]
pub fn VideoSection() -> Element {
    rsx! {
        div { id: "videosection", class: "flex justify-center items-center p-4",
            video {
                class: "w-full max-w-6xl",
                autoplay: "true",
                r#loop: "true",
                muted: "true",
                src: "{VIDEO}",
            }
        }
    }
}

#[component]
pub fn Maintext(
    #[props(default ="main_text".to_string())] id: String,
    #[props(default ="".to_string())] class: String,

    lang: Language,
) -> Element {
    let tr: MainTextTranslate = translate(&lang);

    rsx! {
        div { class: "flex justify-center items-center p-4",
            img { src: "{DIAMOND}", class: "w-[50px] h-[50px]" }
            p { class: "text-5xl text-center font-bold", "INCHEON HEROES" }
            img { src: "{DIAMOND}", class: "w-[50px] h-[50px]" }
        }
        p { id: "main_text", class: "text-xs text-center", "{tr.main_text}" }
    }
}

#[component]
pub fn LoginButton(
    #[props(default ="login_button".to_string())] id: String,
    #[props(default ="".to_string())] class: String,

    lang: Language,
) -> Element {
    let tr: LoginButtonTranslate = translate(&lang);

    rsx! {
        div { class: "flex justify-center items-center p-4",
            button {
                onclick: |_| println!("Button clicked!"),
                class: "px-4 py-2 bg-gray-500 text-white rounded-full hover:bg-gray-600 text-xl font-bold",
                style: "width: 250px; height: 60px; padding-10 px",
                "{tr.button_text}"
            }
        }
    }
}
