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

            div { class: "w-full max-w-[700px] flex flex-row gap-[20px] items-start bg-[#FAFAFA] rounded-[10px] p-[10px]",

                div { class: "w-[300px] flex flex-col gap-[10px] items-center",
                    Image {
                        image: song.image_url.unwrap(),
                        is_play: ctrl.is_playing(),
                        onplay: move |_| { ctrl.is_playing.set(!ctrl.is_playing()) },
                    }

                    div { class: "w-full grid grid-cols-2 gap-[10px] p-[10px]  text-[14px] text-[#666666]",
                        div { class: "col-span-1 flex flex-row gap-[10px] items-center justify-center w-full",
                            img {
                                class: "w-[20px] h-[20px]",
                                src: asset!("/public/images/play_count.png"),
                            }
                            span { "{song.play_count}" }
                        }

                        div { class: "col-span-1 flex flex-row gap-[10px] items-center justify-center w-full",
                            img {
                                src: asset!(
                                    "/public/images/like_green.png", ImageAssetOptions::new()
                                    .with_size(ImageSize::Manual { width : 20, height : 20 })
                                ),
                            }
                            span { "{song.like_count}" }
                        }
                    }


                    div { class: "w-full grid grid-cols-2 gap-[10px] p-[10px]  text-[14px] text-white",

                        button {
                            class: "col-span-1 w-full rounded-[5px] bg-[#32C564] p-[10px] flex flex-row gap-[10px] items-center justify-center",
                            box_shadow: "0px 4px 4px rgba(0, 0, 0, 0.25)",
                            img {
                                src: asset!(
                                    "/public/images/share_white.png", ImageAssetOptions::new()
                                    .with_size(ImageSize::Manual { width : 20, height : 20 })
                                ),
                            }
                            "{tr.share}"
                        }

                        button {
                            class: "col-span-1 w-full rounded-[5px] bg-[#32C564] p-[10px] flex flex-row gap-[10px] items-center justify-center",
                            box_shadow: "0px 4px 4px rgba(0, 0, 0, 0.25)",
                            img {
                                src: asset!(
                                    "/public/images/like_white.png", ImageAssetOptions::new()
                                    .with_size(ImageSize::Manual { width : 20, height : 20 })
                                ),
                            }
                            "{tr.like}"
                        }
                    }
                } // Left Section

                div { class: "col-span-1 w-full flex flex-col items-start justify-start text-[#666666] gap-[40px]",
                    div { class: "flex flex-col gap-[10px]",
                        span { class: " font-bold text-[24px]", "{song.title}" }
                        div { class: "flex flex-row gap-[10px] items-center",
                            img {
                                class: "w-[25px] h-[25px]",
                                src: asset!("/public/images/person.png"),
                            }
                            if let Some(ref nft_id) = song.nft_id {
                                span { "{nft_id} / {song.nickname}" }
                            } else {
                                span { "{song.nickname}" }
                            }
                        }
                    }

                    div { class: "w-full flex flex-col gap-[10px]",
                        span { class: " font-bold text-[24px]", "{tr.lyrics}" }
                        pre { class: "w-full max-h-[470px] text-[14px] overflow-y-scroll",
                            "{song.lyrics}"
                        }
                    }
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
