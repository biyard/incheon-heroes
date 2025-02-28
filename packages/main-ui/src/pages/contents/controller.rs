use by_macros::*;
use by_types::QueryResponse;
use dioxus::prelude::*;
use dioxus_translate::Language;
use dto::{Content, ContentQueryBy, ContentSorter, ContentSummary};

use crate::{config, services::user_service::UserService};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    #[allow(dead_code)]
    pub user_service: UserService,
    pub contents: Resource<QueryResponse<ContentSummary>>,
    pub search_keyword: Signal<Option<String>>,
    pub sorter: Signal<ContentSorter>,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let search_keyword: Signal<Option<String>> = use_signal(|| None);
        let sorter: Signal<ContentSorter> = use_signal(|| ContentSorter::Popular);

        let contents: Resource<QueryResponse<ContentSummary>> = use_server_future(move || {
            let keyword = search_keyword();
            let sorter = sorter();

            async move {
                let endpoint = config::get().new_api_endpoint;

                if let Some(keyword) = keyword {
                    match Content::get_client(endpoint)
                        .search(100, None, keyword.clone())
                        .await
                    {
                        Ok(contents) => contents,
                        Err(e) => {
                            tracing::error!("Failed to query contents: {:?}", e);
                            QueryResponse::default()
                        }
                    }
                } else {
                    match Content::get_client(endpoint)
                        .query_by_custom(ContentQueryBy { sorter })
                        .await
                    {
                        Ok(contents) => contents,
                        Err(e) => {
                            tracing::error!("Failed to query contents: {:?}", e);
                            QueryResponse::default()
                        }
                    }
                }
            }
        })?;
        let ctrl = Self {
            lang,
            sorter,
            user_service: use_context(),
            contents,
            search_keyword,
        };

        Ok(ctrl)
    }

    pub async fn search(&mut self, query: String) {
        if query.is_empty() {
            self.search_keyword.set(None);
        } else {
            self.search_keyword.set(Some(query));
        }
    }
}
