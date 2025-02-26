#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn CompleteIcon(#[props(default = 80)] size: i32) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 80 81",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            g { id: "Component 7",
                path {
                    id: "Star 1",
                    d: "M21.8612 25.8458C31.3035 14.5235 48.6965 14.5235 58.1388 25.8458C59.8726 27.9248 61.236 30.2864 62.1696 32.8274C67.2539 46.6658 58.5574 61.7287 44.0308 64.2447C41.3635 64.7067 38.6365 64.7067 35.9692 64.2447C21.4426 61.7287 12.7461 46.6658 17.8304 32.8274C18.764 30.2864 20.1274 27.9248 21.8612 25.8458Z",
                    stroke: "#24B38C",
                    stroke_width: "4",
                }
                path {
                    id: "Rectangle 14",
                    d: "M28.9629 41.2455L36.1421 48.4247L51.0433 33.5234",
                    stroke: "#24B38C",
                    stroke_width: "4",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
            }
        }
    }
}
