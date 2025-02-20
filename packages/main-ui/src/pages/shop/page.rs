#![allow(non_snake_case)]
use crate::components::headings::Heading1;

use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn ShopPage(lang: Language) -> Element {
    let mut _ctrl = Controller::new()?;
    let tr: ShopTranslate = translate(&lang);

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
            div { class: "relative flex flex-col gap-[10px]",
                img { class: "w-full max-w-[1200] h-[400px]", src: "{img}" }
                div { class: "absolute bottom-0 left-0  w-full bg-black bg-opacity-50 p-[20px] flex flex-row justify-between items-center",
                    p { class: "text-white text-[25px] font-bold", "{title}" }
                    p { class: "text-white",
                        pre { class: "text-white text-[12px]", "{desc}" }
                    }
                }
            }
        }
    })
    .collect::<Vec<Element>>();

    rsx! {
        div {
            id: "shop",
            class: "flex flex-col gap-[50px] items-center justify-center w-full",
            Heading1 { lang, "{tr.title}" }
            div { class: "flex flex-col gap-[10px] text-center font-bold text-[16px]",
                pre { "{tr.desc}" }
                p { class: "text-[12px] font-semibold", "{tr.sub_desc}" }
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
    rsx! {
        div { class: "carousel w-full max-w-[1200px]",
            for (i , stack) in stacks.iter().enumerate() {
                div {
                    class: "carousel-item relative w-full",
                    id: format!("slide{}", i),
                    {stack}
                    div { class: "absolute left-5 right-5 top-1/2 flex -translate-y-1/2 transform justify-between",
                        a {
                            class: "btn-ghost rounded-full p-[10px]",
                            href: format!("#slide{}", (i + stacks.len() - 1) % stacks.len()),
                            {left.clone()}
                        }
                        a {
                            class: "btn-ghost rounded-full p-[10px]",
                            href: format!("#slide{}", (i + 1) % stacks.len()),
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
    likes: u32,
    image: String,
    name: String,
    price: u32,
    lang: Language,
    remaining_quantity: u32,
    children: Element,
) -> Element {
    let tr = translate::<ShopItemCardTranslate>(&lang);
    rsx! {
        div {..attributes,
            div { class: "relative w-full h-full",

                // Card
                div { class: "flex flex-col rounded-[8px] overflow-hidden shadow-lg bg-white w-full h-full",
                    img { class: "w-full h-245 object-fill", src: "{image}" }
                    div { class: "flex flex-col gap-[10px] p-[10px]",
                        p { "{name}" }
                        div { class: "flex flex-row justify-between items-center gap-[10px]",
                            p { "{price} EXP" }
                            p { "{tr.amount}: {remaining_quantity} {tr.unit}" }
                        }
                    }
                }

                // Like
                div { class: "absolute top-0 right-0 p-[10px]",
                    img {
                        class: "w-[10px]",
                        src: asset!("/public/images/heart.png"),
                    }
                    "{likes}"
                }


            }
        }
    }
}
