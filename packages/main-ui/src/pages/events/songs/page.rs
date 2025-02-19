#![allow(non_snake_case)]
use crate::components::headings::Heading1;
use crate::components::icons::arrows::LeftArrow;
use crate::components::icons::arrows::RightArrow;
use crate::models::nft_metadata::NftMetadata;
use crate::models::songs::Song;
use crate::route::Route;

use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn SongsPage(lang: Language) -> Element {
    let mut ctrl = Controller::new()?;
    let tr: SongsTranslate = translate(&lang);
    let mut song: Signal<Option<String>> = use_signal(|| None);
    let page = ctrl.page();
    let size = 12;
    let songs = ctrl.songs();

    let start = page * size;
    let mut end = start + size;
    if end > songs.len() {
        end = songs.len();
    }

    rsx! {
        div {
            id: "songs",
            class: "w-full flex flex-col gap-[30px] items-center justify-center",
            Heading1 { lang, "{tr.title}" }
            div {
                class: "text-[16px] font-bold text-center",
                dangerous_inner_html: "{tr.description}",
            }

            div { class: "w-full flex flex-row gap-[10px] items-start justify-start px-[50px] font-bold text-[#666666]",
                "{songs.len()} {tr.results}"
            }

            div { class: "w-full flex flex-row items-center justify-center gap-[20px]",
                button {
                    class: "w-[50px] flex items-center justify-center",
                    onclick: move |_| {
                        if ctrl.page() > 0 {
                            ctrl.page.set(ctrl.page() - 1);
                        }
                    },
                    LeftArrow {}
                }
                div { class: "grow grid grid-cols-3 gap-[10px]",
                    if let Some(ref song) = song() {
                        video {
                            class: "w-0 h-0 hidden",
                            controls: true,
                            autoplay: true,
                            src: "{song}",
                        }
                    }
                    for i in start..end {
                        SongCard {
                            lang,
                            song: ctrl.songs()[i].clone(),
                            is_play: song().is_some() && song() == Some(ctrl.songs()[i].audio_url.clone()),
                            onplay: move |audio_url| {
                                tracing::debug!("play song: {audio_url}");
                                if let Some(ref s) = song() {
                                    if s == &audio_url {
                                        song.set(None);
                                        return;
                                    }
                                }
                                spawn(async move {
                                    let _ = ctrl.songs()[i].play().await;
                                    ctrl.songs.restart();
                                });
                                song.set(Some(audio_url));
                            },
                        }
                    }
                }

                button {
                    class: "w-[50px] flex items-center justify-center",
                    onclick: move |_| {
                        if end < songs.len() {
                            ctrl.page.set(ctrl.page() + 1);
                        }
                    },
                    RightArrow {}
                }
            }
        }
    }
}

#[component]
pub fn SongCard(
    lang: Language,
    song: Song,
    is_play: bool,
    onplay: EventHandler<String>,
) -> Element {
    let mut image_url = use_signal(|| song.image_url.clone());
    let nft_id = if song.nft_id.is_none() {
        "".to_string()
    } else {
        song.nft_id.unwrap_or_default().to_string()
    };
    let mut hover = use_signal(|| false);

    use_effect(use_reactive!(|song| {
        if song.nft_id.is_none() {
            return;
        }
        let nft_id = song.nft_id.unwrap().clone();
        spawn(async move {
            let image = NftMetadata::fetch(nft_id).await.unwrap_or_default().image;
            if image.is_empty() {
                return;
            }

            image_url.set(Some(image));
        });
    }));
    let person = asset!("/public/images/person.png");

    rsx! {
        Link {
            to: Route::SongsByIdPage {
                lang,
                id: song.key,
            },
            class: "col-span-1 flex flex-col rounded-[10px]",
            background: "linear-gradient(180deg, #848484 50.5%, #666666 70%, #313131 100%)",

            div { class: "flex flex-row gap-[10px] p-[10px] items-center",
                if let Some(ref image) = song.image_url {
                    div {
                        class: "relative w-[100px] min-w-[100px] h-[100px] rounded-[10px] cursor-pointer overflow-hidden",
                        onmouseenter: move |_| { hover.set(true) },
                        onmouseleave: move |_| { hover.set(false) },
                        onclick: move |evt| {
                            evt.prevent_default();
                            evt.stop_propagation();
                            tracing::debug!("play song");
                            onplay(song.audio_url.clone());
                        },
                        img {
                            class: "flex w-[100px] min-w-[100px] h-[100px] object-fill rounded-[10px]",
                            src: "{image}",
                        }

                        div {
                            class: "absolute top-0 left-0 w-full h-full flex items-center justify-center bg-black/30 z-[10]",
                            visibility: if hover() { "visible" } else { "hidden" },
                            img {
                                src: if is_play { asset!(
                                    "/public/images/stop.png", ImageAssetOptions::new().with_size(ImageSize::Manual {
                                    width : 50, height : 50 })
                                ) } else { asset!(
                                    "/public/images/play.png", ImageAssetOptions::new().with_size(ImageSize::Manual {
                                    width : 50, height : 50 })
                                ) },
                            }
                        }
                    }
                } else if let Some(ref image) = image_url() {
                    div {
                        class: "relative w-[100px] min-w-[100px] h-[100px] rounded-[10px] cursor-pointer overflow-hidden",
                        onmouseenter: move |_| { hover.set(true) },
                        onmouseleave: move |_| { hover.set(false) },
                        onclick: move |evt| {
                            evt.prevent_default();
                            evt.stop_propagation();
                            tracing::debug!("play song");
                            onplay(song.audio_url.clone());
                        },
                        img {
                            class: "flex w-[100px] min-w-[100px] h-[100px] object-fill rounded-[10px]",
                            src: "{image}",
                        }

                        div {
                            class: "absolute top-0 left-0 w-full h-full flex items-center justify-center bg-black/30 z-[10]",
                            visibility: if hover() { "visible" } else { "hidden" },
                            img {
                                src: if is_play { asset!(
                                    "/public/images/stop.png", ImageAssetOptions::new().with_size(ImageSize::Manual {
                                    width : 50, height : 50 })
                                ) } else { asset!(
                                    "/public/images/play.png", ImageAssetOptions::new().with_size(ImageSize::Manual {
                                    width : 50, height : 50 })
                                ) },
                            }
                        }
                    }
                } else {
                    div { class: "w-[100px] h-[100px] flex items-center justify-center",
                        span { class: "loading loading-spinner loading-md" }
                    }
                }

                div { class: "flex flex-col gap-[10px]",
                    p { class: "font-bold text-[16px] text-white overflow-hidden line-clamp-1",
                        "{song.title}"
                    }
                    div { class: "flex flex-row gap-[1px] line-clamp-1 text-white text-[14px]",
                        img {
                            src: "{person}",
                            class: "w-[20px] h-[20px] object-fit",
                        }
                        if nft_id.is_empty() {
                            span { "{song.nickname}" }
                        } else {
                            span { "#{nft_id}" }
                        }
                    }
                    div { class: "line-clamp-1 text-white text-[12px]", "{song.lyrics}" }
                }
            }

            div { class: "flex flex-row gap-[10px] justify-between px-[30px] py-[10px] text-white font-bold",
                div { class: "flex flex-row gap-[10px] items-center",
                    img {
                        src: asset!(
                            "/public/images/play_count.png", ImageAssetOptions::new()
                            .with_size(ImageSize::Manual { width : 15, height : 15 })
                        ),
                    }
                    span { "{song.play_count}" }
                }

                div { class: "flex flex-row gap-[10px] items-center",
                    img {
                        src: asset!(
                            "/public/images/like_green_outline.png", ImageAssetOptions::new()
                            .with_size(ImageSize::Manual { width : 15, height : 15 })
                        ),
                    }
                    span { "{song.like_count}" }
                }

                div { class: "flex flex-row gap-[10px] items-center",
                    img {
                        src: asset!(
                            "/public/images/share_green.png", ImageAssetOptions::new()
                            .with_size(ImageSize::Manual { width : 20, height : 20 })
                        ),
                    }

                }
            }
        }
    }
}
