pub mod contents;

use by_axum::{
    aide,
    auth::Authorization,
    axum::{extract::State, routing::post, Extension, Json},
};
use dto::*;
use sqlx::postgres::PgRow;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct UserPath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct UserController {
    repo: UserRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl UserController {
    async fn signup_or_login(
        &self,
        _auth: Option<Authorization>,
        UserSignupOrLoginRequest {
            evm_address,
            email,
            subject,
            profile_url,
            provider,
        }: UserSignupOrLoginRequest,
    ) -> Result<Json<User>> {
        let user = match self
            .repo
            .insert(evm_address.clone(), email, subject, profile_url, provider)
            .await
        {
            Ok(user) => user,
            Err(_) => {
                User::query_builder()
                    .evm_address_equals(evm_address)
                    .query()
                    .map(|r: PgRow| r.into())
                    .fetch_one(&self.pool)
                    .await?
            }
        };

        Ok(Json(user))
    }
}

impl UserController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = User::get_repository(pool.clone());

        Self { repo, pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act_user))
            .with_state(self.clone())
            .nest(
                "/contents",
                contents::UserContentsController::new(self.pool.clone()).route()?,
            ))
    }

    pub async fn act_user(
        State(_ctrl): State<UserController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<UserAction>,
    ) -> Result<Json<User>> {
        tracing::debug!("act_user {:?}", body);
        match body {
            UserAction::SignupOrLogin(req) => _ctrl.signup_or_login(_auth, req).await,
        }
    }
}
