use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        extract::{Query, State},
        routing::get,
        Extension, Json,
    },
};
use dto::*;
use sqlx::postgres::PgRow;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct UserContentsPath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct UserContentsController {
    pool: sqlx::Pool<sqlx::Postgres>,
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
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/", get(Self::list_user_contents))
            .with_state(self.clone()))
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
}
