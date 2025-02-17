#![allow(non_snake_case)]
use crate::assets::*;
use dioxus::prelude::*;

#[component]
pub fn Heading1(#[props(default = true)] with_symbol: bool, children: Element) -> Element {
    rsx! {
        div { class: "flex flex-row gap-[20px] text-center text-black text-[56px] font-normal font-['Russo_One'] leading-[64px] [text-shadow:_0px_0px_12px_rgb(22_119_93_/_1.00)] justify-center items-center",
            if with_symbol {
                img { src: "{DIAMOND}", class: "w-[50px] h-[50px]" }
            }

            div { {children} }

            if with_symbol {
                img { src: "{DIAMOND}", class: "w-[50px] h-[50px]" }
            }

        }
    }
}
