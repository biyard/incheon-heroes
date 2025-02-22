#![allow(non_snake_case)]
use super::i18n::FooterTranslate;
use super::i18n::HeaderTranslate;
use crate::assets::*;
use crate::components::icons;
use crate::components::icons::arrows::{ArrowDirection, SingleSimpleArrow};
use crate::route::Route;
use crate::services::user_service::UserService;
use by_components::loaders::cube_loader::CubeLoader;
use by_components::meta::MetaSeoTemplate;
use by_components::theme::ColorTheme;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn RootLayout(lang: Language) -> Element {
    let theme: ColorTheme = use_context();
    let path: Route = use_route();

    rsx! {
        MetaSeoTemplate {
            title: "Incheon Heroes",
            canonical: "https://incheonheroes.world",
            keywords: "Incheon, Heroes, 인천, 히어로즈, 유니버스",
            url: "{path}",
        }

        div {
            class: "flex flex-col w-full items-center justify-start min-h-[100vh] text-white",
            background: "{theme.background}",
            color: "{theme.text.primary}",
            Header {
                class: "w-full min-h-[70px] flex flex-row items-center justify-center max-w-[1440px] max-[1440px]:px-[20px]",
                lang,
            }
            div {
                class: "w-full max-w-[1440px] py-[70px] max-[1440px]:px-[20px]",
                min_height: "calc(100vh - 190px)",
                SuspenseBoundary {
                    fallback: |_| rsx! {
                        div { class: "absolute left-0 top-0 w-full h-full flex items-center justify-center",
                            CubeLoader {}
                        }
                    },

                    Outlet::<Route> {}
                }
            }
            Footer { lang }
        }
    }
}

#[component]
pub fn Header(
    #[props(default ="header".to_string())] id: String,
    #[props(default ="".to_string())] class: String,

    lang: Language,
) -> Element {
    let route: Route = use_route();
    let nav = use_navigator();
    let user_wallet: UserService = use_context();
    let tr: HeaderTranslate = translate(&lang);
    let mut expanded = use_signal(|| false);

    let handle_select_menu = move |_| {
        expanded.set(false);
    };

    rsx! {
        div { id, class,
            Link {
                class: "flex items-center justify-center h-[70px]",
                to: Route::HomePage { lang },
                img { src: "{LOGO}", class: "w-[145px] h-[50px]" }
            }

            div { class: "grow flex flex-col items-center justify-center px-[100px]",
                div {
                    id: "menus",
                    class: "w-full flex flex-row justify-start h-[70px]",
                    ExpandableMenu {
                        expanded: expanded(),
                        onclick: move |_| {
                            expanded.set(!expanded());
                        },
                        name: "{tr.history}",
                        SubMenu {
                            to: Route::NoticesPage { lang },
                            onclick: handle_select_menu,
                            "{tr.notice}"
                        }
                        SubMenu {
                            to: Route::StoriesPage { lang },
                            onclick: handle_select_menu,
                            "{tr.story}"
                        }
                        SubMenu {
                            to: Route::ContributorsPage { lang },
                            onclick: handle_select_menu,
                            "{tr.top_contributor}"
                        }
                    }

                    ExpandableMenu {
                        expanded: expanded(),
                        onclick: move |_| {
                            expanded.set(!expanded());
                        },
                        name: "{tr.event}",
                        // SubMenu {
                        //     to: Route::CalendarPage { lang },
                        //     onclick: handle_select_menu,
                        //     "{tr.calendar}"
                        // }
                        SubMenu {
                            to: Route::SongsPage { lang },
                            onclick: handle_select_menu,
                            "{tr.contest_voting}"
                        }

                    }
                    Menu {
                        to: Route::ShopPage { lang },
                        onclick: handle_select_menu,
                        "{tr.shop}"
                    }
                    if user_wallet.is_logined() {
                        Menu {
                            to: Route::MyNftsPage { lang },
                            onclick: handle_select_menu,
                            "{tr.my_nfts}"
                        }
                    }
                    ExpandableMenu {
                        expanded: expanded(),
                        onclick: move |_| {
                            expanded.set(!expanded());
                        },
                        name: "{tr.info}",
                        SubMenu {
                            to: Route::FaqPage { lang },
                            onclick: handle_select_menu,
                            "{tr.faq}"
                        }

                        a {
                            href: "https://incheon-universe.gitbook.io/incheon-universe",
                            target: "_blank",
                            class: "text-[14px] font-regular",
                            onclick: handle_select_menu,
                            "{tr.docs}"
                        }

                    }
                }

            } // end of grow

            div { class: "flex flex-row gap-[15px] items-center h-full",
                button {
                    onclick: move |_| {
                        nav.push(Route::ConnectPage { lang });
                    },
                    icons::Connect { fill: if user_wallet.is_logined() { "#CEF7E3" } else { "black" } }
                }
                Link {
                    class: "flex flex-row gap-[10px] items-center",
                    to: route.switch_lang(),
                    icons::Language {}
                    span { class: "font-bold text-[16px]", "{tr.lang}" }
                }

                if user_wallet.is_logined() {
                    Link { to: Route::MyProfilePage { lang },
                        img {
                            class: "w-[28px] h-[28px] rounded-full",
                            src: asset!("/public/images/profile.png"),
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ExpandableMenu(
    expanded: bool,
    children: Element,
    name: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "relative w-full flex flex-col gap-[20px] items-start justify-center",
            div {
                class: "flex flex-row items-center justify-start text-[16px] font-bold gap-[10px] cursor-pointer",
                onclick,
                "{name}"
                SingleSimpleArrow { direction: if expanded { ArrowDirection::Up } else { ArrowDirection::Down } }
            }
            if expanded {
                div { class: "absolute top-[70px] left-0 w-full flex flex-col gap-[15px]",
                    {children}
                }
            }
        }
    }
}

#[component]
pub fn SubMenu(
    children: Element,
    #[props(into)] to: NavigationTarget,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        Link { to, class: "text-[14px] font-regular", onclick, {children} }
    }
}

#[component]
pub fn Menu(
    children: Element,
    #[props(into)] to: NavigationTarget,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        Link {
            to,
            class: "flex flex-row items-center justify-start text-[16px] font-bold gap-[10px]",
            onclick,
            {children}
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
        footer { id, class: "bg-gray-700 text-gray-500 w-full p-3 h-[120px]",
            div {
                id: "footer",
                class: "w-full flex flex-col justify-start items-start",
                img { src: "{LOGO_WHITE}", class: "w-[145px] h-[50px]" }
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
