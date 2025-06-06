#![allow(non_snake_case)]
use by_components::theme::ColorTheme;
use dioxus::prelude::*;

use dioxus_popup::{Close, PopupService};

#[component]
pub fn PopupZone(
    #[props(default = "".to_string())] background_color: String,
    #[props(default = "border-[#292B3C] border-[1px]".to_string())] border_class: String,
) -> Element {
    let mut popup: PopupService = use_context();
    let mut hover_close = use_signal(|| false);
    let color_theme = try_use_context::<ColorTheme>().unwrap_or_default();

    rsx! {
        div {
            class: format!(
                "{}",
                match popup.is_opened() {
                    true => {
                        "fixed top-0 left-0 w-screen h-screen bg-black bg-opacity-50 flex justify-center items-center backdrop-blur-[4px] bg-black/25 z-[101]"
                    }
                    false => "hidden",
                },
            ),
            onclick: move |_| {
                popup.close();
            },
            if popup.is_opened() {
                div {
                    class: format!(
                        "relative rounded-[12px] {} min-w-[350px] max-[500px]:w-full max-[500px]:mx-[20px] justify-center items-center",
                        border_class,
                    ),
                    background: if background_color == "" { "{color_theme.popup.background}" } else { background_color },
                    onclick: move |e| {
                        e.stop_propagation();
                    },
                    if (popup.close)() {
                        button {
                            class: "absolute top-[15px] right-[10px] rounded-[4px] cursor-pointer",
                            background: if hover_close() { "{color_theme.button.primary}" } else { "transparent" },
                            onclick: move |_| {
                                popup.close();
                            },
                            onmouseenter: move |_| {
                                hover_close.set(true);
                            },
                            onmouseleave: move |_| {
                                hover_close.set(false);
                            },
                            Close { color: "white" }
                        }
                    }
                    div {
                        id: popup.get_id(),
                        class: "flex flex-col items-center justify-center gap-[25px]",
                        match popup.get_title() {
                            Some(title) => {
                                rsx! {
                                    div {
                                        class: "w-full text-center text-white font-bold text-[18px] rounded-t-[8px] py-[15px]",
                                        style: "background-color: #32c564;",
                                        "{title}"
                                    }
                                }
                            }
                            None => rsx! {},
                        }
                        {popup.render()}
                    }
                }
            }
        }
    }
}
