#![allow(non_snake_case)]
use crate::components::icons;

use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;
use num_format::Locale;
use num_format::ToFormattedString;

#[component]
pub fn ShopByIdPage(id: String, lang: Language) -> Element {
    let ctrl = Controller::new(lang, id.clone())?;
    let tr: ShopByIdTranslate = translate(&lang);

    let item = ctrl.item()?;
    tracing::debug!("{:?}", item);
    let color = match item.level {
        4 => "#BA2929",
        3 => "#2168C3",
        _ => "#029F75",
    };
    let name = match lang {
        Language::Ko => &item.name_ko.split_once(" ").unwrap().1,
        Language::En => &item.name_en.split_once(" ").unwrap().1,
    };
    let desc = ctrl.description(lang)?;

    rsx! {
        div { id: "shop-by-id", class: "w-full flex flex-col items-center",
            div { class: "w-full max-w-[900px] flex flex-col gap-[20px] items-center",

                div { class: "w-full flex flex-row gap-[40px] justify-center",

                    div { class: "relative w-[400px] rounded-[10px] overflow-hidden bg-white/40",
                        img {
                            class: "w-full h-full object-cover",
                            src: "{item.image}",
                        }
                        div { class: "absolute p-[8px] top-0 right-0 flex flex-row gap-[5px] text-white font-bold text-[14px]",
                            img {
                                class: "w-[20px]",
                                src: asset!("/public/images/heart.png"),
                            }

                            "{item.likes}"
                        }
                    }

                    div { class: "flex flex-col grow items-center gap-[10px]",
                        h2 { class: "text-[30px] font-semibold", color, "{name}" }

                        // TODO: description
                        div { class: "w-full flex flex-col p-[10px] items-center justify-start grow bg-white/40 rounded-[10px]",
                            div { class: "flex flex-col items-center",
                                icons::Badge {}
                                p { class: "text-[20px] font-bold text-[#16775D]",
                                    "{tr.title}"
                                }
                            }
                            p {
                                class: "w-full text-center text-[#636363] text-[15px]",
                                font_family: "Pretendard",
                                dangerous_inner_html: "{desc}",
                            }
                        }
                        div { class: "w-full bg-white/40 rounded-[10px] grid grid-cols-3 gap-[20px] py-[15px]",
                            span { class: "w-full col-span-1 text-center text-[#636363] text-[14px] font-bold",
                                "{tr.price}"
                            }
                            span { class: "w-full col-span-1 text-right text-[#16775D] text-[14px] font-bold",
                                "{item.price.as_u64().to_formatted_string(&Locale::en)}"
                            }

                            span { class: "w-full col-span-1 text-left text-[#636363] text-[14px]",
                                "EXP"
                            }
                        } // end of price

                        div { class: "w-full bg-white/40 rounded-[10px] grid grid-cols-3 gap-[20px] py-[15px]",
                            span { class: "w-full col-span-1 text-center text-[#636363] text-[14px] font-bold",
                                "{tr.remaining}"
                            }
                            span { class: "w-full col-span-1 text-right text-[#16775D] text-[14px] font-bold",
                                "{item.remaining.as_u64().to_formatted_string(&Locale::en)}"
                            }

                            span { class: "w-full col-span-1 text-left text-[#636363] text-[14px]",
                                "{tr.unit}"
                            }
                        } // end of remaining

                        div { class: "w-full grid grid-cols-5 gap-[10px]",
                            // FIXME: use login signal
                            button { class: "col-span-3 w-full bg-[#24B28C] text-center py-[15px] rounded-[10px] text-white font-semibold",
                                "{tr.btn_login}"
                            }

                            button { class: "col-span-2 w-full bg-white/40 text-center py-[15px] rounded-[10px] text-[#636363] font-semibold",
                                "{tr.btn_like} ♡"
                            }
                        }
                    }
                } // end of top header section

                div {
                    id: "shop_item_details",
                    class: "w-full bg-white/40 rounded-[10px] px-[50px] p-[40px] text-center",
                    dangerous_inner_html: "{ctrl.details()?.1}",
                }
            } // end of container

        } // end of this page
    }
}
