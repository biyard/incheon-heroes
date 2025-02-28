use dioxus::prelude::*;

#[component]
pub fn HeartIcon(
    #[props(default = "#979797".to_string())] color: String,
    #[props(default = "none".to_string())] fill: String,
) -> Element {
    rsx! {
        svg {
            width: "24",
            height: "25",
            view_box: "0 0 24 25",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M4.42602 13.2841L12 20.8581L19.574 13.2841C21.4753 11.3828 21.4753 8.30004 19.574 6.39868C17.6726 4.49732 14.5899 4.49732 12.6885 6.39868L12 7.08722L11.3115 6.39868C9.4101 4.49732 6.32738 4.49732 4.42602 6.39868C2.52466 8.30004 2.52466 11.3828 4.42602 13.2841Z",
                fill: "{fill}",
                stroke: "{color}",
                stroke_width: "1.5",
                stroke_linejoin: "round",
            }
        }
    }
}
