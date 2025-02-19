use by_macros::DioxusController;
use dioxus::prelude::*;

use crate::models::songs::{Song, Songs};

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    pub songs: Resource<Vec<Song>>,
    pub page: Signal<usize>,
}

impl Controller {
    pub fn new() -> std::result::Result<Self, RenderError> {
        let songs = use_server_future(|| async {
            match Songs::fetch().await {
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
