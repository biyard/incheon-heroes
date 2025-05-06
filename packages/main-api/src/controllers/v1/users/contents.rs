use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::get,
        Extension, Json,
    },
};
use dto::{nft::Metadata, *};
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
        if query.limit > 100 {
            tracing::warn!("Limit is too high, setting to 100");
            let limit = 100;
        } else {
            let limit = query.limit;
        }
        if query.offset < 0 {
            tracing::warn!("Offset is negative, setting to 0");
            let offset = 0;
        } else {
            let offset = query.offset;
        }

        let rows = sqlx::query(
            r#"
            SELECT
                e.token_id,
                unt.amount,
                e.tx_hash
            FROM user_nft_transfer AS unt
            INNER JOIN events AS e ON unt.event_id = e.id
            WHERE unt.user_id = $1
            ORDER BY unt.created_at DESC
            LIMIT $2
            OFFSET $3
            "#,
            id,
            limit,
            offset
        )
        .fetch_all(&ctrl.pool)
        .await?;

        let gallery_items: Vec<UserGalleryItem> = futures::future::join_all(rows.iter().map(|row| {
            let token_id_hex = format!("{:064x}", row.token_id);
            let url = format!("{}/{}.json", ctrl.asset_dir, token_id_hex);
            let token_id = row.token_id;
            let amount = row.amount;
            let tx_hash = row.tx_hash.clone();

            async move {
                match reqwest::get(&url).await {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            match resp.json::<Metadata>().await {
                                Ok(metadata) => Some(UserGalleryItem {
                                    token_id,
                                    amount,
                                    tx_hash,
                                    metadata,
                                }),
                                Err(err) => {
                                    tracing::warn!("Failed to parse metadata for token_id {}: {}", token_id, err);
                                    None
                                }
                            }
                        } else {
                            tracing::warn!(
                                "Failed to fetch metadata for token_id {}: HTTP status {}",
                                token_id,
                                resp.status()
                            );
                            None
                        }
                    }
                    Err(err) => {
                        tracing::warn!("Failed to fetch metadata for token_id {}: {}", token_id, err);
                        None
                    }
                }
            }
        }))
        .await
        .into_iter()
        .flatten()
        .collect();

        Ok(Json(gallery_items))
    }
}
