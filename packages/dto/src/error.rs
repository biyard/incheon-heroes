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
    #[translate(
        ko = "요청에 실패했습니다. 네트워크 상태를 점검해주세요.",
        en = "Request failed. Please check your network status."
    )]
    Reqwest(String),
    #[translate(
        ko = "Kaia 블록체인 네트워크 요청에 실패했습니다. 잠시후 다시 시도해주세요.",
        en = "Kaia blockchain network request failed. Please try again later."
    )]
    Klaytn(String),
    #[translate(ko = "ICP 주소가 올바르지 않습니다.", en = "ICP address is invalid.")]
    PrincipalError(String),
    #[translate(
        ko = "ICP 블록체인의 데이터를 읽을 수 없습니다. 커뮤니티에 보고해주세요.",
        en = "Failed to read data from ICP blockchain. Please report to the community."
    )]
    CandidError(String),
    #[translate(
        ko = "ICP 블록체인 요청에 실패했습니다. 잠시후 다시 시도해주세요.",
        en = "ICP blockchain request failed. Please try again later."
    )]
    AgentError(String),
    #[translate(
        ko = "파일 업로드에 실패했습니다. 다시 시도해주세요.",
        en = "Failed to upload file. Please try again."
    )]
    UploadMetadataError(String),
    #[translate(
        ko = "데이터베이스 오류가 발생했습니다. 관리자에게 문의해주세요.",
        en = "A database error has occurred. Please contact the administrator."
    )]
    DatabaseError(String),
    #[translate(
        ko = "업로드를 위한 URL을 얻을 수 없습니다. 다시 시도해주세요.",
        en = "Failed to get URL for upload. Please try again."
    )]
    AssetError(String),
    #[translate(
        ko = "입력데이터가 모두 정상인지 확인해주세요.",
        en = "Please check if all input data is correct."
    )]
    ValidationError(String),
    #[translate(
        ko = "알 수 없는 에러가 발생했습니다. 다시 시도해주세요.",
        en = "An unknown error has occurred. Please try again."
    )]
    Unknown(String),
    #[translate(
        ko = "데이터가 정상적으로 들어가지 않았을 수 있습니다. 새로고침 후 데이터가 정상주입되었는지 확인해주세요.",
        en = "The data may not have been entered correctly. Please check if the data has been injected correctly after refreshing."
    )]
    NoContentAfterInsert,
    #[translate(
        ko = "인증 토큰 생성에 실패하였습니다. 다시 시도해주세요.",
        en = "Failed to generate authentication token. Please try again."
    )]
    JwtGenerationFailed,
    #[translate(
        ko = "인가되지 않은 사용자 입니다. 로그인 상태를 확인해주세요.",
        en = "Unauthorized user. Please check your login status."
    )]
    Unauthorized,
    #[translate(
        ko = "이미 존재하는 데이터입니다. 다른 데이터를 입력해주세요.",
        en = "Data already exists. Please enter different data."
    )]
    InvalidType,
    #[translate(ko = "데이터가 존재하지 않습니다.", en = "Data does not exist.")]
    EmptyData,

    // Contents
    #[translate(ko = "컨텐츠가 존재하지 않습니다.", en = "Content does not exist.")]
    NotFoundContent,
    #[translate(
        ko = "Kaia 지갑이 없습니다. Chrome 웹 스토어에서 Kaia 지갑을 설치해주세요.",
        en = "No Kaia wallet. Please install Kaia wallet from Chrome Web Store."
    )]
    NoKaikasWallet,
    #[translate(
        ko = "서명에 실패했습니다. 다시 시도해주세요.",
        en = "Failed to sign. Please try again."
    )]
    SignError,
    #[translate(
        ko = "지갑이 초기화되지 않았습니다. 지갑을 초기화해주세요.",
        en = "The wallet has not been initialized. Please initialize the wallet."
    )]
    WalletNotInitialized,

    #[translate(
        ko = "시스템을 어뷰징 한 것으로 추측됩니다.",
        en = "It is suspected that the system is being abused."
    )]
    MisUsed(String),

    #[translate(
        ko = "카카오 메시지 전송에 실패했습니다.",
        en = "Failed to send Kakao message."
    )]
    KakaoSendMessageException,

    #[translate(
        ko = "데이터를 가져오는데 실패하였습니다. 관리자에게 문의해주세요.",
        en = "Failed to get data. Please contact the administrator."
    )]
    ParsingError,

    #[translate(ko = "잘못된 Seed 값을 입력했습니다.", en = "Invalid Seed value.")]
    InvalidSeed,

    #[translate(ko = "이미 발행된 NFT입니다.", en = "Already minted NFT.")]
    AlreadyMinted,
    #[translate(
        ko = "NFT 발행자는 민팅할 수 없습니다.",
        en = "NFT creators can not mint their own NFT"
    )]
    CannotMintedByCreator,

    #[translate(
        ko = "민팅된 NFT에 대한 사용권에 동의해야합니다.",
        en = "You must agree to the terms of use for the minted NFT."
    )]
    MustAgreeToTerms,
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
