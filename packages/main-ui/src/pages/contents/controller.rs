use by_macros::*;
use by_types::QueryResponse;
use dioxus::prelude::*;
use dioxus_translate::Language;
use dto::{Content, ContentQuery, ContentSummary};

use crate::{config, services::user_service::UserService};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    #[allow(dead_code)]
    pub user_service: UserService,
    pub contents: Resource<QueryResponse<ContentSummary>>,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let contents: Resource<QueryResponse<ContentSummary>> =
            use_server_future(move || async move {
                let endpoint = config::get().new_api_endpoint;
                match Content::get_client(endpoint)
                    .query(ContentQuery::new(100))
                    .await
                {
                    Ok(contents) => contents,
                    Err(e) => {
                        tracing::error!("Failed to query contents: {:?}", e);
                        QueryResponse::default()
                    }
                }
            })?;
        let ctrl = Self {
            lang,
            user_service: use_context(),
            contents,
        };

        Ok(ctrl)
    }
}
