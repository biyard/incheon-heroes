use crate::config;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostResult {
    pub status: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DailyMission {
    pub mission_title: String,
    pub mission_title_en: String,
    pub mission_description: String,
    pub mission_description_en: String,
    pub lang: String,
    pub mission_experience: i32,
    pub token_id: i32,
    #[serde(rename = "imageURL")]
    pub image_url: String,
    pub account: String,
    pub version: i32,
}

impl PostResult {
    pub async fn send_discord_channel(req: DailyMission) -> dto::Result<Self> {
        let endpoint = config::get().discord_api_endpoint;
        let endpoint = format!("{}/v1/mission/channel", endpoint);
        rest_api::post(&endpoint, req).await
    }
}
