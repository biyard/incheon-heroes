use crate::config;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Songs {
    pub status: String,
    pub candidates: Vec<Song>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    pub key: String,
    #[serde(default)]
    pub nickname: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub lyrics: String,
    #[serde(default)]
    pub audio_url: String,
    pub image_url: Option<String>,
    pub nft_id: Option<i64>,
    #[serde(default)]
    pub play_count: i32,
    #[serde(default)]
    pub like_count: i32,
}

impl Songs {
    pub async fn fetch() -> dto::Result<Self> {
        let endpoint = config::get().main_api_endpoint;
        let endpoint = format!("{}/v1/heroes-song-contest/candidates/all", endpoint);
        rest_api::get(&endpoint).await
    }
}

impl Song {
    pub async fn play(&self) -> dto::Result<()> {
        let endpoint = config::get().main_api_endpoint;
        let endpoint = format!("{}/v1/heroes-song-contest/play/count", endpoint);
        let body = SongPlayRequest {
            candidate_id: self.key.clone(),
        };
        let _res: dto::Result<SongPlayResponse> = rest_api::post(&endpoint, &body).await;
        Ok(())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SongPlayRequest {
    pub candidate_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SongPlayResponse {
    pub status: String,
}
