use by_macros::*;
use by_types::QueryResponse;
use dioxus::prelude::*;
use dioxus_translate::Language;
use dto::{Content, ContentQuery, ContentSorter, ContentSummary};

use crate::{config, services::user_service::UserService};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    #[allow(dead_code)]
    pub user_service: UserService,
    pub contents: Resource<QueryResponse<ContentSummary>>,
    pub searched_contents: Signal<Vec<ContentSummary>>,
    pub search_keyword: Signal<Option<String>>,
    pub cols: Signal<usize>,
    #[allow(dead_code)]
    pub sorter: Signal<ContentSorter>,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let search_keyword: Signal<Option<String>> = use_signal(|| None);
        let sorter: Signal<ContentSorter> = use_signal(|| ContentSorter::Popular);

        let contents: Resource<QueryResponse<ContentSummary>> = use_server_future(move || {
            let keyword = search_keyword();
            let _sort = sorter();

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
                        .query(ContentQuery::new(100))
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
            searched_contents: use_signal(|| vec![]),
            search_keyword,
            cols: use_signal(|| 4),
        };

        Ok(ctrl)
    }

    pub fn resize(&mut self, width: f64) {
        let cols = if width > 1200.0 {
            4
        } else if width > 700.0 {
            3
        } else if width > 400.0 {
            2
        } else {
            1
        };

        self.cols.set(cols);
    }

    pub fn contents_by_cols(&self) -> std::result::Result<Vec<Vec<ContentSummary>>, RenderError> {
        let cols = self.cols();
        let contents = if self.search_keyword().is_some() {
            self.searched_contents()
        } else {
            self.contents()?.items
        };
        let mut result = vec![vec![]; cols];
        for (i, content) in contents.iter().enumerate() {
            result[i % cols].push(content.clone());
        }
        tracing::debug!("Contents by cols: {:?}", result);

        Ok(result)
    }

    pub async fn search(&mut self, query: String) {
        if query.is_empty() {
            self.searched_contents.set(vec![]);
            self.search_keyword.set(None);
            return;
        }

        let endpoint = config::get().new_api_endpoint;
        match Content::get_client(endpoint)
            .search(100, None, query.clone())
            .await
        {
            Ok(contents) => {
                self.searched_contents.set(contents.items);
                self.search_keyword.set(Some(query));
            }
            Err(e) => {
                tracing::error!("Failed to query contents: {:?}", e);
            }
        };
    }
}
