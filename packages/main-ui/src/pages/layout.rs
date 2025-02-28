#![allow(non_snake_case)]
use super::{
    i18n::{FooterTranslate, HeaderTranslate},
    logout_popup::LogoutPopup,
};
use crate::{
    assets::*,
    components::icons,
    components::icons::arrows::{ArrowDirection, SingleSimpleArrow},
    pages::i18n::RootLayoutTranslate,
    route::Route,
    services::user_service::UserService,
};
use by_components::{
    loaders::cube_loader::CubeLoader, meta::MetaSeoTemplate, responsive::ResponsiveService,
    theme::ColorTheme,
};
use dioxus::prelude::*;
use dioxus_popup::{PopupService, PopupZone};
use dioxus_translate::*;

#[component]
pub fn RootLayout(lang: Language) -> Element {
    let theme: ColorTheme = use_context();
    let path: Route = use_route();
    let logo = asset!("/public/logos/logo_symbol_color.png");
    let tr: RootLayoutTranslate = translate(&lang);
    let responsive: ResponsiveService = use_context();

    rsx! {
        document::Title { "{tr.title}" }
        document::Meta { property: "og:title", content: "{tr.title}" }
        document::Meta { name: "description", content: "{tr.description}" }
        document::Meta { property: "og:description", content: "{tr.description}" }
        document::Meta { property: "og:type", content: "website" }

        MetaSeoTemplate {
            lang,
            logo_url: "{logo}",
            title: "Incheon Heroes",
            canonical: "https://incheonheroes.world",
            keywords: "Incheon, Heroes, 인천, 히어로즈, 유니버스",
            url: "{path}",
        }

        div {
            class: "flex flex-col w-full items-center justify-start min-h-[100vh] text-white",
            background: "{theme.background}",
            color: "{theme.text.primary}",
            if responsive.width() > 1200.0 {
                Header {
                    class: "w-full min-h-[70px] flex flex-row items-center justify-center bg-white",
                    lang,
                }
            } else {
                MobileHeader {
                    class: "w-full h-full flex flex-col items-center justify-start",
                    lang,
                }
            }
            PopupZone { background_color: "#ffffff", border_class: "" }
            div {
                class: "w-full max-w-[1440px] py-[70px] max-[1440px]:px-[20px]",
                min_height: "calc(100vh - 190px)",
                ErrorBoundary {
                    handle_error: move |errors: ErrorContext| {
                        let nav = use_navigator();
                        errors.clear_errors();
                        use_effect(move || {
                            nav.push(Route::HomePage { lang });
                        });
                        rsx! {
                        "Oops, we encountered an error. Please report {errors:?} to the developer of this application"
                        }
                    },

                    SuspenseBoundary {
                        fallback: |_| rsx! {
                            div { class: "absolute left-0 top-0 w-full h-full flex items-center justify-center",
                                CubeLoader {}
                            }
                        },

                        Outlet::<Route> {}
                    }
                }
            }

            if !path.should_hide_footer() {
                Footer { lang }
            }
        }
    }
}

#[component]
pub fn MobileHeader(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    lang: Language,
) -> Element {
    use crate::components::icons as c;
    use by_components::icons;
    let nav = use_navigator();
    let user_wallet: UserService = use_context();
    let route: Route = use_route();
    let mut expanded = use_signal(|| false);
    let handle_select_menu = move |_| {
        expanded.set(false);
    };
    let tr: HeaderTranslate = translate(&lang);

    rsx! {
        div {..attributes,
            div { class: "w-full flex flex-row items-center justify-between h-[70px] bg-white px-[20px]",
                button {
                    onclick: move |_| {
                        expanded.set(!expanded());
                    },
                    icons::alignments::AlignJustify { class: "w-[30px] h-[30px] text-black" }
                }
                Link {
                    class: "flex items-center justify-center h-[70px] z-[1] max-[400px]:hidden",
                    to: Route::HomePage { lang },
                    img {
                        src: "{LOGO}",
                        class: "w-[145px] max-[500px]:w-[100px]",
                    }
                }
                div { class: "flex flex-row gap-[15px] items-center h-full z-[1]",
                    button {
                        onclick: move |_| {
                            nav.push(Route::ConnectPage { lang });
                        },
                        c::Connect { fill: if user_wallet.is_logined() { "#CEF7E3" } else { "black" } }
                    }
                    Link {
                        class: "flex flex-row gap-[10px] items-center",
                        to: route.switch_lang(),
                        c::Language {}
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
            if expanded() {
                div { class: "absolute top-[70px] left-0 w-full h-[calc(100vh-70px)] grow bg-white flex flex-col items-center text-black",
                    div {
                        id: "menus",
                        class: "w-full flex flex-col justify-start h-[70px] pl-[12px]",
                        ExpandableMenu { expanded: true, name: "{tr.history}",
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

                        ExpandableMenu { expanded: true, name: "{tr.event}",
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
                        Menu {
                            to: Route::ContentsPage { lang },
                            onclick: handle_select_menu,
                            "{tr.contents}"
                        }
                        if user_wallet.is_logined() {
                            Menu {
                                to: Route::MyNftsPage { lang },
                                onclick: handle_select_menu,
                                "{tr.my_nfts}"
                            }
                        }
                        ExpandableMenu { expanded: true, name: "{tr.info}",
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
                }
            }
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
    let mut popup: PopupService = use_context();

    let handle_select_menu = move |_| {
        expanded.set(false);
    };

    rsx! {
        div { id, class,
            if expanded() {
                div { class: "absolute top-0 left-0 w-full h-[200px] bg-white z-[9]" }
            }
            div { class: "w-full flex flex-row items-center justify-center max-w-[1440px] max-[1440px]:px-[20px] z-[10]",
                Link {
                    class: "flex items-center justify-center h-[70px] z-[1] max-[400px]:hidden",
                    to: Route::HomePage { lang },
                    img {
                        src: "{LOGO}",
                        class: "w-[145px] h-[50px] object-contain",
                    }
                }

                div { class: "w-full flex flex-col items-center justify-center px-[100px] max-[1440px]:px-[50px]",
                    div {
                        id: "menus",
                        class: "w-full flex flex-row justify-start h-[70px] justify-between",
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
                        Menu {
                            to: Route::ContentsPage { lang },
                            onclick: handle_select_menu,
                            "{tr.contents}"
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

                div { class: "flex flex-row gap-[15px] items-center h-full z-[1]",
                    button {
                        onclick: move |_| {
                            if user_wallet.is_logined() {
                                tracing::debug!("logout popup clicked");
                                popup.open(rsx! {
                                    LogoutPopup { lang: lang.clone() }
                                }).with_id("logout_popup");
                            } else {
                                nav.push(Route::ConnectPage {
                                    lang: lang.clone(),
                                });
                            }
                        },
                        if user_wallet.is_logined() {
                            icons::Logout { fill: "black" }
                        } else {
                            icons::Connect { fill: "black" }
                        }
                    }
                    Link {
                        class: "flex flex-row gap-[10px] items-center",
                        to: route.switch_lang(),
                        icons::Language {}
                        span { class: "font-bold text-[16px]", "{tr.lang}" }
                    }


                    if user_wallet.is_logined() {
                        Link {
                            to: Route::MyProfilePage { lang },
                            class: "w-[28px]",
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
}

#[component]
pub fn ExpandableMenu(
    expanded: bool,
    children: Element,
    name: String,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let responsive: ResponsiveService = use_context();
    let mut mobile_expanded = use_signal(|| true);
    rsx! {
        div { class: "relative flex flex-col items-start justify-center",
            div {
                class: "h-[70px] w-full flex flex-row items-center justify-start text-[16px] font-bold gap-[10px] cursor-pointer z-[100]",
                onclick: move |e| {
                    if responsive.width() <= 1200.0 {
                        mobile_expanded.set(!mobile_expanded());
                    }
                    if let Some(onclick) = onclick {
                        onclick(e);
                    }
                },
                "{name}"
                SingleSimpleArrow { direction: if responsive.width() > 1200.0 { if expanded { ArrowDirection::Up } else { ArrowDirection::Down } } else { if mobile_expanded() { ArrowDirection::Up } else { ArrowDirection::Down } } }
            }
            if expanded {
                if responsive.width() > 1200.0 {
                    div { class: "absolute top-[70px] left-0 w-full flex flex-col gap-[15px] z-[100]",
                        {children}
                    }
                } else {
                    if mobile_expanded() {
                        div { class: "w-full flex flex-col gap-[10px] py-[5px] pl-[18px] z-[100]",
                            {children}
                        }
                    }
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
        div { class: "flex flex-col items-start justify-center",
            Link {
                to,
                class: "h-[70px] w-full flex flex-row items-center justify-start text-[16px] font-bold gap-[10px]",
                onclick,
                {children}
            }
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
