#![allow(non_snake_case)]
use crate::assets::*;
use dioxus::prelude::*;

#[component]
pub fn Heading1(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(default = true)] with_symbol: bool,
    children: Element,
) -> Element {
    rsx! {
        div { class: "flex flex-row gap-[20px] text-center text-black text-7xl font-normal font-['Russo_One'] leading-[80px] [text-shadow:_0px_0px_12px_rgb(22_119_93_/_1.00)]",
            if with_symbol {
                img { src: "{DIAMOND}", class: "w-[50px] h-[50px]" }
            }

            div { ..attributes,{children} }

            if with_symbol {
                img { src: "{DIAMOND}", class: "w-[50px] h-[50px]" }
            }

        }
    }
}
