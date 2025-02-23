use thiserror::Error as ThisError;

pub mod asset;
pub mod content;
pub mod dao;
pub mod nft;

pub use asset::*;
pub use content::*;

// FIXME: fix the error type to be more specific
pub type Result<T> = std::result::Result<T, Error>;
#[cfg(feature = "server")]
use by_axum::aide;

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
    #[error("Unknown error: {0}")]
    Unknown(String),
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
