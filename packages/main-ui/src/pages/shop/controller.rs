use dioxus::prelude::*;

use crate::services::klaytn::Klaytn;

#[derive(Debug, Clone, Copy)]
pub struct Controller {}

impl Controller {
    pub fn new() -> std::result::Result<Self, RenderError> {
        let klaytn: Klaytn = use_context();

        let _ = use_server_future(move || async move {
            match klaytn.shop().list_items(0, 1000).await {
                Ok(res) => {
                    tracing::debug!("{:?}", res);
                }
                Err(e) => {
                    tracing::error!("{:?}", e);
                }
            };

            ()
        });
        let ctrl = Self {};
        use_context_provider(|| ctrl);

        Ok(ctrl)
    }
}
