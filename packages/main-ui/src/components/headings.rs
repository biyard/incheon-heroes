#![allow(non_snake_case)]
use crate::assets::*;
use dioxus::prelude::*;
use dioxus_translate::Language;

#[component]
pub fn Heading1(
    #[props(default = true)] with_symbol: bool,
    lang: Language,
    children: Element,
) -> Element {
    let font_family = match lang {
        Language::Ko => "Pretendard",
        Language::En => "Russo One",
    };

    rsx! {
        div {
            class: "flex flex-row gap-[20px] text-center text-[56px] max-[350px]:text-[24px] max-[500px]:text-[32px] max-[1000px]:text-[40px] font-black leading-[64px] [text-shadow:_0px_0px_12px_rgb(22_119_93_/_1.00)] justify-center items-center",
            font_family,
            if with_symbol {
                img {
                    src: "{DIAMOND}",
                    class: "w-[50px] h-[50px] max-[350px]:w-[25px] max-[350px]:h-[25px]",
                }
            }

            h1 { {children} }

            if with_symbol {
                img {
                    src: "{DIAMOND}",
                    class: "w-[50px] h-[50px] max-[350px]:w-[25px] max-[350px]:h-[25px]",

                }
            }

        }
    }
}
