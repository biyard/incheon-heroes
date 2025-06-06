use by_macros::DioxusController;
use dioxus::prelude::*;
use dioxus_translate::Translate;
use dto::*;

use crate::models::nft_metadata::NftMetadata;

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct HomePageController {
    pub html: Resource<(String, String)>,
}

impl HomePageController {
    pub fn new() -> std::result::Result<Self, RenderError> {
        let html = use_resource(move || async move {
            let res_ko: Result<String> = get_html(&format!(
                "https://metadata.biyard.co/incheon-universe/notice/html/heroes_shutdown_ko.html"
            ))
            .await;

            let res_en: Result<String> = get_html(&format!(
                "https://metadata.biyard.co/incheon-universe/notice/html/heroes_shutdown_en.html"
            ))
            .await;

            (
                match res_ko {
                    Ok(v) => v,
                    Err(e) => {
                        tracing::error!("get html ko failed with error: {:?}", e);
                        "".to_string()
                    }
                },
                match res_en {
                    Ok(v) => v,
                    Err(e) => {
                        tracing::error!("get html en failed with error: {:?}", e);
                        "".to_string()
                    }
                },
            )
        });

        let ctrl = Self { html };
        use_context_provider(|| ctrl);

        Ok(ctrl)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LeaderBoardController {
    pub leaderboard: Resource<Leaderboard>,

    pub selected_leaderboard_type: Signal<LeaderboardType>,
}

impl LeaderBoardController {
    pub fn new() -> std::result::Result<Self, RenderError> {
        let selected_leaderboard_type = use_signal(|| LeaderboardType::Level);

        let leaderboard = use_resource(move || async move {
            let selected_leaderboard_type = selected_leaderboard_type();

            let endpoint = crate::config::get().main_api_endpoint;
            let res: Result<Leaderboard> =
                rest_api::get(&format!("{}{}", endpoint, selected_leaderboard_type.url())).await;

            match res {
                Ok(mut res) => {
                    res.leaderboard.fetch_names().await;
                    res
                }
                Err(e) => {
                    tracing::error!("Failed to get leaderboard: {:?}", e);
                    Leaderboard::default()
                }
            }
        });

        let ctrl = Self {
            leaderboard,
            selected_leaderboard_type,
        };
        use_context_provider(|| ctrl);

        Ok(ctrl)
    }
}

#[derive(Debug, Clone, Copy, Translate, Default)]
pub enum LeaderboardType {
    #[translate(ko = "레벨 랭킹", en = "Level Ranking")]
    #[default]
    Level,
    #[translate(ko = "경험치 랭킹", en = "Experience Ranking")]
    Experience,
    #[translate(ko = "데일리 미션 참여 랭킹", en = "Daily Mission Ranking")]
    Daily,
    #[translate(ko = "검증 투표 참여 랭킹", en = "Voting Ranking")]
    Voting,
}

impl LeaderboardType {
    pub fn url(&self) -> &'static str {
        match self {
            Self::Level => "/v1/leaderboard/rank/level?version=2&rankCount=10",
            Self::Experience => "/v1/leaderboard/rank/experience?version=2&rankCount=10",
            Self::Daily => "/v1/leaderboard/rank/daily?version=2&rankCount=10",
            Self::Voting => "/v1/leaderboard/rank/voting?version=2&rankCount=10",
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leaderboard {
    pub status: String,
    pub leaderboard: LeaderboardItems,
    pub updated_at: i64,
}

impl Default for Leaderboard {
    fn default() -> Self {
        Self {
            status: "fail".to_string(),
            leaderboard: LeaderboardItems::Level(vec![]),
            updated_at: 0,
        }
    }
}

impl Leaderboard {
    pub fn updated_at(&self) -> String {
        use chrono::prelude::*;
        tracing::debug!("updated_at: {}", self.updated_at);
        Utc.timestamp_opt(self.updated_at, 0).unwrap().to_string()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(untagged)]
pub enum LeaderboardItems {
    Level(Vec<LeaderboardItemLevel>),
    Experience(Vec<LeaderboardItemExperience>),
    Daily(Vec<LeaderboardItemDailyMission>),
    Voting(Vec<LeaderboardItemVoting>),
}

impl LeaderboardItems {
    pub async fn fetch_names(&mut self) {
        match self {
            Self::Level(items) => {
                for item in items.iter_mut() {
                    item.character = nft_id_to_character(item.nft_num).await;
                }
            }
            Self::Experience(items) => {
                for item in items.iter_mut() {
                    item.character = nft_id_to_character(item.nft_num).await;
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LeaderboardItemLevel {
    pub version: i32,
    pub rank: i32,
    pub account_address: String,
    pub nft_num: i64,
    pub level: i32,
    #[serde(default)]
    pub character: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LeaderboardItemExperience {
    pub version: i32,
    pub rank: i32,
    pub account_address: String,
    pub nft_num: i64,
    pub experience: i32,
    #[serde(default)]
    pub character: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LeaderboardItemDailyMission {
    pub version: i32,
    pub rank: i32,
    pub account_address: String,
    pub daily_count: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LeaderboardItemVoting {
    pub version: i32,
    pub rank: i32,
    pub account_address: String,
    pub voting_count: i32,
}

pub async fn nft_id_to_character(id: i64) -> String {
    match NftMetadata::fetch(id).await {
        Ok(m) => m.character(),
        Err(e) => {
            tracing::error!("Failed to get NFT metadata: {:?}", e);
            "Unknown".to_string()
        }
    }
}

pub fn truncate_addr(addr: &str) -> String {
    format!("{}...{}", &addr[..6], &addr[addr.len() - 4..])
}

pub async fn get_html(url: &str) -> Result<String> {
    tracing::debug!("111");
    let client = reqwest::Client::new();
    tracing::debug!("222");
    let res = client.get(url).send().await?;
    tracing::debug!("333");
    let body = res.text().await?;
    tracing::debug!("444");
    Ok(body)
}
