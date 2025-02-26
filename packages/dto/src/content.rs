#![allow(unused_variables)]
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use dioxus_translate::Translate;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v1/contents", table = contents, action = create_bulk(items = Vec<ContentCreateRequest>), action_by_id = [mint, like])]
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

    #[api_model(action = create, query_action = search)]
    #[validate(length(min = 1, max = 300))]
    pub description: String,

    #[api_model(many_to_one = users, action = create)]
    pub creator_id: i64,

    #[api_model(one_to_many = content_downloads, foreign_key = content_id, aggregator = count )]
    pub downloads: i64,

    #[api_model(one_to_many = content_likes, foreign_key = content_id, aggregator = count )]
    pub likes: i64,

    #[api_model(many_to_many = content_likes, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = content_id, aggregator = exist, unique)]
    pub liked: bool,
}

#[derive(
    Debug, Clone, Copy, Eq, PartialEq, serde::Serialize, serde::Deserialize, Translate, Default,
)]
#[serde(rename_all = "snake_case")]
pub enum ContentSorter {
    #[default]
    #[translate(ko => "인기순")]
    Popular,
    #[translate(ko => "최신순")]
    Newest,
}
