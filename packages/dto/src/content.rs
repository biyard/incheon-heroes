#![allow(unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v1/contents", table = contents)]
pub struct Content {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action = create)]
    pub title: String,
    #[api_model(summary, action = create)]
    #[validate(url)]
    pub thumbnail_image: String,
    #[api_model(summary, action = create)]
    #[validate(url)]
    pub source: String,
}
