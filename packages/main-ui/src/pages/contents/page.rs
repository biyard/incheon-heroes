#![allow(non_snake_case)]
use crate::route::Route;

use super::controller::*;
use super::i18n::*;
use by_components::icons;
use dioxus::prelude::*;
use dioxus_translate::*;
use dto::ContentSorter;
use dto::ContentSummary;

#[component]
pub fn ContentsPage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: ContentsTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: "{tr.title}" }

        div { id: "contents", class: "w-full flex flex-col gap-[50px]",
            SearchBar {
                class: "w-full bg-white h-[48px] rounded-[12px]",
                onsearch: move |query| async move {
                    ctrl.search(query).await;
                },
            }

            div { class: " w-full flex flex-row justify-end items-center",
                details { class: "dropdown",
                    summary { class: "btn m-1 text-[#16775D] w-[117px] bg-transparent border-[#16775D] flex flex-row justify-center items-center hover:bg-[#E4F4E4]",

                        {ctrl.sorter().translate(&lang)}
                        icons::arrows::ChevronDown { color: "#16775D", width: "12", height: "12" }
                    }
                    ul {
                        class: "menu dropdown-content bg-white rounded-[12px] z-[1] w-[117px] shadow overflow-hidden",
                        padding: "0px",
                        for option in ContentSorter::VARIANTS {
                            li {
                                class: "hover:bg-[#E4F4E4] px-[20px] py-[15px] cursor-pointer overflow-hidden",
                                role: "button",
                                onclick: move |_| {
                                    ctrl.sorter.set(*option);
                                },
                                "{option.translate(&lang)}"
                            }
                        }
                    }
                }
            }

            ColGridCards { lang, contents: ctrl.contents()?.items }

            if ctrl.user_service.is_logined() {
                CreateNftButton { lang, label: "{tr.btn_create}" }
            }
        } // end of this page
    }
}

#[component]
pub fn ColGridCards(lang: Language, contents: ReadOnlySignal<Vec<ContentSummary>>) -> Element {
    use by_components::responsive::ResponsiveService;

    let responsive: ResponsiveService = use_context();
    let cols_contents = use_memo(move || {
        let width = responsive.width();
        let cols: usize = if width >= 1140.0 {
            4
        } else if width >= 640.0 {
            3
        } else if width >= 340.0 {
            2
        } else {
            1
        };

        let mut cols_contents = vec![vec![]; cols];
        for (i, content) in contents().iter().enumerate() {
            cols_contents[i % cols].push(content.clone());
        }
        cols_contents
    });

    rsx! {
        div { class: "w-full grid grid-cols-4 max-[1200px]:grid-cols-3 max-[700px]:grid-cols-2 max-[400px]:grid-cols-1 gap-[24px]",
            for contents in cols_contents().iter() {
                div { class: "w-full col-span-1 flex flex-col justify-start gap-[24px]",
                    for content in contents.iter() {
                        ContentCard {
                            to: Route::ContentsByIdPage {
                                lang,
                                id: content.id,
                            },
                            class: "w-full",
                            content: content.clone(),
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ContentCard(
    #[props(into)] to: NavigationTarget,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    content: ContentSummary,
    children: Element,
) -> Element {
    rsx! {
        div {..attributes,
            Link { class: "", to,
                img {
                    src: "{content.thumbnail_image}",
                    alt: "{content.title}",
                    class: "w-full object-cover bg-white rounded-[12px]",
                }
            }
        }
    }
}

#[component]
pub fn SearchBar(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    onsearch: EventHandler<String>,
    #[props(default = "#979797".to_string())] color: String,
) -> Element {
    rsx! {
        div {..attributes,
            div { class: "relative w-full h-full",
                input {
                    class: "w-full h-full px-[20px] text-[#16775D] bg-transparent font-semibold focus:outline-none",
                    color: "{color}",
                    placeholder: "Search",
                    oninput: move |e| onsearch(e.value()),
                }

                div { class: "absolute top-0 right-[10px] h-full flex items-center justify-center",
                    icons::edit::Search { color: "{color}" }
                }
            }
        }
    }
}

#[component]
pub fn CreateNftButton(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    lang: Language,
    label: String,
) -> Element {
    let mut hover = use_signal(|| false);
    let w = if hover() { "" } else { "w-[0px]" };

    rsx! {
        div { class: "fixed bottom-[40px] right-[40px]", ..attributes,
            Link { to: Route::NewContentsPage { lang },
                div {
                    class: "transition-all duration-500 ease-in-out flex flex-row items-center justify-center h-[58px] w-[58px] px-[23px] bg-white hover:bg-[#E4F4E4] hover:border-[1px] hover:border-[#16775D] hover:px-[35px] hover:gap-[16px] rounded-full cursor-pointer overflow-hidden hover:w-[200px]",
                    box_shadow: "0px 4px 30px rgba(84, 157, 159, 0.8)",
                    onmouseenter: move |_| hover.set(true),
                    onmouseleave: move |_| hover.set(false),
                    span { class: "transition-all {w} overflow-hidden text-[#16775D] font-semibold",
                        "{label}"
                    }
                    Plus { color: if hover() { "#16775D" } else { "#5B5B5B" } }
                }
            }
        }
    }
}

#[component]
pub fn Plus(#[props(default = "#5B5B5B".to_string())] color: String) -> Element {
    rsx! {
        svg {
            fill: "none",
            height: "15",
            view_box: "0 0 14 15",
            width: "14",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M1 7.96875L7 7.96875M7 7.96875L13 7.96875M7 7.96875V1.96875M7 7.96875L7 13.9688",
                stroke: "{color}",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "2",
            }
        }
    }
}
