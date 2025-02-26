use std::sync::Arc;

use aws_sdk_s3::primitives::ByteStream;
use by_axum::aide;
use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Extension, Json,
    },
};
use by_types::{AwsConfig, QueryResponse};
use contracts::incheon_contents::IncheonContentsContract;
use dto::*;
use ethers::prelude::*;
use sqlx::postgres::PgRow;
use validator::Validate;
use wallets::local_fee_payer::LocalFeePayer;
use wallets::wallet::KaiaLocalWallet;

use crate::config::{BucketConfig, ContractConfig, KlaytnConfig};

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct ContentPath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct ContentController {
    repo: ContentRepository,
    #[allow(unused)]
    contract: IncheonContentsContract<LocalFeePayer, KaiaLocalWallet>,
    pool: sqlx::Pool<sqlx::Postgres>,
    cli: aws_sdk_s3::Client,
    bucket_name: &'static str,
    asset_dir: &'static str,
}

impl ContentController {
    async fn create_bulk(
        &self,
        auth: Option<Authorization>,
        body: Vec<ContentCreateRequest>,
    ) -> Result<Json<Content>> {
        let evm_address = match auth {
            Some(Authorization::Bearer { claims }) => claims.sub,
            _ => {
                return Err(Error::Unauthorized.into());
            }
        };

        let mut tx = self.pool.begin().await?;
        let mut docs = vec![];

        for item in body {
            item.validate()?;
            let ContentCreateRequest {
                creator_id,
                title,
                thumbnail_image,
                source,
                description,
            } = item;

            if let Some(doc) = self
                .repo
                .insert_with_tx(
                    &mut *tx,
                    title,
                    thumbnail_image,
                    source,
                    description,
                    creator_id,
                )
                .await?
            {
                docs.push(doc);
            }
        }

        tx.commit().await?;

        tracing::debug!("successful insertion docs: {:?}", docs);

        let mut ids = vec![];
        let mut values = vec![];

        for doc in docs.clone() {
            let path = format!("{}/json/{:064x}.json", self.asset_dir, doc.id);
            match self
                .cli
                .put_object()
                .bucket(self.bucket_name)
                .key(&path)
                .body(ByteStream::from(
                    serde_json::json!({
                        "name": doc.title,
                        "image": doc.thumbnail_image,
                        "source": doc.source,
                        "description": doc.description,
                    })
                    .to_string()
                    .as_bytes()
                    .to_vec(),
                ))
                .content_type("application/json")
                .send()
                .await
            {
                Ok(_) => {
                    ids.push(doc.id as u64);
                    values.push(1);
                    tracing::debug!("Uploaded to s3: {}", path);
                }
                Err(e) => {
                    tracing::warn!("Failed to upload to s3 for {}: {}", doc.id, e);
                }
            }
        }

        tracing::debug!(
            "address: {} ids: {:?}, values: {:?}",
            evm_address,
            ids,
            values
        );

        self.contract.mint_batch(evm_address, ids, values).await?;

        Ok(Json(
            docs.last().ok_or(Error::NoContentAfterInsert)?.clone(),
        ))
    }

    async fn mint(&self, _auth: Option<Authorization>, _id: i64) -> Result<Json<Content>> {
        todo!()
    }

    async fn query(
        &self,
        _auth: Option<Authorization>,
        _param: ContentQuery,
    ) -> Result<QueryResponse<ContentSummary>> {
        let mut total_count = 0;
        let items: Vec<ContentSummary> = ContentSummary::query_builder()
            .order_by_created_at_desc()
            .with_count()
            .query()
            .map(|row: PgRow| {
                use sqlx::Row;
                total_count = row.get("total_count");
                row.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(QueryResponse { total_count, items })
    }
}

impl ContentController {
    pub async fn new(
        pool: sqlx::Pool<sqlx::Postgres>,
        config: &AwsConfig,
        &BucketConfig {
            name, asset_dir, ..
        }: &BucketConfig,
        provider: Arc<Provider<Http>>,
        &ContractConfig {
            incheon_contents, ..
        }: &ContractConfig,
        &KlaytnConfig {
            owner_key,
            feepayer_key,
            feepayer_address,
            ..
        }: &KlaytnConfig,
    ) -> Result<Self> {
        let repo = Content::get_repository(pool.clone());
        use aws_config::BehaviorVersion;
        use aws_config::{defaults, Region};
        use aws_sdk_s3::config::Credentials;

        let config = defaults(BehaviorVersion::latest())
            .region(Region::new(config.region))
            .credentials_provider(Credentials::new(
                config.access_key_id,
                config.secret_access_key,
                None,
                None,
                "credential",
            ));

        let config = config.load().await;
        let cli = aws_sdk_s3::Client::new(&config);

        let onwer = KaiaLocalWallet::new(&owner_key, provider.clone()).await?;
        let feepayer =
            LocalFeePayer::new(&feepayer_address, feepayer_key, provider.clone()).await?;

        let mut contract = IncheonContentsContract::new(incheon_contents, provider);
        contract.set_wallet(onwer);
        contract.set_fee_payer(feepayer);

        Ok(Self {
            repo,
            contract,
            pool,
            cli,
            bucket_name: name,
            asset_dir,
        })
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/:id", get(Self::get_content).post(Self::act_content_by_id))
            .with_state(self.clone())
            .route("/", post(Self::act_content).get(Self::list_content))
            .with_state(self.clone()))
    }

    pub async fn act_content(
        State(ctrl): State<ContentController>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<ContentAction>,
    ) -> Result<Json<Content>> {
        tracing::debug!("act_content {:?}", body);

        match body {
            ContentAction::CreateBulk(ContentCreateBulkRequest { items }) => {
                ctrl.create_bulk(auth, items).await
            }
            ContentAction::Create(item) => ctrl.create_bulk(auth, vec![item]).await,
        }
    }

    pub async fn act_content_by_id(
        State(ctrl): State<ContentController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(ContentPath { id }): Path<ContentPath>,
        Json(body): Json<ContentByIdAction>,
    ) -> Result<Json<Content>> {
        tracing::debug!("act_content_by_id {:?} {:?}", id, body);

        match body {
            ContentByIdAction::Mint(_) => ctrl.mint(auth, id).await,
        }
    }

    pub async fn get_content(
        State(ctrl): State<ContentController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(ContentPath { id }): Path<ContentPath>,
    ) -> Result<Json<Content>> {
        tracing::debug!("get_content {:?}", id);

        Ok(Json(
            Content::query_builder()
                .id_equals(id)
                .query()
                .map(Content::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
    }

    pub async fn list_content(
        State(ctrl): State<ContentController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<ContentParam>,
    ) -> Result<Json<ContentGetResponse>> {
        tracing::debug!("list_content {:?}", q);

        match q {
            ContentParam::Query(param) => Ok(Json(ContentGetResponse::Query(
                ctrl.query(auth, param).await?,
            ))),
        }
    }
}
