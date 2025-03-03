#![allow(unused)]
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;

// use crate::User;

#[api_model(base = "/events", table = events)]
pub struct Event {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,

    #[api_model(summary, action = create)]
    pub from_address: String,
    #[api_model(summary, action = create)]
    pub to_address: String,

    #[api_model(summary, action = create, query_action = find_by_hash)]
    pub tx_hash: String,

    //user_nft_transfer table amount
    // User N:N Event
    // #[api_model(many_to_many = user_nft_transfer, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = event_id)]
    // pub user_id: User,
    #[api_model(summary, action = create)]
    pub sort_key: i64,
    #[api_model(summary, action = create)]
    pub timestamp: i64,
    #[api_model(summary, action = create)]
    pub tx_index: i64,
    #[api_model(summary, action = create)]
    pub log_index: i64,
    #[api_model(summary, action = create)]
    pub block_number: i64,

    #[api_model(summary, action = create)]
    pub operator: String,

    #[api_model(summary, action = create)]
    pub token_id: i64,
}

#[api_model(base = "/", table = user_nft_transfer)]
pub struct UserNftTransfer {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(many_to_one = users)]
    pub user_id: i64,
    #[api_model(many_to_one = events)]
    pub event_id: i64,
    #[api_model(summary, action = create)]
    pub amount: i64,
}

impl Event {
    pub fn generate_sort_key(timestamp_diff: u64, tx_index: u64, log_index: u64) -> u64 {
        /*
            tiestamp_diff   | tx_index  | log_index
            6 Byte          | 1 Byte    | 1 Byte
        */
        let mut sort_key = 0u64;
        sort_key |= timestamp_diff << 16;
        sort_key |= tx_index << 8;
        sort_key |= log_index;
        sort_key
    }
}
