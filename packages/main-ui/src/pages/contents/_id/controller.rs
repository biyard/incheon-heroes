use by_macros::*;
use dioxus::prelude::*;
use dioxus_popup::PopupService;
use dioxus_translate::Language;
use dto::{Content, UserContents};

use crate::{config, pages::minting_popup::MintingPopup, route::Route};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    pub rsc: Resource<(Content, UserContents)>,
    pub id: ReadOnlySignal<i64>,
    pub popup: PopupService,
    pub path: Memo<String>,
}

impl Controller {
    pub fn new(lang: Language, id: ReadOnlySignal<i64>) -> std::result::Result<Self, RenderError> {
        let rsc = use_server_future(move || {
            let id = id();
            async move {
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
            }
        })?;

        let route: Route = use_route();

        let path = use_memo(move || {
            let _ = id();

            format!("{route}")
        });

        let ctrl = Self {
            lang,
            rsc,
            id,
            path,
            popup: use_context(),
        };

        use_context_provider(move || ctrl);

        Ok(ctrl)
    }

    pub async fn handle_share(&self) {
        let result = wasm_bindgen_futures::JsFuture::from(
            web_sys::window()
                .unwrap()
                .navigator()
                .clipboard()
                .write_text(&self.path()),
        )
        .await;

        match result {
            Ok(_) => {
                btracing::info!("Copied sharing URL: {}", self.path());
            }
            Err(e) => {
                btracing::error!("Failed to copy sharing URL: {:?}", e);
            }
        }
    }

    pub fn opensea_url(&self) -> String {
        let opensea_endpoint = config::get().opensea_endpoint;
        let contract = config::get().contracts.incheon_contents.to_lowercase();
        format!("{}/{}/{}", opensea_endpoint, contract, self.id())
    }

    pub async fn handle_like(&mut self) {
        let endpoint = config::get().new_api_endpoint;
        let _ = Content::get_client(endpoint)
            .like(self.id())
            .await
            .unwrap_or_default();
        self.rsc.restart();
        btracing::info!("Liked!");
    }

    pub fn open_minting_popup(&mut self) {
        let lang = self.lang;
        self.popup.open(rsx! {
            MintingPopup { lang, id: self.id }
        });
    }
}
