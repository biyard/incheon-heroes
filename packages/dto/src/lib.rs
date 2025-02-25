use thiserror::Error as ThisError;

pub mod asset;
pub mod content;
pub mod contracts;
pub mod dao;
pub mod nft;
pub mod user;

pub use asset::*;
pub use content::*;
pub use user::*;

// FIXME: fix the error type to be more specific
pub type Result<T> = std::result::Result<T, Error>;
#[cfg(feature = "server")]
use by_axum::aide;
use validator::ValidationErrors;

#[derive(Debug, ThisError, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Error {
    #[error("Reqwest error: {0}")]
    Reqwest(String),
    #[error("Klaytn error: {0}")]
    Klaytn(String),
    #[error("Principal error: {0}")]
    PrincipalError(String),
    #[error("Candid error: {0}")]
    CandidError(String),
    #[error("Agent error: {0}")]
    AgentError(String),
    #[error("Service error: {0}")]
    DatabaseError(String),
    #[error("Asset error: {0}")]
    AssetError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
    #[error("No content after insert")]
    NoContentAfterInsert,
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e.to_string())
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Self::PrincipalError(e)
    }
}

impl From<ValidationErrors> for Error {
    fn from(e: ValidationErrors) -> Self {
        Self::ValidationError(e.to_string())
    }
}

#[cfg(feature = "server")]
impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Error::DatabaseError(e.to_string())
    }
}

#[cfg(feature = "server")]
impl by_axum::axum::response::IntoResponse for Error {
    fn into_response(self) -> by_axum::axum::response::Response {
        (
            by_axum::axum::http::StatusCode::BAD_REQUEST,
            by_axum::axum::Json(self),
        )
            .into_response()
    }
}
