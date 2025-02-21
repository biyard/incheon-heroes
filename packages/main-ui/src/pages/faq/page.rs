#![allow(non_snake_case)]
use crate::components::headings::Heading1;

use super::controller::*;
use super::i18n::*;
use by_components::meta::MetaPage;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn FaqPage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: FaqTranslate = translate(&lang);

    rsx! {
        MetaPage { title: "{tr.title}" }
        div {
            id: "faq",
            class: "flex flex-col gap-[80px] items-center justify-start",
            Heading1 { lang, "{tr.title}" }
            if let Ok(faqs) = ctrl.faqs() {
                div { class: "w-full max-w-[900px] flex flex-col border-t-[1px] border-black/40",
                    for (i , faq) in faqs.iter().enumerate() {
                        FaqComponent {
                            faq: faq.clone(),
                            expanded: ctrl.selected().is_some() && ctrl.selected() == Some(i),
                            onclick: move |_| {
                                if ctrl.selected() == Some(i) {
                                    ctrl.selected.set(None);
                                } else {
                                    ctrl.selected.set(Some(i));
                                }
                            },
                        }
                    }
                }
            } else {
                "Loading..."
            }
        }
    }
}

#[component]
pub fn FaqComponent(faq: Faq, expanded: bool, onclick: EventHandler<()>) -> Element {
    rsx! {
        div {
            id: "faq",
            class: "flex flex-col w-full border-b-[1px] border-black/40 cursor-pointer p-[40px] gap-[40px]",
            onclick: move |_| {
                onclick(());
            },
            div { class: "flex flex-row items-center gap-[20px]",
                div { class: "w-[40px] flex flex-col items-center justify-center",
                    Q { fill: if expanded { "#16775D" } }
                }
                div {
                    class: "grow w-full text-[20px] font-bold",
                    color: if expanded { "#16775D" },
                    "{faq.question}"
                }
            }
            if expanded {
                div { class: "flex flex-row items-center gap-[20px]",
                    div { class: "w-[40px] flex flex-col items-center justify-center",
                        A {}
                    }
                    div { class: "grow text-[20px] font-normal", "{faq.answer}" }
                }
            }
        }
    }
}

#[component]
pub fn Q(fill: Option<String>) -> Element {
    let fill = fill.unwrap_or_else(|| "black".to_string());

    rsx! {
        svg {
            fill: "none",
            height: "31",
            view_box: "0 0 28 31",
            width: "28",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M11.4229 18.2735H17.2793L18.771 20.1519C19.6273 18.7983 20.0831 16.9199 20.0831 14.5304C20.0831 9.32321 17.9284 6.49171 14.0472 6.49171C10.166 6.49171 8.01131 9.32321 8.01131 14.5304C8.01131 19.7376 10.166 22.5691 14.0472 22.5691C14.2544 22.5691 14.4616 22.5553 14.655 22.5414L11.4229 18.2735ZM0.359375 14.5304C0.359375 5.1105 6.34004 0 14.0472 0C21.6992 0 27.7351 5.1105 27.7351 14.5304C27.7351 19.4337 26.1052 23.163 23.4533 25.6215L27.7213 30.8011H21.0914L18.9505 28.2873C17.4312 28.7983 15.7737 29.0608 14.0472 29.0608C6.34004 29.0608 0.359375 23.895 0.359375 14.5304Z",
                fill: "{fill}",
            }
        }
    }
}

#[component]
pub fn A() -> Element {
    rsx! {
        svg {
            fill: "none",
            height: "53",
            view_box: "0 0 54 53",
            width: "54",
            xmlns: "http://www.w3.org/2000/svg",
            g {
                path {
                    d: "M20.3842 40.2873H12.3594L21.7516 12H31.876L41.2682 40.2873H33.2434L31.5445 34.7348H22.0831L20.3842 40.2873ZM23.7958 29.1271H29.8318L26.9174 19.6105H26.6964L23.7958 29.1271Z",
                    fill: "black",
                }
            }
        }
    }
}
