use by_macros::*;
use dioxus::prelude::*;

use crate::models::{nft_metadata::NftMetadata, songs::Song};

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    pub song: Resource<Song>,
    pub is_playing: Signal<bool>,
}

impl Controller {
    pub fn new(id: String) -> std::result::Result<Self, RenderError> {
        let song = use_server_future(move || {
            let id = id.clone();

            async move {
                match Song::fetch(&id).await {
                    Ok(mut res) => {
                        if let Some(ref id) = res.nft_id {
                            res.image_url = Some(NftMetadata::fetch(*id).await.unwrap().image);
                        }

                        res
                    }
                    Err(e) => {
                        tracing::error!("Failed to get songs: {:?}", e);
                        Song::default()
                    }
                }
            }
        })?;
        let ctrl = Self {
            song,
            is_playing: use_signal(|| false),
        };

        Ok(ctrl)
    }
}
