use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Path, Query, State},
        routing::get,
    },
};
use chrono::Utc;
use dto::{
    events::{Event, UserNftTransfer},
    nft::Metadata,
    *,
};
use sqlx::postgres::PgRow;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct UserContentsPath {
    pub id: i64,
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserGalleryItem {
    pub token_id: i64,
    pub amount: i64,
    pub tx_hash: String,
    pub metadata: Metadata,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ListNftsQuery {
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

fn default_limit() -> i64 {
    20
}

#[derive(Clone, Debug)]
pub struct UserContentsController {
    pool: sqlx::Pool<sqlx::Postgres>,
    asset_dir: &'static str,
}

impl UserContentsController {
    async fn contents_by(
        &self,
        _auth: Option<Authorization>,
        UserContentsReadAction { evm_address, .. }: UserContentsReadAction,
    ) -> Result<UserContents> {
        Ok(UserContents::query_builder()
            .evm_address_equals(evm_address.unwrap_or_default())
            .query()
            .map(|r: PgRow| r.into())
            .fetch_one(&self.pool)
            .await?)
    }

    pub fn new(pool: sqlx::Pool<sqlx::Postgres>, asset_dir: &'static str) -> Self {
        Self { pool, asset_dir }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/:id", get(Self::get_user_contents))
            .with_state(self.clone())
            .route("/:id/nfts", get(Self::list_user_minted_nfts))
            .with_state(self.clone()))
    }

    pub async fn get_user_contents(
        State(ctrl): State<UserContentsController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(UserContentsPath { id }): Path<UserContentsPath>,
    ) -> Result<Json<UserContents>> {
        Ok(Json(
            UserContents::query_builder()
                .id_equals(id)
                .query()
                .map(UserContents::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
    }

    pub async fn list_user_minted_nfts(
        State(ctrl): State<UserContentsController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(UserContentsPath { id }): Path<UserContentsPath>,
        Query(query): Query<ListNftsQuery>,
    ) -> Result<Json<UserContents>> {
        let limit = if query.limit > 100 {
            tracing::warn!("Limit too high, setting to 100");
            100
        } else {
            query.limit
        };
    
        let offset = if query.offset < 0 {
            tracing::warn!("Negative offset, setting to 0");
            0
        } else {
            query.offset
        };
    
        let user_nft_transfers = UserNftTransfer::query_builder()
            .user_id_equals(id)
            .query()
            .map(|r: PgRow| r.into())
            .fetch_all(&ctrl.pool)
            .await?;
    
        if user_nft_transfers.is_empty() {
            return Ok(Json(UserContents {
                id,
                profile_url: String::new(),
                evm_address: String::new(),
                contents: vec![],
            }));
        }
    
        let event_ids: Vec<i64> = user_nft_transfers
            .iter()
            .map(|nft: &UserNftTransfer| nft.event_id)
            .skip(offset as usize)
            .take(limit as usize)
            .collect();
    
        let mut contents = Vec::new();
    
        for event_id in event_ids {
            let event_opt = Event::query_builder()
                .id_equals(event_id)
                .query()
                .map(|r: PgRow| Event::from(r))
                .fetch_optional(&ctrl.pool)
                .await?;
    
            if let Some(event) = event_opt {
                // let amount = user_nft_transfers
                //     .iter()
                //     .find(|t| t.event_id == event.id)
                //     .map(|t| t.amount)
                //     .unwrap_or(1);
    
                let token_id = event.token_id;
                // let tx_hash = event.tx_hash.clone();
                let token_id_hex = format!("{:064x}", token_id);
                let url = format!("{}/{}.json", ctrl.asset_dir, token_id_hex);
    
                if let Ok(resp) = reqwest::get(&url).await {
                    if resp.status().is_success() {
                        if let Ok(metadata) = resp.json::<Metadata>().await {
                            contents.push(UserContent {
                                id: token_id,
                                created_at: event.created_at,
                                updated_at: Utc::now().timestamp(),
                                thumbnail_image: metadata.image.clone(),
                                source: url,
                            });
                        } else {
                            tracing::warn!("Failed to parse metadata for token_id {}", token_id);
                        }
                    } else {
                        tracing::warn!(
                            "Failed to fetch metadata for token_id {}: HTTP {}",
                            token_id,
                            resp.status()
                        );
                    }
                } else {
                    tracing::warn!("HTTP request failed for token_id {}", token_id);
                }
            }
        }
    
        let user_contents = UserContents {
            id,
            profile_url: String::new(), 
            evm_address: String::new(),
            contents,
        };
    
        Ok(Json(user_contents))
    }
    
}

