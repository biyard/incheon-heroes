use std::{num::TryFromIntError, result::Result as StdResult};

use candid::CandidType;
use serde::Deserialize;

pub type Result<T = u128, E = Error> = StdResult<T, E>;

#[derive(CandidType, Debug, Deserialize)]
pub enum Error {
    // Universal errors (0-99)
    #[serde(rename = "1:Unauthorized")]
    Unauthorized,
    #[serde(rename = "2:InvalidTokenId")]
    InvalidTokenId,
    #[serde(rename = "3:InvalidTokenAmount")]
    ZeroAddress,
    #[serde(rename = "4:InvalidTokenAmount")]
    Other,

    // Proposal errors (100-199)
    #[serde(rename = "100:TooLongDescription")]
    TooLongDescription,
    #[serde(rename = "101:NotFoundProposal")]
    NotFoundProposal,
    #[serde(rename = "102:ExpiredProposal")]
    ExpiredProposal,
    #[serde(rename = "103:AlreadyFinished")]
    AlreadyFinished,
    #[serde(rename = "104:ActiveProposal")]
    ActiveProposal,
    #[serde(rename = "105:NotDiscussionProposal")]
    NotDiscussionProposal,
    #[serde(rename = "106:InvalidProposalType")]
    InvalidProposalType,

    // Vote errors (200-299)
    #[serde(rename = "200:NotAnTokenOwner")]
    NotAnTokenOwner,
    #[serde(rename = "201:NotFoundToken")]
    NotFoundToken,
    #[serde(rename = "202:AlreadyVoted")]
    AlreadyVoted,

    // Address errors (300-399)
    #[serde(rename = "300:NotFoundEvmAddress")]
    NotFoundEvmAddress,

    // Comments errors (400-499)
    #[serde(rename = "400:NotFoundComment")]
    NotFoundComment,
    #[serde(rename = "401:CommentNotInProposal")]
    CommentNotInProposal,
    #[serde(rename = "402:DuplicateNftId")]
    DuplicateNftId, // Another User Already Using NFT id
    #[serde(rename = "403:UnmodifiableNftId")]
    UnmodifiableNftId, //User Cannot Change NFT id
}

impl From<TryFromIntError> for Error {
    fn from(_: TryFromIntError) -> Self {
        Self::InvalidTokenId
    }
}

impl Error {
    pub fn err<T>(self) -> Result<T> {
        println!("{:?}", self);
        Err(self)
    }

    pub fn err_with_msg<T>(self, msg: &str) -> Result<T> {
        println!("{self:?} - {msg}");
        Err(self)
    }
}
