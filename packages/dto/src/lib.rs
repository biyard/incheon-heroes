pub mod dao;
pub mod nft;

// FIXME: fix the error type to be more specific
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Error {
    Reqwest(String),
    Klaytn(String),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e.to_string())
    }
}
