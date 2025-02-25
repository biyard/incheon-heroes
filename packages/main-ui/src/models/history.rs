use crate::config;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MissionHistorys {
    pub status: String,
    pub mission_infos: Vec<MissionHistory>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MissionHistory {
    pub key: String,
    #[serde(default)]
    pub mission_name: String,
    #[serde(default)]
    pub mission_name_en: String,
    #[serde(default)]
    pub mission_description: String,
    #[serde(default)]
    pub mission_description_en: String,
    #[serde(default)]
    pub mission_start_date: String,
    #[serde(default)]
    pub mission_reward_date: String,
    #[serde(default)]
    pub progress: String, //Inprogress, accepted, rejected
    #[serde(default)]
    pub experience: i32,
}

impl MissionHistorys {
    pub async fn fetch(account: String, token_id: i64) -> dto::Result<Self> {
        let endpoint = config::get().discord_api_endpoint;
        let endpoint = format!(
            "{}/v1/mission/account/{}/tokenId/{}",
            endpoint, account, token_id
        );
        rest_api::get(&endpoint).await
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenHistorys {
    pub status: String,
    pub items: Vec<TokenHistory>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TokenHistory {
    pub key: String,
    #[serde(default)]
    pub from: String,
    #[serde(default)]
    pub to: String,
    #[serde(default)]
    pub transaction_hash: String,
}

impl TokenHistorys {
    pub async fn fetch(account: String) -> dto::Result<Self> {
        let endpoint = config::get().logs_api_endpoint;
        let endpoint = format!("{}/v2/logs/user-address/{}", endpoint, account);
        rest_api::get(&endpoint).await
    }
}
