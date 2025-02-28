use dioxus::prelude::*;

#[component]
pub fn SendIcon(#[props(default = "#979797".to_string())] color: String) -> Element {
    rsx! {
        svg {
            width: "19",
            height: "20",
            view_box: "0 0 19 20",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            g {
                path {
                    d: "M4.80467 4.75522L12.843 2.07578C16.4503 0.873345 18.4102 2.84269 17.2172 6.45L14.5378 14.4883C12.7389 19.8945 9.78485 19.8945 7.98593 14.4883L7.19061 12.1024L4.80467 11.3071C-0.601557 9.50813 -0.601557 6.56361 4.80467 4.75522Z",
                    stroke: "{color}",
                    stroke_width: "1.5",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
                path {
                    d: "M7.37109 11.692L10.7606 8.29297",
                    stroke: "{color}",
                    stroke_width: "1.5",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
            }
        }
    }
}
