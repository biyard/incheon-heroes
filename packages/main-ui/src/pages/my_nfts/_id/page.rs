#![allow(non_snake_case)]
use crate::components::icons::{Badge, Experience, LinkIcon, Mint, Send, Swap, Trait, Transfer};
use crate::config::{self};
use crate::models::history::{MissionHistory, TokenHistory};
use crate::models::nft_metadata::NftMetadata;
use crate::services::mission_contract::Mission;
use crate::utils::address::parse_address;
use crate::utils::time::{formatted_timestamp, time_diff_from_10};

use super::controller::*;
use super::i18n::*;
use by_components::charts::horizontal_bar::HorizontalBar;
use by_components::files::DropZone;
use by_components::responsive::ResponsiveService;
use dioxus::prelude::*;
use dioxus_translate::*;

use dto::{AssetPresignedUris, Error};
#[cfg(feature = "web")]
use dto::{File, FileType};

#[cfg(feature = "web")]
use dioxus::html::FileEngine;

#[cfg(feature = "web")]
use dto::AssetPresignedUrisReadAction;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};

#[cfg(feature = "web")]
use std::sync::Arc;

#[cfg(feature = "web")]
use crate::BackendApi;

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
    let missions_ko = ctrl.get_mission_ko();
    tracing::debug!("daily mission: {:?}", missions);

    let mission_historys = ctrl.get_mission_historys();
    tracing::debug!("mission historys: {:?}", mission_historys);

    let token_historys = ctrl.get_token_historys();
    tracing::debug!("token historys: {:?}", token_historys);

    let progress_daily_missions = ctrl.get_progress_daily_missions();
    tracing::debug!("progress daily missions: {:?}", progress_daily_missions);

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

                DailySection {
                    lang,
                    missions,
                    missions_ko,
                    progress_missions: progress_daily_missions,
                    send_channel: move |(index, uri): (usize, String)| async move {
                        ctrl.send_discord_channel(lang, index as i64, uri).await;
                    },
                }

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
            div { class: "flex flex-row w-full h-[55px] justify-center items-center font-semibold text-[#636363] text-[20px] max-[900px]:text-[16px] max-[650]:text-[12px] max-[650]:text-[10px]",
                "Mission History"
            }

            div { class: "w-full overflow-x-scroll touch-auto",
                div { class: "min-w-[900px]",
                    div { class: "flex flex-row w-full h-[55px] border-t border-t-[#e0e0e0] border-b border-b-[#e0e0e0] justify-start items-start font-medium text-[20px] text-[#636363] max-[900px]:text-[16px] max-[650]:text-[12px] max-[650]:text-[10px]",
                        div { class: "flex flex-1 w-full h-full justify-center items-center text-center",
                            "{tr.mission_name}"
                        }
                        div { class: "flex flex-1 w-full h-full justify-center items-center text-center",
                            "{tr.progress_date}"
                        }
                        div { class: "flex flex-1 w-full h-full justify-center items-center text-center",
                            "{tr.verification}"
                        }
                        div { class: "flex flex-1 w-full h-full justify-center items-center text-center",
                            "{tr.gained_experience}"
                        }
                    }

                    for (index , history) in mission_historys.iter().enumerate() {
                        div {
                            class: format!(
                                "flex flex-row w-full min-h-[55px] {} justify-start items-center font-light text-[18px] text-[#636363]",
                                if index != mission_historys.len() - 1 {
                                    "border-b border-b-[#e0e0e0]"
                                } else {
                                    ""
                                },
                            ),
                            div { class: "flex flex-1 w-full h-full justify-center items-center whitespace-pre-line text-center",
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
        div { class: "flex flex-col w-full justify-start items-center bg-white border border-[#e0e0e0] rounded-[10px]",
            div { class: "flex flex-row w-full h-[55px] justify-center items-center font-semibold text-[#636363] text-[20px] max-[900px]:text-[16px] max-[650]:text-[12px] max-[650]:text-[10px]",
                "Activity"
            }

            div { class: "w-full overflow-x-scroll touch-auto",
                div { class: "w-full min-w-[900px]",
                    div { class: "flex flex-row w-full h-[55px] border-t border-t-[#e0e0e0] border-b border-b-[#e0e0e0] justify-start items-start font-medium text-[20px] text-[#636363] max-[900px]:text-[16px] max-[650]:text-[12px] max-[650]:text-[10px]",
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
                                "flex flex-row w-full min-h-[55px] {} justify-start items-center font-light text-[18px] text-[#636363] max-[900px]:text-[16px] max-[650]:text-[12px] max-[650]:text-[10px]",
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
    }
}

#[component]
pub fn DailySection(
    lang: Language,
    missions: Vec<Mission>,
    missions_ko: Vec<Mission>,
    progress_missions: Vec<String>,
    send_channel: EventHandler<(usize, String)>,
) -> Element {
    let len = missions.len();

    rsx! {

        div { class: "flex flex-col w-full justify-center items-center p-[10px] gap-[10px] bg-white rounded-[15px]",
            div { class: "font-bold text-[#636363] text-[25px]", "Daily Mission" }

            div { class: "flex flex-row flex-wrap w-full justify-center items-center gap-[10px]",
                for (index , mission) in missions.iter().enumerate() {
                    if index < 4 {
                        if progress_missions.contains(&missions_ko[index].mission.clone()) {
                            ProgressMissionBox { lang }
                        } else {
                            DailyEnableBox {
                                index,
                                title: mission.mission.clone(),
                                exp: mission.exp.as_u64() as i64,
                                send_channel: move |uri: String| {
                                    send_channel.call((index, uri));
                                },
                            }
                        }
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
pub fn ProgressMissionBox(lang: Language) -> Element {
    let time = time_diff_from_10();
    let mut mouse_hover = use_signal(|| false);
    let navigator = use_navigator();
    let tr: ProgressMissionBoxTranslate = translate(&lang);

    rsx! {
        div {
            class: format!(
                "cursor-pointer flex flex-col w-[230px] min-h-[80px] justify-center items-center {} rounded-[10px] gap-[2px] p-[5px]",
                if mouse_hover() { "bg-[#a0c1b8]" } else { "bg-[#e4e4e4]" },
            ),
            onmouseenter: move |_| {
                mouse_hover.set(true);
            },
            onmouseleave: move |_| {
                mouse_hover.set(false);
            },
            onclick: move |_| {
                let conf = config::get();
                let discord_mission_url = conf.discord_mission_url;
                navigator.push(format!("{}", discord_mission_url));
            },
            if mouse_hover() {
                div { class: "font-semibold text-[14px] text-white text-center", "{tr.go_to_vote}" }
            } else {
                div { class: "font-normal text-[14px] text-[#5b5b5b] text-center",
                    "{tr.inprogress_voting}"
                }
                div { class: "font-normal text-[14px] text-[#5b5b5b] text-center",
                    "{tr.remained_time} : {time}"
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
            img {
                src: asset!("/public/images/lock.png"),
                width: 25,
                height: 25,
            }
            div { class: "font-semibold text-[#5b5b5b] text-[14px]", "LV.{level}{tr.unlocked}" }
        }
    }
}

#[component]
pub fn DailyEnableBox(
    index: usize,
    title: String,
    exp: i64,
    send_channel: EventHandler<String>,
) -> Element {
    rsx! {
        DropZone {
            class: "cursor-pointer flex flex-col w-[230px] min-h-[80px] justify-center items-center bg-[#e4f4e4] rounded-[10px] gap-[2px] p-[5px]",
            onupload: move |(file_bytes, ext)| async move {
                match handle_upload(file_bytes, ext).await {
                    Ok((uri, _)) => {
                        send_channel(uri);
                    }
                    Err(e) => {
                        btracing::error!("Failed to upload source file: {:?}", e);
                    }
                }
            },
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

    let responsive: ResponsiveService = use_context();
    let flex = if responsive.width() > 500.0 {
        "flex-row"
    } else {
        "flex-col"
    };

    let icon_size = if responsive.width() > 500.0 {
        "40"
    } else {
        "20"
    };

    rsx! {
        div { class: "grid grid-cols-2 w-full max-[1200px]:grid-cols-1 gap-[20px] items-start",
            div { class: "col-span-1 w-full h-full flex items-center justify-center",
                img {
                    class: "w-full object-cover max-w-[600px] rounded-[15px]",
                    src: "{metadata.image}",
                }
            }
            div { class: "col-span-1 flex flex-col w-full justify-end items-end gap-[20px]",
                div { class: "flex flex-col w-full justify-start items-start px-[20px] gap-[10px]",
                    div { class: "flex flex-row w-full justify-between items-center",
                        div { class: "flex flex-col w-full justify-start items-start gap-[5px]",
                            div { class: "font-extrabold text-[35px] max-[630px]:text-[18px] text-[#636363]",
                                "{metadata.name}"
                            }
                        }

                        div { class: "flex flex-row w-fit gap-[10px]",
                            div {
                                class: "cursor-pointer",
                                onclick: move |e: Event<MouseData>| {
                                    open_swap_modal.call(e);
                                },
                                Swap {
                                    width: "{icon_size}",
                                    height: "{icon_size}",
                                }
                            }
                            div {
                                class: "cursor-pointer",
                                onclick: move |e: Event<MouseData>| {
                                    open_send_modal.call(e);
                                },
                                Send {
                                    width: "{icon_size}",
                                    height: "{icon_size}",
                                }
                            }
                        }
                    }

                    div { class: "font-normal text-[16px] text-[#636363]", "{metadata.description}" }
                }

                if let Some(ref character) = character() {
                    div { class: "flex {flex} w-full justify-center items-center p-[10px] bg-white rounded-[10px] p-[30px] gap-[30px]",
                        div { class: "flex flex-col justify-center items-center gap-[3px]",
                            Badge { width: "30", height: "30" }
                            div { class: "font-bold text-[#16775d] text-[25px]", "{character.name}" }
                        }

                        div { class: "flex flex-row justify-start items-start whitespace-pre-line",
                            "{character.description}"
                        }
                    }
                }

                div { class: "flex flex-col w-full justify-center items-center py-[10px] px-[20px] bg-white rounded-[10px]",
                    div { class: "flex flex-row w-full justify-center items-center gap-[10px] pb-[20px]",
                        Trait {}
                        div { class: "font-bold text-[25px] text-[#636363]", "Traits" }
                    }

                    div { class: "w-full grid grid-cols-3 max-[600px]:grid-cols-2 max-[460px]:grid-cols-1 gap-[10px] items-start auto-rows-fr",
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
        div { class: "col-span-1 flex flex-col w-full justify-center items-center p-[10px] gap-[5px] bg-[#e4f4e4] rounded-[10px]",
            div { class: "font-medium text-[14px] text-[#5b5b5b]", "{title}" }
            div { class: "font-bold text-[16px] max-[700px]:text-[14px]  text-[#16775d]",
                "{description}"
            }
        }
    }
}

#[cfg(feature = "web")]
pub async fn handle_file_upload(file_engine: Arc<dyn FileEngine>, api: BackendApi) -> Vec<File> {
    let mut result: Vec<File> = vec![];
    let files = file_engine.files();

    for f in files {
        match file_engine.read_file(f.as_str()).await {
            Some(bytes) => {
                let file_name: String = f.into();
                let file_name_copy = file_name.clone();
                let ext = file_name.rsplitn(2, '.').nth(0).unwrap_or("").to_string();

                let file_type = FileType::from_str(&ext.clone());
                let file_type = if file_type.is_ok() {
                    Some(file_type.unwrap())
                } else {
                    None
                };

                let req = AssetPresignedUrisReadAction {
                    action: None,
                    total_count: None,
                    file_type,
                };

                let bytes = process_image(&bytes, 400, 400);

                let url = match api.upload_metadata(bytes, req).await {
                    Ok(v) => Some(v),
                    Err(_) => None,
                };

                result.push(File {
                    name: file_name_copy.clone(),
                    ext: ext.to_string(),
                    url,
                });
            }
            None => {
                tracing::error!("Error reading file");
                continue;
            }
        };
    }
    result
}

pub fn process_image(input: &[u8], width: u32, height: u32) -> Vec<u8> {
    let img = image::load_from_memory(input).expect("Failed to load image");

    let resized = img.resize(width, height, image::imageops::FilterType::Lanczos3);
    let output = resized.as_bytes();

    if output.len() > 1000000000 {
        btracing::error!("Too large file");
        return vec![];
    }

    output.to_vec()
}

pub async fn handle_upload(file_bytes: Vec<u8>, ext: String) -> dto::Result<(String, String)> {
    let cli = AssetPresignedUris::get_client(config::get().new_api_endpoint);
    let res = cli
        .get_presigned_uris(1, dto::FileType::from_str(&ext).unwrap_or_default())
        .await?;

    let presigned_uri = res.presigned_uris.first().ok_or(Error::EmptyData)?;
    let uri = res.uris.first().ok_or(Error::EmptyData)?;

    use infer;

    let kind = infer::get(&file_bytes).ok_or(Error::EmptyData)?;
    tracing::debug!("Inferred file type: {:?}", kind);

    let content_type = kind.mime_type().to_string();
    tracing::debug!("Content type: {:?}", content_type);
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_str(&content_type).map_err(|_| Error::InvalidType)?,
    );

    reqwest::Client::new()
        .put(presigned_uri)
        .body(file_bytes)
        .headers(headers)
        .send()
        .await?;

    Ok((uri.to_string(), content_type))
}
