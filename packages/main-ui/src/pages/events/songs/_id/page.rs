#![allow(non_snake_case)]
use crate::components::headings::Heading1;

use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn SongsByIdPage(id: String, lang: Language) -> Element {
    let mut ctrl = Controller::new(id)?;
    let tr: SongsByIdTranslate = translate(&lang);
    let song = ctrl.song();

    rsx! {
        div { id: "songs-by-id", class: "flex flex-col gap-[50px] items-center",
            if ctrl.is_playing() {
                video {
                    class: "w-0 h-0 hidden",
                    controls: true,
                    autoplay: true,
                    src: "{song.audio_url}",
                }
            }
            Heading1 { lang, "{tr.title}" }
            div { class: "bg-[#FAFAFA] rounded-[10px] p-[10px]",
                Image {
                    image: song.image_url.unwrap(),
                    is_play: ctrl.is_playing(),
                    onplay: move |_| { ctrl.is_playing.set(!ctrl.is_playing()) },
                }
            }
        }
    }
}

#[component]
pub fn Image(image: String, is_play: bool, onplay: EventHandler<()>) -> Element {
    let mut hover = use_signal(|| false);

    rsx! {
        div { class: "flex flex-row gap-[10px] p-[10px] items-center",
            div {
                class: "relative w-[300px] min-w-[300px] h-[300px] rounded-[10px] cursor-pointer overflow-hidden",
                onmouseenter: move |_| { hover.set(true) },
                onmouseleave: move |_| { hover.set(false) },
                onclick: move |_| {
                    tracing::debug!("play song");
                    onplay(());
                },
                img {
                    class: "flex w-full min-w-full h-full object-fill rounded-[10px]",
                    src: "{image}",
                }

                div {
                    class: "absolute top-0 left-0 w-full h-full flex items-center justify-center bg-black/30 z-[10]",
                    visibility: if hover() { "visible" } else { "hidden" },
                    img {
                        src: if is_play { asset!(
                            "public/images/stop.png", ImageAssetOptions::new().with_size(ImageSize::Manual {
                            width : 50, height : 50 })
                        ) } else { asset!(
                            "public/images/play.png", ImageAssetOptions::new().with_size(ImageSize::Manual {
                            width : 50, height : 50 })
                        ) },
                    }
                }
            }
        }
    }
}
