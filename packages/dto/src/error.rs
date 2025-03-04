use dioxus_translate::Translate;
use ethers::{
    abi::AbiError,
    contract::ContractError,
    providers::{Middleware, ProviderError},
    signers::WalletError,
    utils::ConversionError,
};

#[cfg(feature = "server")]
use by_axum::aide;
use validator::ValidationErrors;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Translate)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Error {
    Reqwest(String),
    Klaytn(String),
    PrincipalError(String),
    CandidError(String),
    AgentError(String),
    UploadMetadataError(String),
    DatabaseError(String),
    AssetError(String),
    ValidationError(String),
    Unknown(String),
    NoContentAfterInsert,
    JwtGenerationFailed,
    Unauthorized,
    InvalidType,
    EmptyData,

    // Contents
    NotFoundContent,
    NoKaikasWallet,
    SignError,
    WalletNotInitialized,

    MisUsed(String),

    KakaoSendMessageException,

    ParsingError,

    InvalidSeed,

    #[translate(ko = "이미 발행된 NFT입니다.", en = "Already minted NFT.")]
    AlreadyMinted,
    #[translate(
        ko = "NFT 발행자는 민팅할 수 없습니다.",
        en = "NFT creators can not mint their own NFT"
    )]
    CannotMintedByCreator,
}

impl std::error::Error for Error {}

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
