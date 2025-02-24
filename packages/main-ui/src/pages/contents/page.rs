#![allow(non_snake_case)]
use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn ContentsPage(lang: Language) -> Element {
    let mut _ctrl = Controller::new(lang)?;
    let tr: ContentsTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: "{tr.title}" }

        div { id: "contents", class: "flex flex-col",
            "{tr.title} PAGE"
            CreateNftButton {}
        } // end of this page
    }
}
#[component]
pub fn CreateNftButton(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    label: String,
) -> Element {
    let mut hover = use_signal(|| false);

    rsx! {
        div {
            class: "absolute fixed bottom-[40px] right-[40px] rounded-full",
            ..attributes,
            div {
                class: "transition-all flex flex-row items-center justify-center p-[23px] bg-white hover:bg-[#E4F4E4] hover:border-[1px] hover:border-[#16775D] gap-[16px]",
                onmouseenter: move |_| hover.set(true),
                onmouseleave: move |_| hover.set(false),
                if hover() {
                    span { "{label}" }
                }
                Plus {}

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
