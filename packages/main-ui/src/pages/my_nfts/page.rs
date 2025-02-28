#![allow(non_snake_case)]
use crate::components::headings::Heading1;
use crate::models::nft_metadata::NftMetadata;
use crate::route::Route;

use super::controller::*;
use super::i18n::*;
use by_components::icons;
use dioxus::prelude::*;
use dioxus_translate::*;
use ethers::types::U256;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq)]
pub enum NftType {
    #[default]
    SBT,
    Kaia,
    Icp,
}

#[component]
pub fn MyNftsPage(lang: Language) -> Element {
    let ctrl = Controller::new(lang)?;
    let tr: MyNftsTranslate = translate(&lang);

    let sbts = ctrl.user_service.sbts().unwrap_or(vec![]);
    let icps = ctrl.user_service.icp_nfts().unwrap_or(vec![]);
    let evms = ctrl.user_service.evm_nfts().unwrap_or(vec![]);

    let account_exp = ctrl
        .user_service
        .account_exp()
        .unwrap_or(U256::from(0))
        .as_u64();
    let total_ids = ctrl.user_service.total_nfts();

    rsx! {
        div {
            id: "my-nfts",
            class: "flex flex-col w-full gap-[80px] justify-center items-center",
            Heading1 { lang, "{tr.title}" }
            div { class: "flex flex-col w-full max-w-[1325px] bg-white rounded-[30px] p-[20px] gap-[20px]",
                div { class: "flex flex-row w-full justify-end items-end",
                    button {
                        class: "cursor-pointer min-w-[150px] px-4 h-[40px] rounded-[10px] bg-[#2ab28c] justify-center items-center",
                        onclick: move |_| {
                            ctrl.open_nft_select_modal(lang, account_exp, total_ids.clone());
                        },
                        div { class: "flex flex-row w-full h-full justify-center items-center font-bold text-base text-white",
                            "{tr.distribute_exp}"
                        }
                    }
                }

                div { class: "flex flex-row w-full h-[1px] bg-[#e6e6e6]" }

                NftList {
                    lang,
                    sbts,
                    icps,
                    evms,
                }
            }
        }
    }
}

#[component]
pub fn NftSelectModal(
    lang: Language,
    total_nfts: Vec<u64>,
    account_exp: u64,
    oncancel: EventHandler<MouseEvent>,
    ondistibute: EventHandler<(u64, u64)>,
) -> Element {
    let tr: NftSelectModalTranslate = translate(&lang);

    let mut selected_id: Signal<Option<u64>> = use_signal(|| None);
    let mut exp_value: Signal<u64> = use_signal(|| 0);
    rsx! {
        div { class: "flex flex-col w-[350px] md:w-[800px] max-w-[800px] h-[300px] justify-start items-center gap-[10px]",
            div { class: "font-semibold text-[15px] md:text-[20px] text-[#16775d]",
                "{tr.title}"
            }
            div { class: "flex flex-row md:w-[400px] w-[330px] justify-center items-center gap-[30px]",
                div { class: "flex flex-row flex-1 justify-center items-center font-semibold text-[#636363] text-[13px] md:text-[18px]",
                    "{tr.experience_points}"
                }
                div { class: "flex flex-row flex-1 justify-center items-center font-semibold text-[#636363] text-[13px] md:text-[18px]",
                    "{account_exp} EXP"
                }
            }
            div { class: "flex flex-col md:w-[400px] w-[330px] justify-center items-center bg-white border border-[#e0e0e0] rounded-[8px]",
                div { class: "flex flex-row w-full h-[40px] justify-center items-center border-b border-b-[#e0e0e0]",
                    div { class: "flex flex-row flex-1 justify-center items-center px-[10px] text-[#636363] text-[13px] md:text-[18px]",
                        "{tr.nfts_held}"
                    }
                    div { class: "flex flex-row flex-1 justify-center items-center px-[10px] text-[#636363] text-[13px] md:text-[18px]",
                        "{tr.distribution_experience}"
                    }
                }
                div { class: "flex flex-row w-full h-[50px] justify-center items-center",
                    div { class: "flex flex-row flex-1 justify-center items-center px-[10px] text-[#636363] text-[13px] md:text-[18px]",
                        select {
                            class: "bg-white text-center h-[40px] px-[10px] flex items-center justify-center w-full rounded-[10px] text-[#636363] font-medium border border-[#e0e0e0] focus:outline-none",
                            value: (selected_id)(),
                            onchange: move |e: Event<FormData>| {
                                selected_id.set(Some(e.value().parse::<u64>().unwrap()));
                            },
                            option {
                                value: 0,
                                disabled: true,
                                selected: (selected_id)().is_none(),
                                "{tr.select_nft}"
                            }
                            for nft in total_nfts.clone() {
                                option {
                                    value: nft.clone(),
                                    selected: (selected_id)().is_some() && (selected_id)().unwrap() == nft,
                                    "#{nft}"
                                }
                            }
                        }
                    }
                    div { class: "flex flex-row flex-1 justify-center items-center px-[10px] text-[#636363] text-[13px] md:text-[18px] gap-[10px]",
                        input {
                            class: "max-w-[100px] px-[10px] h-[40px] text-[#636363] bg-transparent font-medium focus:outline-none",
                            placeholder: tr.exp_hint,
                            value: exp_value(),
                            r#type: "text",
                            onkeydown: move |e: KeyboardEvent| {
                                let key = e.key();
                                if key != Key::Backspace && key != Key::Delete {
                                    let s = match key {
                                        Key::Character(c) => c,
                                        _ => "".to_string(),
                                    };
                                    if !s.chars().all(|c| c.is_ascii_digit()) {
                                        e.prevent_default();
                                    }
                                }
                            },
                            oninput: move |e| {
                                exp_value.set(e.value().parse::<u64>().unwrap_or_default());
                            },
                        }
                        div { class: " text-[#636363] text-[13px] md:text-[18px]", "EXP" }
                    }
                }
            }
            div { class: "flex flex-row w-full justify-center items-center gap-[10px]",
                div {
                    class: "cursor-pointer flex flex-row w-[150px] h-[45px] justify-center items-center rounded-[10px] bg-white border border-[#e0e0e0] font-semibold text-[13px] md:text-[16px] text-[#16775d]",
                    //FIXME: Add exception handling code
                    onclick: move |_| {
                        ondistibute.call((selected_id().unwrap_or_default(), exp_value()));
                    },
                    "{tr.distribute}"
                }
                div {
                    class: "cursor-pointer flex flex-row w-[150px] h-[45px] justify-center items-center rounded-[10px] bg-white border border-[#e0e0e0] font-semibold text-[13px] md:text-[16px] text-[#636363]",
                    onclick: move |e: Event<MouseData>| {
                        oncancel.call(e);
                    },
                    "{tr.cancel}"
                }
            }
            div { class: "font-semibold text-[12px] md:text-[15px] text-[#636363]",
                "{tr.experience_description_1}"
            }
            div { class: "flex flex-col w-full justify-center items-center gap-[5px]",
                div { class: "font-semibold text-[12px] md:text-[15px] text-[#636363]",
                    "{tr.experience_description_2}"
                }
                div { class: "font-semibold text-[12px] md:text-[15px] text-[#636363]",
                    "{tr.experience_description_3}"
                }
            }
        }
    }
}

#[component]
pub fn NftList(
    lang: Language,
    sbts: Vec<(u64, NftMetadata)>,
    icps: Vec<(u64, NftMetadata)>,
    evms: Vec<(u64, NftMetadata)>,
) -> Element {
    let total_items = sbts.len() + icps.len() + evms.len();
    rsx! {
        div { class: "flex flex-col w-full gap-[40px]",
            div { class: "font-bold text-black text-[30px] max-[500px]:text-[16px]",
                "{total_items} items"
            }
            div { class: "grid grid-cols-4 max-[1200px]:grid-cols-3 max-[900px]:grid-cols-2 max-[600px]:grid-cols-1 gap-[20px] justify-items-center",
                for nft in sbts {
                    NftCard { lang, nft, nft_type: NftType::SBT }
                }

                for nft in icps {
                    NftCard { lang, nft, nft_type: NftType::Icp }
                }

                for nft in evms {
                    NftCard { lang, nft, nft_type: NftType::Kaia }
                }
            }
        }
    }
}

#[component]
pub fn NftCard(lang: Language, nft: (u64, NftMetadata), nft_type: NftType) -> Element {
    let id = nft.0;
    let metadata = nft.1;

    rsx! {
        Link {
            class: "w-full col-span-1 h-full flex flex-col justify-start items-center",
            to: Route::MyNftsByIdPage {
                lang,
                id: id.to_string(),
            },
            div { class: "relative flex flex-col w-full max-w-[250px] max-h-[350px] rounded-[12px] shadow-lg bg-white overflow-hidden",
                div {
                    class: format!(
                        "flex flex-row justify-center items-center absolute top-[10px] left-[10px] w-[30px] h-[30px] {}",
                        if nft_type == NftType::Icp { "rounded-[100px] bg-white " } else { "" },
                    ),

                    if nft_type == NftType::Icp {
                        icons::logo::InternetIdentity { size: 25 }
                    } else if nft_type == NftType::Kaia {
                        icons::logo::Kaia { size: 25 }
                    }
                }
                img {
                    class: "w-full object-fill min-h-[250px] rounded-t-[12px]",
                    src: "{metadata.image}",
                }
                div { class: "flex flex-row w-full justify-start items-start px-[10px] py-[7px] font-bold text-[16px] text-black",
                    "{metadata.name}"
                }
                div { class: "w-full px-[10px] py-[7px] font-normal text-[14px]",
                    p { class: "line-clamp-2", "{metadata.description}" }
                }
            }
        }
    }
}
