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
                    body.replacen("line-height: 200%", "line-height: 1000%", 1)
                        .to_string(),
                )
            }
        })?;

        let ctrl = Self { item, details };

        Ok(ctrl)
    }

    pub fn description(&self, lang: Language) -> Result<&'static str, RenderError> {
        let name = match lang {
            Language::En => &self.item()?.name_en,
            Language::Ko => &self.item()?.name_ko,
        };

        match lang {
            Language::En => {
                if name.contains("등대리") {
                    Ok("DngDaery, the lighthouse keeper, acts as the guardian of the sea, observing all movements of the ocean around the clock and providing information to the heroes.")
                } else {
                    Ok("Cute spotted seals appeared!</br>Working hard today to protect the EarthSince you're a keeper of the earth, meet Ainy, Bumy and Comy.")
                }
            }
            Language::Ko => {
                if name.contains("등대리") {
                    Ok("등대리는 바다의 감시자로서 24시간 바다의 모든 움직임을 주시하고 히어로들에게 정보를 제공합니다.")
                } else {
                    Ok("귀여운 점박이 물범들이 나타났다!</br>지구를 지키기 위해 오늘도 열심히 활동하는</br>지구 지킴이 애이니, 버미, 꼬미를 만나보세요.")
                }
            }
        }
    }
}
