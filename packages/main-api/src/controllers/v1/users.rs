pub mod contents;

use std::collections::HashMap;

use by_axum::{
    aide,
    auth::{self, Authorization},
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Extension, Json,
    },
};
use by_types::{Claims, JsonWithHeaders};
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
    ) -> Result<JsonWithHeaders<User>> {
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

        let mut claims = Claims {
            sub: user.evm_address.clone(),
            exp: 0,
            role: by_types::Role::User,
            custom: HashMap::from([("id".to_string(), user.id.to_string())]),
        };

        let jwt = auth::generate_jwt(&mut claims).map_err(|e| {
            tracing::error!("jwt generation error: {:?}", e);
            Error::JwtGenerationFailed
        })?;

        Ok(JsonWithHeaders::new(user).with_bearer_token(&jwt))
    }

    async fn get_user_by_address(
        &self,
        UserReadAction { evm_address, .. }: UserReadAction,
    ) -> Result<JsonWithHeaders<User>> {
        let user: User = User::query_builder()
            .evm_address_equals(evm_address.unwrap_or_default())
            .query()
            .map(|r: PgRow| r.into())
            .fetch_one(&self.pool)
            .await?;

        let mut claims = Claims {
            sub: user.evm_address.clone(),
            exp: 0,
            role: by_types::Role::User,
            custom: HashMap::from([("id".to_string(), user.id.to_string())]),
        };

        let jwt = auth::generate_jwt(&mut claims).map_err(|e| {
            tracing::error!("jwt generation error: {:?}", e);
            Error::JwtGenerationFailed
        })?;

        Ok(JsonWithHeaders::new(user).with_bearer_token(&jwt))
    }
}

impl UserController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = User::get_repository(pool.clone());

        Self { repo, pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act_user).get(Self::get_user))
            .route("/:id", get(Self::get_user_by_id))
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
    ) -> Result<JsonWithHeaders<User>> {
        tracing::debug!("act_user {:?}", body);
        match body {
            UserAction::SignupOrLogin(req) => _ctrl.signup_or_login(_auth, req).await,
        }
    }
    pub async fn get_user_by_id(
        State(ctrl): State<UserController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(UserPath { id }): Path<UserPath>,
    ) -> Result<Json<User>> {
        tracing::debug!("get_content {:?}", id);

        Ok(Json(
            User::query_builder()
                .id_equals(id)
                .query()
                .map(User::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
    }

    pub async fn get_user(
        State(_ctrl): State<UserController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(q): Query<UserParam>,
    ) -> Result<JsonWithHeaders<User>> {
        match q {
            UserParam::Read(param)
                if param.action == Some(UserReadActionType::GetUserByAddress) =>
            {
                _ctrl.get_user_by_address(param).await
            }
            _ => todo!(),
        }
    }
}
