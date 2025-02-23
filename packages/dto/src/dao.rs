use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct ProposalSummary {
    pub id: u64,
    pub proposer: Principal,
    pub proposal_type: u64,
    pub title: String,
    pub status: ProposalStatus,
    pub result: Option<ProposalResult>,
    pub votes: u64,
    pub deadline: u64,
    pub total_comments: Option<u64>,
}

#[derive(CandidType, Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProposalType {
    EventProposalType = 0,
    DiscussionProposalType = 1,
}

impl ProposalType {
    pub fn from_u64(value: u64) -> Option<Self> {
        match value {
            0 => Some(ProposalType::EventProposalType),
            1 => Some(ProposalType::DiscussionProposalType),
            _ => None,
        }
    }
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProposalStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "finished")]
    Finished,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProposalResult {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "pending")]
    Pending,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct ProposalDetail {
    pub id: u64,
    pub description: String,
    pub external_link: Option<String>,
    pub metadata: ProposalMetadata,
    pub created_at: u64,

    pub accepts: u64,
    pub rejects: u64,
    pub abstains: u64,
    pub voting_powers: u64,
    pub adopted_comment_id: Option<u64>,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub enum ProposalMetadata {
    #[serde(rename = "event_proposal")]
    EventProposal(EventProposalMetadata),
    #[serde(rename = "discussion_proposal")]
    DiscussionProposal(DiscussionProposalMetadata),
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct EventProposalMetadata {
    pub name: String,
    pub start_date: u64,
    pub end_date: u64,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct DiscussionProposalMetadata {
    pub purpose: String,
    pub budget: Option<String>,
    pub reward: Option<Reward>,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub enum Reward {
    Goods(Goods),
    Experience(Experience),
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct Goods {
    pub name: String,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub exp: u32,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Voting {
    #[serde(rename = "accept")]
    Accept,
    #[serde(rename = "reject")]
    Reject,
    #[serde(rename = "abstain")]
    Abstain,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct CommentData {
    pub id: u64,
    pub proposal_id: u64,
    pub parent_id: Option<u64>,
    pub commenter: Principal,
    pub nft_id: u64,
    pub created_at: u64,
    pub contents: String,
    pub total_likes: u64,
    pub total_replies: u64,
    pub user_liked: bool,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct LikeData {
    pub id: u64,
    pub comment_id: u64,
    pub user: Principal,
}
