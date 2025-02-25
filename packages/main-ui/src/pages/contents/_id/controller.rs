use by_macros::*;
use dioxus::prelude::*;
use dioxus_translate::Language;
use dto::{Content, UserContents};

use crate::config;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    pub rsc: Resource<(Content, UserContents)>,
}

impl Controller {
    pub fn new(lang: Language, id: i64) -> std::result::Result<Self, RenderError> {
        let rsc = use_server_future(move || async move {
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
}
