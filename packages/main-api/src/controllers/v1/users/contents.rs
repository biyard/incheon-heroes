use std::collections::HashMap;

use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::get,
        Extension, Json,
    },
};
use dto::{events::UserNftTransfer, nft::Metadata, *};
use sqlx::{postgres::PgRow, PgPool};
use crate::config::BucketConfig;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct UserContentsPath {
    pub id: i64,
}

pub struct UserGalleryItem {
    pub token_id: i64,
    pub amount: i64,
    pub tx_hash: String,
    pub metadata: Metadata,
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
    
}

impl UserContentsController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>,
        &BucketConfig {
            name, asset_dir, ..
        }: &BucketConfig) -> Self {
        Self { 
            pool,
            asset_dir,
         }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/", get(Self::list_user_contents))
            .with_state(self.clone())
            .route("/:id", get(Self::get_user_contents))
            .route("/nfts", get(Self::list_user_minted_nfts))
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

    pub async fn list_user_contents(
        State(ctrl): State<UserContentsController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<UserContentsParam>,
    ) -> Result<Json<UserContentsGetResponse>> {
        tracing::debug!("list_user_contents {:?}", q);

        match q {
            UserContentsParam::Read(param)
                if param.action == Some(UserContentsReadActionType::ContentsBy) =>
            {
                let res = ctrl.contents_by(auth, param).await?;
                Ok(Json(UserContentsGetResponse::Read(res)))
            }
            _ => todo!(),
        }
    }

    pub async fn list_user_minted_nfts(
        State(ctrl): State<UserContentsController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(UserContentsPath { id }): Path<UserContentsPath>,
    ) -> Result<Json<Vec<UserGalleryItem>>> {
        
        let rows = sqlx::query!(
            r#"
            SELECT
                e.token_id,
                unt.amount,
                e.tx_hash
            FROM user_nft_transfer AS unt
            INNER JOIN events AS e ON unt.event_id = e.id
            WHERE unt.user_id = $1
            ORDER BY unt.created_at DESC
            "#,
            id
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
                reqwest::get(&url)
                    .await
                    .ok() 
                    .and_then(|resp| resp.json::<Metadata>()) 
                    .map(|metadata| UserGalleryItem {
                        token_id,
                        amount,
                        tx_hash,
                        metadata,
                    })
                    .or_else(|| {
                        tracing::warn!("Failed to fetch or parse metadata for token_id {}", token_id);
                        None
                    })
            }
        }))
        .await
        .into_iter()
        .flatten()
        .collect();
    
        Ok(Json(gallery_items))
    }
    
    
}
