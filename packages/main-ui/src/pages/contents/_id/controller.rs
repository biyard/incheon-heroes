use by_macros::*;
use dioxus::prelude::*;
use dioxus_translate::Language;
use dto::{Content, ContentSummary, UserContents};

use crate::config;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    pub lang: Language,
    pub rsc: Resource<(Content, UserContents)>,
}

impl Controller {
    pub fn new(lang: Language, id: i64) -> std::result::Result<Self, RenderError> {
        tracing::debug!("log1");
        let rsc = use_server_future(move || async move {
            tracing::debug!("log2");

            let endpoint = config::get().new_api_endpoint;
            tracing::debug!("endpoint: {endpoint}");

            let content = Content::get_client(endpoint)
                .get(id)
                .await
                .unwrap_or_default();
            let user = UserContents::get_client(endpoint)
                .get(content.creator_id)
                .await
                .unwrap_or_default();

            (content, user)
        })?;

        let ctrl = Self { lang, rsc };

        Ok(ctrl)
    }
    pub fn get_nft_data(&self) -> (Content, UserContents) {
        match self.rsc.value()() {
            Some(v) => (
                Content {
                    id: 1,
                    created_at: 1740478684,
                    updated_at: 1740478684,
                    title: "Heroes".to_string(),
                    thumbnail_image: "https://metadata.dagit.club/images/c5ebc4d0-7492-4dea-8b9a-6da41b7a5acf.jpg".to_string(),
                    source: "dagit.club".to_string(),
                    description: "Text".to_string(),
                    creator_id: 1,
                },
                UserContents { 
                    id: 1, 
                    profile_url: "https://metadata.dagit.club/images/c5ebc4d0-7492-4dea-8b9a-6da41b7a5acf.jpg".to_string(), 
                    evm_address: "0x07e3274ae79ddac23344039a2d134ec3dbb83575".to_string(), 
                    contents: vec![
                        ContentSummary{ 
                        id: 1, 
                        created_at: 1740478684, 
                        updated_at: 1740478684, 
                        title: "text".to_string(), 
                        thumbnail_image: "https://metadata.dagit.club/images/c5ebc4d0-7492-4dea-8b9a-6da41b7a5acf.jpg".to_string(), 
                        source: "dagit.club".to_string() 
                        },
                        ContentSummary{ 
                            id: 2, 
                            created_at: 1740478684, 
                            updated_at: 1740478684, 
                            title: "text".to_string(), 
                            thumbnail_image: "https://metadata.dagit.club/images/c5ebc4d0-7492-4dea-8b9a-6da41b7a5acf.jpg".to_string(), 
                            source: "dagit.club".to_string() 
                            },
                            ContentSummary{ 
                                id: 3, 
                                created_at: 1740478684, 
                                updated_at: 1740478684, 
                                title: "text".to_string(), 
                                thumbnail_image: "https://metadata.dagit.club/images/c5ebc4d0-7492-4dea-8b9a-6da41b7a5acf.jpg".to_string(), 
                                source: "dagit.club".to_string() 
                                }
                        ] },
            ),

            None => (Content::default(), UserContents::default()),
        }
    }
}
