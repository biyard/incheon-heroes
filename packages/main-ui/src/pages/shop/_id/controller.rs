use by_macros::DioxusController;
use dioxus::prelude::*;
use dioxus_translate::Language;

use crate::services::{klaytn::Klaytn, shop_contract::ShopItem};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    pub item: Resource<ShopItem>,
    pub details: Resource<(String, String)>,
}

impl Controller {
    pub fn new(lang: Language, id: String) -> std::result::Result<Self, RenderError> {
        let klaytn: Klaytn = use_context();

        let id_cloned = id.clone();
        let item = use_server_future(move || {
            let id = id_cloned.clone();
            async move {
                klaytn
                    .shop()
                    .get_item(id.clone().parse().unwrap())
                    .await
                    .unwrap()
            }
        })?;

        let details = use_server_future(move || {
            let id = id.clone();
            async move {
                let res = reqwest::get(format!(
                    "https://metadata.biyard.co/incheon-heroes/html/shop-items/{}/{}.html",
                    match lang {
                        Language::Ko => "ko",
                        Language::En => "en",
                    },
                    id
                ))
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
                let (head, body) = res.split_once("</head>").unwrap();

                let css = head
                    .split_once("<style type='text/css'>")
                    .unwrap()
                    .1
                    .split_once("</style>")
                    .unwrap()
                    .0;

                let css = format!(
                    r#"
#shop_item_details {{
{css}
}}"#
                );

                (
                    css.to_string(),
                    body.replace("line-height: 200%", "line-height: 1000%")
                        .to_string(),
                )
            }
        })?;
        let ctrl = Self { item, details };

        Ok(ctrl)
    }

    #[cfg(feature = "web")]
    pub fn inject_styles(&self) {
        let document = web_sys::window().unwrap().document().unwrap();
        if let Ok(style_element) = document.create_element("style") {
            style_element.set_inner_html(&self.details().unwrap().0);
            if let Some(head) = document.head() {
                let _ = head.append_child(&style_element);
            }
        }
    }
}
