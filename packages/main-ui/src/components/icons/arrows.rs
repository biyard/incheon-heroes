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
            class: "transition-all {rotate} {class} cursor-pointer",
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

#[component]
pub fn LeftArrow() -> Element {
    rsx! {
        svg {
            class: "cursor-pointer",
            fill: "none",
            height: "50",
            view_box: "0 0 18 50",
            width: "18",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M15.375 3.41797L2.625 25.0013L15.375 46.5846",
                stroke: "#666666",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "5",
            }
        }
    }
}

#[component]
pub fn RightArrow() -> Element {
    rsx! {
        svg {
            class: "cursor-pointer",
            fill: "none",
            height: "50",
            view_box: "0 0 18 50",
            width: "18",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M2.625 46.582L15.375 24.9987L2.625 3.41537",
                stroke: "#666666",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "5",
            }
        }
    }
}
