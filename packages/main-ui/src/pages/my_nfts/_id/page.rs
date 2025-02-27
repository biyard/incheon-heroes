#![allow(non_snake_case)]
use crate::components::icons::{
    Badge, Experience, LinkIcon, Lock, Mint, Send, Swap, Trait, Transfer,
};
use crate::models::history::{MissionHistory, TokenHistory};
use crate::models::nft_metadata::NftMetadata;
use crate::services::mission_contract::Mission;
use crate::utils::address::parse_address;
use crate::utils::time::formatted_timestamp;

use super::controller::*;
use super::i18n::*;
use by_components::charts::horizontal_bar::HorizontalBar;
use dioxus::prelude::*;
use dioxus_translate::*;

#[derive(Debug, Clone)]
pub struct CharacterName {
    pub name: String,
    pub description: String,
}

#[component]
pub fn MyNftsByIdPage(id: String, lang: Language) -> Element {
    let ctrl = Controller::new(id.clone().parse::<i64>().unwrap())?;
    let _tr: MyNftsByIdTranslate = translate(&lang);
    let (level, elevel, exp, max_exp) = ctrl.get_level_info();

    let metadata = ctrl.get_metadata();
    tracing::debug!("nft metadata: {:?}", metadata);

    let missions = ctrl.get_mission(lang);
    tracing::debug!("daily mission: {:?}", missions);

    let mission_historys = ctrl.get_mission_historys();
    tracing::debug!("mission historys: {:?}", mission_historys);

    let token_historys = ctrl.get_token_historys();
    tracing::debug!("token historys: {:?}", token_historys);

    let klaytn_scope_endpoint = ctrl.get_scope_endpoint();

    rsx! {
        div { class: "flex flex-col w-full justify-center items-center",
            div { class: "flex flex-col w-full max-w-[1300px] justify-center items-center gap-[30px]",
                MetadataSection {
                    lang,
                    id,
                    level,
                    elevel,
                    exp,
                    max_exp,
                    metadata,

                    open_swap_modal: move |_| {
                        ctrl.open_swap_modal(lang);
                    },
                    open_send_modal: move |_| {
                        ctrl.open_send_modal(lang);
                    },
                }

                DailySection { lang, missions }

                MissionHistorySection { lang, mission_historys }
                ActivitySection { lang, token_historys, klaytn_scope_endpoint }
            }
        }
    }
}

#[component]
pub fn MissionHistorySection(lang: Language, mission_historys: Vec<MissionHistory>) -> Element {
    let ctrl = HistoryController::new()?;
    let tr: MissionHistorySectionTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start bg-white border border-[#e0e0e0] rounded-[10px]",
            div { class: "flex flex-row w-full h-[55px] justify-center items-center font-semibold text-[#636363] text-[20px]",
                "Mission History"
            }
            div { class: "flex flex-row w-full h-[55px] border-t border-t-[#e0e0e0] border-b border-b-[#e0e0e0] justify-start items-start font-medium text-[20px] text-[#636363]",
                div { class: "flex flex-1 w-full h-full justify-center items-center",
                    "{tr.mission_name}"
                }
                div { class: "flex flex-1 w-full h-full justify-center items-center",
                    "{tr.progress_date}"
                }
                div { class: "flex flex-1 w-full h-full justify-center items-center",
                    "{tr.verification}"
                }
                div { class: "flex flex-1 w-full h-full justify-center items-center",
                    "{tr.gained_experience}"
                }
            }

            for (index , history) in mission_historys.iter().enumerate() {
                div {
                    class: format!(
                        "flex flex-row w-full h-[55px] {} justify-start items-start font-light text-[18px] text-[#636363]",
                        if index != mission_historys.len() - 1 {
                            "border-b border-b-[#e0e0e0]"
                        } else {
                            ""
                        },
                    ),
                    div { class: "flex flex-1 w-full h-full justify-center items-center",
                        {
                            ctrl.translate_mission_title(
                                lang,
                                history.mission_name.clone(),
                                history.mission_name_en.clone(),
                            )
                        }
                    }
                    div { class: "flex flex-1 w-full h-full justify-center items-center",
                        {formatted_timestamp(history.mission_start_date.parse::<i64>().unwrap())}
                    }
                    div { class: "flex flex-1 w-full h-full justify-center items-center",
                        {
                            if history.progress == "Inprogress" {
                                tr.verification_in_progress
                            } else if history.progress == "accepted" {
                                tr.accepted
                            } else {
                                tr.rejected
                            }
                        }
                    }
                    div { class: "flex flex-1 w-full h-full justify-center items-center",
                        {
                            if history.experience <= 0 {
                                "0 EXP".to_string()
                            } else {
                                format!("{} EXP", history.experience)
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ActivitySection(
    lang: Language,
    token_historys: Vec<TokenHistory>,
    klaytn_scope_endpoint: String,
) -> Element {
    let ZERO_ADDRESS = "0x0000000000000000000000000000000000000000";
    let navigator = use_navigator();

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start bg-white border border-[#e0e0e0] rounded-[10px]",
            div { class: "flex flex-row w-full h-[55px] justify-center items-center font-semibold text-[#636363] text-[20px]",
                "Activity"
            }
            div { class: "flex flex-row w-full h-[55px] border-t border-t-[#e0e0e0] border-b border-b-[#e0e0e0] justify-start items-start font-medium text-[20px] text-[#636363]",
                div { class: "flex flex-1 w-full h-full justify-center items-center",
                    "Event"
                }
                div { class: "flex flex-1 w-full h-full justify-center items-center",
                    "From"
                }
                div { class: "flex flex-1 w-full h-full justify-center items-center",
                    "To"
                }
                div { class: "flex flex-1 w-full h-full justify-center items-center",
                    "Transaction"
                }
            }

            for (index , history) in token_historys.iter().enumerate() {
                div {
                    class: format!(
                        "flex flex-row w-full h-[55px] {} justify-start items-start font-light text-[18px] text-[#636363]",
                        if index != token_historys.len() - 1 {
                            "border-b border-b-[#e0e0e0]"
                        } else {
                            ""
                        },
                    ),
                    div { class: "flex flex-row flex-1 w-full h-full justify-center items-center gap-[10px]",
                        {
                            if history.from == ZERO_ADDRESS {
                                rsx! {
                                    Mint {}
                                    div { "Mint" }
                                }
                            } else {
                                rsx! {
                                    Transfer {}
                                    div { "Transfer" }
                                }
                            }
                        }
                    }
                    div { class: "flex flex-1 w-full h-full justify-center items-center",
                        {
                            if history.from == ZERO_ADDRESS {
                                "NullAddress".to_string()
                            } else {
                                parse_address(history.from.clone())
                            }
                        }
                    }
                    div { class: "flex flex-1 w-full h-full justify-center items-center",
                        {
                            if history.to == ZERO_ADDRESS {
                                "NullAddress".to_string()
                            } else {
                                parse_address(history.to.clone())
                            }
                        }
                    }
                    div { class: "flex flex-1 w-full h-full justify-center items-center",
                        div {
                            class: "flex flex-row w-fit h-fit cursor-pointer",
                            onclick: {
                                let history = history.clone();
                                let klaytn_scope_endpoint = klaytn_scope_endpoint.clone();
                                move |_| {
                                    navigator
                                        .push(format!("{}/{}", klaytn_scope_endpoint, history.transaction_hash));
                                }
                            },
                            LinkIcon {}
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn DailySection(lang: Language, missions: Vec<Mission>) -> Element {
    let len = missions.len();
    rsx! {
        div { class: "flex flex-col w-full justify-center items-center p-[10px] gap-[10px] bg-white rounded-[15px]",
            div { class: "font-bold text-[#636363] text-[25px]", "Daily Mission" }

            div { class: "flex flex-wrap w-full justify-center items-start gap-[10px]",
                for mission in missions {
                    DailyEnableBox {
                        title: mission.mission.clone(),
                        exp: mission.exp.as_u64() as i64,
                    }
                }

                for level in (len + 1)..5 {
                    DailyNotEnableBox { lang, level: level as i64 }
                }
            }
        }
    }
}

#[component]
pub fn DailyNotEnableBox(lang: Language, level: i64) -> Element {
    let tr: DailyNotEnableBoxTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-[230px] min-h-[80px] justify-center items-center bg-[#dadfdb] rounded-[10px] gap-[2px] p-[5px]",
            Lock {}
            div { class: "font-semibold text-[#5b5b5b] text-[14px]", "LV.{level}{tr.unlocked}" }
        }
    }
}

#[component]
pub fn DailyEnableBox(title: String, exp: i64) -> Element {
    rsx! {
        div { class: "flex flex-col w-[230px] min-h-[80px] justify-center items-center bg-[#e4f4e4] rounded-[10px] gap-[2px] p-[5px]",
            div { class: "font-normal text-[#5b5b5b] text-[14px] whitespace-pre-line text-center",
                "{title}"
            }
            div { class: "font-semibold text-[#16775d] text-[16px]", "+ {exp} EXP" }
        }
    }
}

#[component]
pub fn MetadataSection(
    lang: Language,
    id: String,
    level: u64,
    elevel: u64,
    exp: u64,
    max_exp: u64,
    metadata: NftMetadata,

    open_swap_modal: EventHandler<MouseEvent>,
    open_send_modal: EventHandler<MouseEvent>,
) -> Element {
    let tr: MetadataSectionTranslate = translate(&lang);
    let mut character = use_signal(|| None);
    let attributes = metadata.attributes.clone();
    let mut exp_val = use_signal(|| exp);
    let mut max_exp_val = use_signal(|| max_exp);

    use_effect(use_reactive(&(exp, max_exp), move |(exp, max_exp)| {
        exp_val.set(exp);
        max_exp_val.set(max_exp);
    }));

    use_effect({
        move || {
            if !metadata.attributes.is_empty() {
                let attribute = metadata.attributes[0].clone();

                if attribute.value == "Ainy" {
                    character.set(Some(CharacterName {
                        name: "Ainy".to_string(),
                        description: tr.ainy_description.to_string(),
                    }));
                } else if attribute.value == "Bumy" {
                    character.set(Some(CharacterName {
                        name: "Bumy".to_string(),
                        description: tr.bumy_description.to_string(),
                    }));
                } else if attribute.value == "Comy" {
                    character.set(Some(CharacterName {
                        name: "Comy".to_string(),
                        description: tr.comy_description.to_string(),
                    }));
                } else {
                    character.set(Some(CharacterName {
                        name: "DngDaery".to_string(),
                        description: tr.dngdaery_description.to_string(),
                    }));
                }
            }
        }
    });

    rsx! {
        div { class: "flex flex-col w-full md:grid md:grid-cols-2 gap-[20px] items-start md:items-end auto-rows-fr",
            img {
                class: "flex w-full aspect-square object-cover max-w-[600px] rounded-[15px]",
                src: "{metadata.image}",
            }
            div { class: "flex flex-col w-full justify-end items-end gap-[20px]",
                div { class: "flex flex-col w-full justify-start items-start px-[20px] gap-[10px]",
                    div { class: "flex flex-row w-full justify-between items-center",
                        div { class: "flex flex-col w-full justify-start items-start gap-[5px]",
                            div { class: "font-extrabold text-[35px] text-[#636363]",
                                "{metadata.name}"
                            }
                        }

                        div { class: "flex flex-row w-fit gap-[10px]",
                            div {
                                class: "w-[40px] h-[40px] cursor-pointer",
                                onclick: move |e: Event<MouseData>| {
                                    open_swap_modal.call(e);
                                },
                                Swap { width: "40", height: "40" }
                            }
                            div {
                                class: "w-[40px] h-[40px] cursor-pointer",
                                onclick: move |e: Event<MouseData>| {
                                    open_send_modal.call(e);
                                },
                                Send { width: "40", height: "40" }
                            }
                        }
                    }

                    div { class: "font-normal text-[16px] text-[#636363]", "{metadata.description}" }
                }

                if character().is_some() {
                    div { class: "flex flex-row w-full justify-between items-center p-[10px] bg-white rounded-[10px]",
                        div { class: "flex flex-col min-w-[110px] justify-center items-center gap-[3px]",
                            Badge { width: "30", height: "30" }
                            div { class: "font-bold text-[#16775d] text-[25px]",
                                "{character().unwrap().name}"
                            }
                        }

                        div { class: "flex flex-row w-full px-[10px] py-[6px] justify-start items-start whitespace-pre-line",
                            "{character().unwrap().description}"
                        }
                    }
                }

                div { class: "flex flex-col w-full justify-center items-center py-[10px] px-[20px] bg-white rounded-[10px]",
                    div { class: "flex flex-row w-full justify-center items-center gap-[10px] pb-[20px]",
                        Trait {}
                        div { class: "font-bold text-[25px] text-[#636363]", "Traits" }
                    }

                    div { class: "w-full grid gird-cols-2 md: grid-cols-3 gap-[10px] items-start auto-rows-fr",
                        for attribute in attributes.clone() {
                            TraitBox {
                                title: attribute.trait_type.clone(),
                                description: attribute.value.clone(),
                            }
                        }
                    }
                }

                div { class: "flex flex-col w-full justify-start items-start px-[15px] py-[7px] bg-white rounded-[10px] gap-[6px]",
                    div { class: "font-bold text-[25px] text-[#16775d]", "LV. {level}" }
                    HorizontalBar {
                        id: format!("experience_bar"),
                        value: exp_val() as i64,
                        height: "13px",
                        max_value: max_exp_val() as i64,
                        class: "flex flex-row w-full bg-white border border-[#16775d] rounded-[6px] overflow-hidden",
                    }
                    div { class: "flex flex-row w-full justify-end items-end gap-[10px]",
                        Experience {}
                        div { class: "font-normal text-[#16775d] text-[10px]", "{exp}/{max_exp}" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn TraitBox(title: String, description: String) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-center items-center p-[10px] gap-[5px] bg-[#e4f4e4] rounded-[10px]",
            div { class: "font-medium text-[14px] text-[#5b5b5b]", "{title}" }
            div { class: "font-bold text-[16px] text-[#16775d]", "{description}" }
        }
    }
}
