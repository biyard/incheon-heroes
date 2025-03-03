#![allow(non_snake_case)]
use std::collections::HashMap;

use by_macros::DioxusController;
use dioxus::prelude::*;

use crate::config;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Template {
    pub object_type: String,
    pub content: Content,
    pub item_content: ItemContent,
    pub buttons: Vec<Button>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub struct Content {
    pub title: String,
    pub description: String,
    pub image_url: String,
    pub link: Link,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub struct ItemContent {
    pub profile_text: String,
    pub profile_image_url: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub struct Button {
    pub title: String,
    pub link: Link,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub struct Link {
    pub web_url: String,
    pub mobile_web_url: String,
}

impl Template {
    pub fn new(id: &str, backup_key: &str) -> Self {
        let conf = config::get();

        let uri = conf.domain;

        let content = Content {
            title: "[공유절대금지] 인천히어로즈 개인키 백업용 메시지".to_string(),
            description: "인천히어로즈에 접속하시려면 상단 이미지를 클릭해주세요.\n\n#인천히어로즈 #개인키 #보안 \n#유출금지 #공유금지".to_string(),
            image_url: format!("https://{uri}/images/main.jpeg"),
            link: Link {
                web_url: format!("{}", uri),
                mobile_web_url: format!("{}", uri),
            },
        };

        let item_content = ItemContent {
            profile_text: format!(
                "{}인천히어로즈",
                if conf.env != "prod" {
                    "[개발환경] "
                } else {
                    ""
                }
            ),
            profile_image_url: format!("https://{uri}/logos/logo_symbol_color.png"),
        };

        let buttons = vec![Button {
            title: "내계정으로 접속".to_string(),
            link: Link {
                web_url: format!(
                    "https://{}/ko/restore/kakao?id={}&seed={}",
                    uri,
                    urlencoding::encode(id),
                    backup_key
                ),
                mobile_web_url: format!(
                    "https://{}/ko/restore/kakao?id={}&seed={}",
                    uri,
                    urlencoding::encode(id),
                    backup_key
                ),
            },
        }];

        Self {
            object_type: "feed".to_string(),
            content,
            item_content,
            buttons,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub struct KakaoResponse {
    result_code: i32,
}

#[derive(Clone, Copy, DioxusController)]
pub struct KakaoService {
    pub access_token: Signal<String>,
}

impl KakaoService {
    pub fn init() {
        let srv = Self {
            access_token: use_signal(|| "".to_string()),
        };

        use_context_provider(move || srv);
    }

    pub fn logged_in(&self) -> bool {
        !self.access_token().is_empty()
    }

    pub async fn send_message(&self, id: &str, backup_key: &str) -> dto::Result<()> {
        let template_object = Template::new(id, backup_key);
        tracing::debug!("Sending Kakao message: {:?}", template_object);

        let client = reqwest::Client::new();
        let mut form_data = HashMap::new();
        form_data.insert(
            "template_object",
            serde_json::to_string(&template_object).map_err(|e| {
                tracing::error!("serde_json Error: {:?}", e);
                dto::Error::ParsingError
            })?,
        );

        let res = client
            .post("https://kapi.kakao.com/v2/api/talk/memo/default/send")
            .header("Authorization", format!("Bearer {}", self.access_token()))
            .form(&form_data)
            .send()
            .await;

        match res {
            Ok(res) => {
                if res.status().is_success() {
                    return Ok(());
                } else {
                    tracing::error!("Failed to send Kakao message: {:?}", res);
                }
            }
            Err(e) => {
                tracing::error!("reqwest Error: {:?}", e);
            }
        }

        Err(dto::Error::KakaoSendMessageException)
    }
}
