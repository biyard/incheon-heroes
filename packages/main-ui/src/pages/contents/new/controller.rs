use std::str::FromStr;

use by_macros::*;
use dioxus::{prelude::*, CapturedError};
use dioxus_translate::Language;
use dto::AssetPresignedUris;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};

use crate::config;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let ctrl = Self { lang };

        Ok(ctrl)
    }
}

pub async fn handle_upload(
    file_bytes: Vec<u8>,
    ext: String,
) -> std::result::Result<String, CapturedError> {
    let cli = AssetPresignedUris::get_client(config::get().new_api_endpoint);
    let res = match cli
        .get_presigned_uris(1, dto::FileType::from_str(&ext).unwrap_or_default())
        .await
    {
        Ok(res) => res,
        Err(e) => {
            tracing::error!("Failed to get presigned uris: {:?}", e);
            return Err(e.into());
        }
    };

    let presigned_uri = res.presigned_uris.first().context("No presigned uri")?;
    let uri = res.uris.first().context("No uri")?;

    use infer;

    let kind = infer::get(&file_bytes).context("Failed to infer file type")?;
    tracing::debug!("Inferred file type: {:?}", kind);

    let content_type = kind.mime_type().to_string();
    tracing::debug!("Content type: {:?}", content_type);
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_str(&content_type).context("could not convert content type to header")?,
    );

    reqwest::Client::new()
        .put(presigned_uri)
        .body(file_bytes)
        .headers(headers)
        .send()
        .await
        .context("Failed to upload file")?;

    Ok(uri.to_string())
}
