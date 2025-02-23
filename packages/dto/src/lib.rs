use thiserror::Error as ThisError;

pub mod dao;
pub mod nft;

// FIXME: fix the error type to be more specific
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, ThisError, Clone, serde::Serialize, serde::Deserialize)]
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
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e.to_string())
    }
}
