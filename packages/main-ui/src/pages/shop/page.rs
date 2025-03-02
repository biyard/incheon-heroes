#![allow(non_snake_case)]
use crate::components::headings::Heading1;
use crate::route::Route;

use super::controller::*;
use super::i18n::*;
use by_components::responsive::ResponsiveService;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn ShopPage(lang: Language) -> Element {
    let ctrl = Controller::new()?;
    let tr: ShopTranslate = translate(&lang);
    let responsive: ResponsiveService = use_context();
    let stacks = vec![
        (
            tr.usb_title,
            tr.usb_desc,
            "https://metadata.biyard.co/incheon-universe/shop/shop_banner_3.png",
        ),
        (
            tr.magnet_title,
            tr.magnet_desc,
            "https://metadata.biyard.co/incheon-universe/shop/shop_banner_4.png",
        ),
        (
            tr.notebook_title,
            tr.notebook_desc,
            "https://metadata.biyard.co/incheon-universe/shop/shop_banner_5.png",
        ),
        (
            tr.legendary_title,
            tr.legendary_desc,
            "https://metadata.biyard.co/incheon-universe/shop/shop_banner_1.png",
        ),
        (
            tr.rare_title,
            tr.rare_desc,
            "https://metadata.biyard.co/incheon-universe/shop/shop_banner_2.png",
        ),
    ]
    .iter()
    .map(|(title, desc, img)| {
        rsx! {
            div { class: "relative w-full flex flex-col gap-[10px]",
                img { class: "w-full h-[450px] object-cover", src: "{img}" }
                if responsive.width() > 1200.0 {
                    div { class: "absolute bottom-0 left-0  w-full bg-black bg-opacity-50 p-[20px] flex flex-row justify-between items-center",
                        p {
                            class: "text-white text-[25px] font-bold",
                            style: "max-width: calc(100% - 50px); white-space: pre-wrap; overflow-wrap: break-word;",
                            "{title}"
                        }
                        p { class: "text-white",
                            pre { class: "max-w-full overflow-auto whitespace-pre-wrap text-white text-[12px]",
                                "{desc}"
                            }
                        }
                    }
                } else {
                    div { class: "absolute bottom-0 left-0  w-full bg-black bg-opacity-50 p-[20px] flex flex-col gap-[10px]",
                        p {
                            class: "text-white text-[25px] font-bold",
                            style: "max-width: calc(100% - 50px); white-space: pre-wrap; overflow-wrap: break-word;",
                            "{title}"
                        }
                        p { class: "text-white",
                            pre { class: "max-w-full overflow-auto whitespace-pre-wrap text-white text-[12px]",
                                "{desc}"
                            }
                        }
                    }
                }
            }
        }
    })
    .collect::<Vec<Element>>();

    rsx! {
        by_components::meta::MetaPage { title: "{tr.title}", description: "{tr.desc}" }
        div {
            id: "shop",
            class: "flex flex-col gap-[50px] items-center justify-center w-full",
            Heading1 { lang, "{tr.title}" }
            div { class: "break-all flex flex-col gap-[10px] text-center font-bold text-[16px] max-[500px]:text-[10px]",
                pre { "{tr.desc}" }
                p { class: "text-[12px] max-[500px]:text-[8px] font-semibold", "{tr.sub_desc}" }
            }
            HorizontalSlide {
                lang,
                left: rsx! {
                    img { class: "w-[50px]", src: asset!("/public/images/back.png") }
                },
                right: rsx! {
                    img { class: "w-[50px]", src: asset!("/public/images/forward.png") }
                },
                stacks,
            }

            div { class: "grid grid-cols-4 max-[1000px]:grid-cols-3 max-[850px]:grid-cols-2 max-[500px]:grid-cols-1 p-[20px] gap-[20px] justify-start items-start bg-white/60",
                for (i , item) in ctrl.items()?.iter().enumerate() {
                    ShopItemCard {
                        likes: item.likes.as_u64(),
                        image: item.image.clone(),
                        name: match lang {
                            Language::Ko => item.name_ko.clone(),
                            Language::En => item.name_en.clone(),
                        },
                        price: item.price.as_u64(),
                        id: item.id.to_string(),
                        level: item.level,
                        lang,
                        remaining_quantity: item.remaining.as_u64(),
                        onbuy: move |_| async move {
                            ctrl.handle_buy(i).await;
                        },
                        onlike: move |_| async move {
                            ctrl.handle_like(i).await;
                        },
                        disable_like: !ctrl.user.is_logined() || ctrl.liked_items()?.contains(&item.id.as_u64()),
                        disable_buy: !ctrl.user.is_logined() || ctrl.user.get_account_exp() < item.price.as_u64(),
                    }
                }
            }
        }
    }
}

#[component]
pub fn HorizontalSlide(
    lang: Language,
    left: Element,
    right: Element,
    stacks: Vec<Element>,
) -> Element {
    let mut current_slide = use_signal(|| 0);
    let stack_count = stacks.len();
    rsx! {
        div { class: "carousel w-full",
            for (i , stack) in stacks.iter().enumerate() {
                div {
                    class: "carousel-item relative w-full",
                    style: if i == current_slide() { "display: block" } else { "display: none" },
                    id: format!("slide{}", i),
                    {stack}
                    div { class: "absolute left-5 right-5 top-1/2 flex -translate-y-1/2 transform justify-between",
                        button {
                            class: "btn-ghost rounded-full p-[10px]",
                            onclick: move |_| {
                                current_slide.set((i + stack_count - 1) % stack_count);
                            },
                            {left.clone()}
                        }
                        button {
                            class: "btn-ghost rounded-full p-[10px]",
                            onclick: move |_| {
                                current_slide.set((i + 1) % stack_count);
                            },
                            {right.clone()}
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ShopItemCard(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    likes: u64,
    image: String,
    name: String,
    price: u64,
    lang: Language,
    remaining_quantity: u64,
    id: String,
    level: u8,
    onbuy: EventHandler<()>,
    onlike: EventHandler<()>,
    disable_like: bool,
    disable_buy: bool,
    children: Element,
) -> Element {
    let tr = translate::<ShopItemCardTranslate>(&lang);
    let color = match level {
        4 => "#BA2929",
        3 => "#2168C3",
        _ => "#029F75",
    };
    let mut hover = use_signal(|| false);
    let responsive: ResponsiveService = use_context();

    rsx! {
        div {..attributes,
            Link {
                class: "relative w-full h-full",
                to: Route::ShopByIdPage { lang, id },

                // Card
                div {
                    class: "flex flex-col rounded-[8px] overflow-hidden shadow-lg bg-white w-full h-full",
                    onmouseenter: move |_| hover.set(true),
                    onmouseleave: move |_| hover.set(false),
                    img { class: "w-full object-fill", src: "{image}" }
                    div { class: "flex flex-col gap-[10px]",
                        p {
                            class: "p-[10px] text-[14px] font-bold",
                            color,
                            "{name}"
                        }
                        if responsive.width() > 1200.0 {
                            div { class: "px-[10px] flex flex-row justify-between items-center gap-[10px] ",
                                p { class: "text-[#16775D] font-semibold text-[12px]",
                                    "{price} EXP"
                                }
                                p { class: "text-[#16775D] font-semibold text-[12px]",
                                    "{tr.amount}: {remaining_quantity} {tr.unit}"
                                }
                            }
                        } else {
                            div { class: "px-[10px] flex flex-col gap-[10px] ",
                                p { class: "text-[#16775D] font-semibold text-[12px]",
                                    "{price} EXP"
                                }
                                p { class: "text-[#16775D] font-semibold text-[12px]",
                                    "{tr.amount}: {remaining_quantity} {tr.unit}"
                                }
                            }
                        }
                        div {
                            class: "w-full grid grid-cols-2 gap-[1px] text-white font-bold",
                            visibility: if !hover() { "hidden" },
                            button {
                                class: "col-span-1 w-full bg-[#00C564] py-[10px] disabled:bg-gray-100 disabled:text-gray-400",
                                disabled: disable_buy,
                                onclick: move |evt| {
                                    evt.prevent_default();
                                    evt.stop_propagation();
                                    onbuy(());
                                },
                                "{tr.buy}"
                            }
                            button {
                                class: "col-span-1 w-full bg-[#00C564] py-[10px] flex flex-row items-center justify-center gap-[10px] disabled:bg-gray-100 disabled:text-gray-400",
                                disabled: disable_like,
                                onclick: move |evt| {
                                    evt.prevent_default();
                                    evt.stop_propagation();
                                    onlike(());
                                },
                                if !disable_like {
                                    img {
                                        class: "w-[20px]",
                                        src: asset!("/public/images/heart-white.png"),
                                    }
                                } else {
                                    img {
                                        class: "w-[20px]",
                                        src: asset!("/public/images/heart.png"),
                                    }
                                }

                                "{tr.like}"
                            }
                        }
                    }
                }

                // Like
                div { class: "absolute top-2 right-2 px-2 py-1 flex flex-row items-center gap-2 text-[12px] text-white font-semibold justify-center rounded-full bg-gray-500",
                    img {
                        class: "w-[15px]",
                        src: asset!("/public/images/heart.png"),
                    }
                    "{likes}"
                }
            }
        }
    }
}
