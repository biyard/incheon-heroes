use by_macros::DioxusController;
use dioxus::prelude::*;

use crate::config;
use dto::*;

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    pub songs: Resource<Vec<Song>>,
    pub page: Signal<usize>,
}

impl Controller {
    pub fn new() -> std::result::Result<Self, RenderError> {
        let songs = use_server_future(|| async {
            let endpoint = config::get().main_api_endpoint;
            let endpoint = format!("{}/v1/heroes-song-contest/candidates/all", endpoint);
            let res: Result<SongResponse> = rest_api::get(&endpoint).await;
            tracing::trace!("Got songs: {:?}", res);

            match res {
                Ok(res) => res.candidates,
                Err(e) => {
                    tracing::error!("Failed to get songs: {:?}", e);
                    vec![]
                }
            }
        })?;

        let ctrl = Self {
            songs,
            page: use_signal(|| 0),
        };

        Ok(ctrl)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SongResponse {
    pub status: String,
    pub candidates: Vec<Song>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    pub key: String,
    pub title: String,
    pub lyrics: String,
    pub audio_url: String,
    pub image_url: String,
    pub email: String,
    pub nickname: String,
    pub name: String,
    pub phone: String,
    pub play_count: i32,
    pub like_count: i32,
}
