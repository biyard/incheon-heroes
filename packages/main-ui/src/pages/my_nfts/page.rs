#![allow(non_snake_case)]
use crate::components::headings::Heading1;
use crate::models::nft_metadata::NftMetadata;
use crate::route::Route;

use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn MyNftsPage(lang: Language) -> Element {
    let ctrl = Controller::new(lang)?;
    let tr: MyNftsTranslate = translate(&lang);

    let sbts = ctrl.user_service.sbts().unwrap_or(vec![]);
    let icps = ctrl.user_service.icp_nfts().unwrap_or(vec![]);
    let evms = ctrl.user_service.evm_nfts().unwrap_or(vec![]);

    rsx! {
        div {
            id: "my-nfts",
            class: "flex flex-col w-full gap-[80px] justify-center items-center",
            Heading1 { lang, "{tr.title}" }
            div { class: "flex flex-col w-full max-w-[1325px] bg-white rounded-[30px] p-[20px] gap-[20px]",
                div { class: "flex flex-row w-full justify-end items-end",
                    div { class: "min-w-[150px] px-4 h-[40px] rounded-[10px] bg-[#2ab28c] justify-center items-center",
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
pub fn NftList(
    lang: Language,
    sbts: Vec<(u64, NftMetadata)>,
    icps: Vec<(u64, NftMetadata)>,
    evms: Vec<(u64, NftMetadata)>,
) -> Element {
    let total_items = sbts.len() + icps.len() + evms.len();
    rsx! {
        div { class: "flex flex-col w-full gap-[40px]",
            div { class: "font-bold text-black text-[30px]", "{total_items} items" }
            div { class: "grid grid-cols-4 gap-[20px] justify-items-center",
                for nft in sbts {
                    NftCard { lang, nft }
                }

                for nft in icps {
                    NftCard { lang, nft }
                }

                for nft in evms {
                    NftCard { lang, nft }
                }
            }
        }
    }
}

#[component]
pub fn NftCard(lang: Language, nft: (u64, NftMetadata)) -> Element {
    let id = nft.0;
    let metadata = nft.1;
    rsx! {
        Link {
            to: Route::MyNftsByIdPage {
                lang,
                id: id.to_string(),
            },
            div { class: "flex flex-col w-full max-w-[250px] max-h-[350px] rounded-[12px] shadow-lg bg-white",
                img {
                    class: "w-full object-fill min-h-[250px]",
                    src: "{metadata.image}",
                }
                div { class: "flex flex-row w-full justify-start items-start px-[10px] py-[7px] font-bold text-[16px] text-black",
                    "{metadata.name}"
                }
                div {
                    class: "block w-full justify-start items-start px-[10px] py-[7px] font-normal text-[14px] text-black",
                    style: "display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden;",
                    "{metadata.description}"
                }
            }
        }
    }
}
