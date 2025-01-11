#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum ArrowDirection {
    Up = 180,    // rotate-[180deg]
    Down = 0,    // rotate-[0deg]
    Left = 90,   // rotate-[90deg]
    Right = 270, // rotate-[270deg]
}

#[component]
pub fn SingleSimpleArrow(
    #[props(default ="single_simple_arrow_down".to_string())] id: String,
    #[props(default ="".to_string())] class: String,
    direction: ArrowDirection,
    #[props(default = 20)] size: u32,
    #[props(default = "black".to_string())] color: String,
) -> Element {
    let rotate = format!("rotate-[{}deg]", direction as u32);

    rsx! {
        svg {
            class: "transition-all {rotate} {class}",
            fill: "none",
            height: "{size}",
            "viewBox": "0 0 21 21",
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}",
            path {
                fill: "{color}",
                d: "M10.9693 11.2598L15.0944 7.13477L16.2727 8.31393L10.9693 13.6173L5.66602 8.31393L6.84435 7.1356L10.9693 11.2598Z",
            }
        }
    }
}
