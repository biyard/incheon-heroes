#![allow(unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;

#[api_model(base = "/v1/metadata", read_action = [get_presigned_uris(len = usize)], database = skip)]
pub struct Metadata {
    pub presigned_uris: Vec<String>,
}
