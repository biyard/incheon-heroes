use ethers::{
    abi::AbiError,
    contract::ContractError,
    providers::{Middleware, ProviderError},
    signers::WalletError,
    utils::ConversionError,
};
use thiserror::Error as ThisError;

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
    #[error("Failed to upload metadata {0}")]
    UploadMetadataError(String),
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
    #[error("Failed to generate JWT")]
    JwtGenerationFailed,
    #[error("Failed to verify JWT")]
    Unauthorized,
    #[error("Invalid Type")]
    InvalidType,

    // Contents
    #[error("Content not found")]
    NotFoundContent,
    #[error("Kaikas wallet not found")]
    NoKaikasWallet,
    #[error("could not sign message")]
    SignError,
    #[error("Wallet not initialized")]
    WalletNotInitialized,

    #[error("MisUsed: {0}")]
    MisUsed(String),

    #[error("Failed to send kakao backup message")]
    KakaoSendMessageException,

    #[error("Failed to parse data")]
    ParsingError,

    #[error("Invalid seed")]
    InvalidSeed,
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

impl<M: Middleware> From<ContractError<M>> for Error {
    fn from(e: ContractError<M>) -> Self {
        Self::Klaytn(e.to_string())
    }
}

impl From<AbiError> for Error {
    fn from(e: AbiError) -> Self {
        Self::Klaytn(e.to_string())
    }
}

impl From<ProviderError> for Error {
    fn from(e: ProviderError) -> Self {
        Self::Klaytn(e.to_string())
    }
}

impl From<ConversionError> for Error {
    fn from(e: ConversionError) -> Self {
        Self::Klaytn(e.to_string())
    }
}

impl From<WalletError> for Error {
    fn from(e: WalletError) -> Self {
        Self::Klaytn(e.to_string())
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
