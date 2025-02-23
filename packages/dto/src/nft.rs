use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub token_id: u64,
    pub event_name: String,
    pub from: Principal,
    pub to: Principal,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct Nft {
    pub owner: Principal,
    pub approved: Option<Principal>,
    pub id: u64,
    pub metadata: Metadata,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub description: String,
    pub image: String,
    pub external_url: Option<String>,
    pub background_color: Option<String>,
    pub animation_url: Option<String>,
    pub youtube_url: Option<String>,
    pub attributes: Vec<Attribute>,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub enum DisplayType {
    #[serde(rename = "number")]
    Number,

    #[serde(rename = "boost_number")]
    BoostNumber,

    #[serde(rename = "boost_percentage")]
    BoostPercentage,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub trait_type: String,
    pub display_type: Option<DisplayType>,
    pub value: String,
}
