use core::str;
use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::time::Duration;

use candid::{CandidType, Deserialize, Principal};
use dto::dao::{
    CommentData, LikeData, ProposalDetail, ProposalMetadata, ProposalResult, ProposalStatus,
    ProposalSummary, ProposalType, Reward as DAOReward,
};
use ic_cdk::{caller, println, trap};
use ic_cdk::{query, update};
use serde::Serialize;
use serde_bytes::ByteBuf;

use super::siwe_provider::service::get_address::get_address;
use crate::owner_of;
use crate::types::{Error, Result};
use crate::utils::backend_api::{BackendApi, NotifyProposalActionRequest, Reward};

thread_local! {
    pub static STATE: RefCell<State> = RefCell::default();
    pub static TIMERS: RefCell<TimerState> = RefCell::default();
}

#[derive(CandidType, Deserialize, Default)]
pub struct TimerState {
    pub timers: HashSet<u64>,
}

pub type ProposalId = u64;
pub type CommentId = u64;

#[derive(CandidType, Deserialize, Default)]
pub struct State {
    pub proposals: Vec<ProposalSummary>,
    pub proposal_details: HashMap<u64, ProposalDetail>,
    pub votes: HashMap<u64, HashMap<Principal, (u64, u64)>>,
    pub proposal_types: Vec<HashMap<String, String>>,
    pub comments: Option<HashMap<ProposalId, Vec<CommentData>>>,
    pub likes: Option<HashMap<(ProposalId, CommentId), HashMap<Principal, LikeData>>>,
    pub user_selected_nfts: Option<HashMap<u64, HashMap<Principal, u64>>>,
}

#[update]
async fn submit_proposal(
    proposal_type: u64,
    title: String,
    description: String,
    external_link: Option<String>,
    metadata: ProposalMetadata,
    deadline: u64,
) -> Result<u64> {
    if description.chars().count() > 500 {
        return Error::TooLongDescription.err();
    }
    let now = ic_cdk::api::time();
    let secs: u64 = now / 1_000_000_000;
    let created_at: u64 = secs - (secs % 60);
    let proposer = caller();
    let proposer_address = match get_address(ByteBuf::from(proposer.as_slice().to_vec())) {
        Ok(a) => a,
        _ => "".to_string(),
    };
    let api = BackendApi::new();

    let id = STATE.with(|state| {
        let state = state.borrow();
        state.proposals.len() as u64
    });
    if let ProposalMetadata::DiscussionProposal(meta) = &metadata {
        if let Some(DAOReward::Experience(experience)) = &meta.reward {
            let exp = experience.exp as i64 * -1;
            if !api
                .update_reward(
                    proposer_address.clone(),
                    exp,
                    created_at,
                    id,
                    None,
                    "Reduce Reward EXP".to_string(),
                )
                .await
            {
                trap("reduce exp request has failed");
            }
        }
    }

    let (summary, detail, delay, proposer_reward) = STATE.with(|state| {
        let mut state = state.borrow_mut();
        // let id = state.proposals.len() as u64;
        let summary = ProposalSummary {
            id,
            proposer,
            proposal_type,
            title,
            status: ProposalStatus::Active,
            result: None,
            votes: 0,
            deadline,
            total_comments: Some(0),
        };

        state.proposals.push(summary.clone());
        let proposer_reward: Option<Reward> =
            if proposal_type == ProposalType::DiscussionProposalType as u64 {
                Some(Reward {
                    address: proposer_address.clone(),
                    amount: 50,
                })
            } else {
                None
            };

        let detail = ProposalDetail {
            id,
            description,
            external_link,
            metadata,
            created_at,
            accepts: 0,
            rejects: 0,
            abstains: 0,
            voting_powers: 0,
            adopted_comment_id: None,
        };

        state.proposal_details.insert(id, detail.clone());
        state.votes.insert(id, HashMap::new());

        (summary, detail, deadline - secs, proposer_reward)
    });

    set_timer_handle(id, Some(delay));
    api.notify_proposal_action(
        1,
        NotifyProposalActionRequest {
            proposal: ProposalDto {
                summary,
                detail,
                comments: None,
            },
            rewards: Some((
                vec![],
                proposer_reward.unwrap_or(Reward {
                    address: "".to_string(),
                    amount: 0,
                }),
            )),
        },
    );

    Ok(id)
}

fn finish_proposal(proposal_id: u64) {
    if let Ok((summary, detail, proposal_type)) = STATE.with(|state: &RefCell<State>| {
        let mut state = state.borrow_mut();
        let proposal_type = state
            .proposals
            .get(proposal_id as usize)
            .unwrap()
            .proposal_type;
        let result = match ProposalType::from_u64(proposal_type) {
            Some(ProposalType::EventProposalType) => {
                // Event Proposal
                let detail = state.proposal_details.get(&proposal_id).unwrap().clone();
                let result = if detail.voting_powers < 50 {
                    Some(ProposalResult::Pending)
                } else if detail.accepts > detail.rejects {
                    Some(ProposalResult::Accepted)
                } else {
                    Some(ProposalResult::Rejected)
                };

                result
            }
            _ => None,
        };
        let proposal = state.proposals.get_mut(proposal_id as usize).unwrap();
        println!("Finish processing proposal {}", proposal.title);
        if proposal.status == ProposalStatus::Finished {
            return Error::AlreadyFinished.err();
        }
        proposal.status = ProposalStatus::Finished;
        proposal.result = result;
        Ok((
            state.proposals[proposal_id as usize].clone(),
            state.proposal_details[&proposal_id].clone(),
            proposal_type,
        ))
    }) {
        match ProposalType::from_u64(proposal_type).unwrap() {
            ProposalType::EventProposalType => {
                // Event Proposal
                BackendApi::new().notify_proposal_action(
                    2,
                    NotifyProposalActionRequest {
                        proposal: ProposalDto {
                            summary,
                            detail,
                            comments: None,
                        },
                        rewards: Some(reward_event_proposal(proposal_id)),
                    },
                );
            }
            ProposalType::DiscussionProposalType => {
                if let Some(rewards) = reward_discussion_proposal(proposal_id) {
                    BackendApi::new().update_multiple_rewards(
                        rewards,
                        summary.deadline,
                        proposal_id,
                        "Get more than 3 likes on a comment.".to_string(),
                    );
                }
            }
        }
    };
}

enum VoteResult {
    Accepted,
    Rejected,
    Pending,
}

fn reward_discussion_proposal(proposal_id: u64) -> Option<Vec<Reward>> {
    STATE.with(|state| {
        let state = state.borrow();
        if state.comments.is_none() {
            return None;
        }
        match state
            .comments
            .as_ref()
            .and_then(|comments| comments.get(&proposal_id))
        {
            Some(v) => {
                let res = v
                    .iter()
                    .filter(|comment| comment.total_likes >= 3)
                    .map(|comment| {
                        let address =
                            match get_address(ByteBuf::from(comment.commenter.as_slice().to_vec()))
                            {
                                Ok(a) => a,
                                _ => "".to_string(),
                            };
                        Reward {
                            address,
                            amount: 50,
                        }
                    })
                    .collect();
                Some(res)
            }
            _ => None,
        }
    })
}
fn reward_event_proposal(proposal_id: u64) -> (Vec<Reward>, Reward) {
    let (rewards, proposer_reward) = STATE.with(|state| {
        let state = state.borrow();
        let detail = state.proposal_details.get(&proposal_id).unwrap();
        let ret = if detail.voting_powers > 50 && detail.accepts > detail.rejects {
            VoteResult::Accepted
        } else if detail.voting_powers > 50 && detail.accepts <= detail.rejects {
            VoteResult::Rejected
        } else {
            VoteResult::Pending
        };

        let mut rewards = HashMap::new();
        for (voter, (vote, vp)) in state.votes.get(&proposal_id).unwrap() {
            match get_address(ByteBuf::from(voter.as_slice().to_vec())) {
                Ok(address) => {
                    rewards.insert(
                        address.clone(),
                        Reward {
                            address,
                            amount: match ret {
                                VoteResult::Accepted => {
                                    if *vote == 0 {
                                        10 * vp
                                    } else {
                                        5 * vp
                                    }
                                }
                                VoteResult::Rejected => {
                                    if *vote == 0 {
                                        5 * vp
                                    } else {
                                        10 * vp
                                    }
                                }
                                VoteResult::Pending => 1,
                            },
                        },
                    );
                }
                Err(_) => {
                    println!("Failed to get address for voter: {:?}", voter.to_string())
                }
            }
        }

        let summary = state.proposals.get(proposal_id as usize).unwrap();
        let proposer = match get_address(ByteBuf::from(summary.proposer.as_slice().to_vec())) {
            Ok(a) => a,
            Err(_) => {
                println!(
                    "Failed to get address for proposer: {:?}",
                    summary.proposer.to_string()
                );
                "".to_string()
            }
        };

        (
            rewards,
            match ret {
                VoteResult::Pending => Reward {
                    address: proposer,
                    amount: 10,
                },
                _ => Reward {
                    address: proposer,
                    amount: 50,
                },
            },
        )
    });

    (rewards.values().cloned().collect(), proposer_reward)
}

pub fn set_timer_handle(proposal_id: u64, delay: Option<u64>) {
    TIMERS.with(|timers| {
        let mut timers = timers.borrow_mut();
        if timers.timers.get(&proposal_id).is_none() {
            timers.timers.insert(proposal_id);
        }
    });

    let now = ic_cdk::api::time() / 1_000_000_000;

    let delay = match delay {
        Some(d) => d,
        None => {
            match STATE.with(|state| state.borrow().proposals[proposal_id as usize].deadline) <= now
            {
                true => 0,
                false => STATE
                    .with(|state| state.borrow().proposals[proposal_id as usize].deadline - now),
            }
        }
    };

    println!(
        "Set timer for proposal {} with delay(secs) {}",
        proposal_id, delay
    );
    let proposal_type =
        STATE.with(|state| state.borrow().proposals[proposal_id as usize].proposal_type);
    ic_cdk_timers::set_timer(Duration::from_secs(delay), move || {
        println!("Finish proposal {}", proposal_id);

        finish_proposal(proposal_id);

        TIMERS.with(|timers| {
            let mut timers = timers.borrow_mut();
            if timers.timers.get(&proposal_id).is_some() {
                timers.timers.remove(&proposal_id);
            }
        });

        match ProposalType::from_u64(proposal_type).unwrap() {
            ProposalType::DiscussionProposalType => {
                let delay = 60 * 60 * 24 * 3;
                ic_cdk_timers::set_timer(Duration::from_secs(delay), move || {
                    adopt_proposal_automatically(proposal_id);
                });
            }
            _ => {}
        }
    });
}

fn adopt_proposal_automatically(proposal_id: ProposalId) {
    let comment_id = STATE.with(|state| {
        let state = state.borrow();
        if let Some(proposal_detail) = state.proposal_details.get(&proposal_id) {
            if proposal_detail.adopted_comment_id.is_some() {
                return None;
            }
        }
        state
            .comments
            .as_ref()
            .and_then(|comments| comments.get(&proposal_id))
            .map(|proposal_comments| {
                proposal_comments
                    .iter()
                    .max_by(|a, b| {
                        a.total_likes
                            .cmp(&b.total_likes)
                            .then_with(|| b.created_at.cmp(&a.created_at))
                    })
                    .map(|v| v.id)
            })
            .flatten()
    });
    if let Some(id) = comment_id {
        set_adopted_comment(proposal_id, id);
    } else {
        let (proposal, detail, reward_amount) = STATE.with(|state| {
            let state = state.borrow();
            let mut reward_amount = None;
            if let Some(detail) = state.proposal_details.get(&proposal_id) {
                if let ProposalMetadata::DiscussionProposal(metadata) = &detail.metadata {
                    if let Some(DAOReward::Experience(experience)) = &metadata.reward {
                        reward_amount = Some(experience.exp);
                    }
                }
            }
            (
                state.proposals.get(proposal_id as usize).unwrap().clone(),
                state.proposal_details.get(&proposal_id).unwrap().clone(),
                reward_amount,
            )
        });
        let proposer_reward = if reward_amount.is_some() {
            let address = match get_address(ByteBuf::from(proposal.proposer.as_slice().to_vec())) {
                Ok(address) => address,
                _ => "".to_string(),
            };
            Reward {
                address,
                amount: reward_amount.unwrap() as u64,
            }
        } else {
            Reward {
                address: "".to_string(),
                amount: 0,
            }
        };
        BackendApi::new().notify_proposal_action(
            2,
            NotifyProposalActionRequest {
                proposal: ProposalDto {
                    summary: proposal,
                    detail,
                    comments: None,
                },
                rewards: Some((vec![], proposer_reward)),
            },
        )
    }
}

#[update]
async fn get_nft_level(token_id: u64) -> Result<u64> {
    let api = BackendApi::new();
    let level = api.get_nft_level(token_id).await;
    Ok(level)
}

#[update]
async fn calculate_voting_powers(token_id: u64) -> u64 {
    let api: BackendApi = BackendApi::new();
    let level = api.get_nft_level(token_id).await;
    let score = match token_id {
        10000..=10019 => 5,
        10020..=19999 => 3,
        _ => 1,
    };

    level * score
}

// vote: 0 - accept, 1 - reject, 2 - abstain
#[update]
async fn vote_proposal(proposal_id: u64, vote: String, token_id: u64) -> Result<u64> {
    let voter = caller();
    match owner_of(token_id) {
        Ok(owner) => {
            if owner != voter {
                return Error::NotAnTokenOwner.err();
            }
        }
        Err(_) => return Error::NotFoundToken.err(),
    }
    let now = ic_cdk::api::time() / 1_000_000_000;
    let vp = calculate_voting_powers(token_id).await;

    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if state.proposals[proposal_id as usize].deadline < now
            || state.proposals[proposal_id as usize].status == ProposalStatus::Finished
        {
            return Error::ExpiredProposal.err();
        }

        if let Some(votes) = state.votes.get_mut(&proposal_id) {
            if votes.contains_key(&voter) {
                return Error::AlreadyVoted.err();
            }
            votes.insert(
                voter,
                (
                    if vote == "yes" {
                        0
                    } else if vote == "no" {
                        1
                    } else {
                        2
                    },
                    vp,
                ),
            );
            state.proposals[proposal_id as usize].votes += 1;
            if vote == "yes" {
                state
                    .proposal_details
                    .get_mut(&proposal_id)
                    .unwrap()
                    .accepts += 1
            } else if vote == "no" {
                state
                    .proposal_details
                    .get_mut(&proposal_id)
                    .unwrap()
                    .rejects += 1
            } else {
                state
                    .proposal_details
                    .get_mut(&proposal_id)
                    .unwrap()
                    .abstains += 1
            }

            state
                .proposal_details
                .get_mut(&proposal_id)
                .unwrap()
                .voting_powers += vp;
            Ok(vp)
        } else {
            Error::NotFoundProposal.err()
        }
    })
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct ProposalDto {
    pub summary: ProposalSummary,
    pub detail: ProposalDetail,
    pub comments: Option<Vec<CommentData>>,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub enum Sort {
    #[serde(rename = "latest")]
    Latest,
    #[serde(rename = "oldest")]
    Oldest,
    #[serde(rename = "mosted_liked")]
    MostLiked,
    #[serde(rename = "most_commented")]
    MostCommented,
}

#[query]
pub fn get_proposal(
    proposal_id: u64,
    comment_size: Option<u64>,
    comment_page: Option<u64>,
    comment_sort_type: Option<Sort>,
) -> Result<(ProposalDto, Option<u64>)> {
    ic_cdk::api::print(format!(
        "proposal_id={} comment_size={} comment_page={}",
        proposal_id,
        comment_size.unwrap_or(10000),
        comment_page.unwrap_or(10000),
    ));
    STATE.with(|state| {
        let state = state.borrow();
        if proposal_id as usize >= state.proposals.len() {
            return Error::NotFoundProposal.err();
        }
        let nft_id = match get_selected_nft_id(proposal_id) {
            Ok(id) => id,
            _ => None,
        };
        match state.proposal_details.get(&proposal_id) {
            Some(detail) => Ok((
                ProposalDto {
                    summary: state.proposals[proposal_id as usize].clone(),
                    detail: detail.clone(),
                    comments: Some(
                        list_comments_by_type(
                            CommentType::Comment,
                            proposal_id,
                            None,
                            comment_size.unwrap_or(4),
                            comment_page.unwrap_or(0),
                            Some(comment_sort_type.unwrap_or(Sort::Latest)),
                        )
                        .unwrap(),
                    ),
                },
                nft_id,
            )),
            None => Error::NotFoundProposal.err(),
        }
    })
}

#[query]
fn list_all_proposals() -> Result<Vec<ProposalSummary>> {
    Ok(STATE.with(|state| state.borrow().proposals.iter().cloned().collect()))
}

#[query]
fn list_proposals(
    page: usize,
    size: usize,
    sort: Option<Sort>,
    status: Option<ProposalStatus>,
    result: Option<ProposalResult>,
) -> Result<Vec<ProposalSummary>> {
    Ok(STATE.with(|state| {
        let state = state.borrow();
        match sort {
            Some(Sort::Oldest) => state
                .proposals
                .iter()
                .filter(|proposal| {
                    let status_filter = match &status {
                        Some(s) => proposal.status == *s,
                        None => true,
                    };
                    let result_filter = match &result {
                        Some(r) => {
                            proposal.result.is_some() && proposal.result.clone().unwrap() == *r
                        }
                        None => true,
                    };
                    status_filter && result_filter
                })
                .skip(page * size)
                .take(size)
                .cloned()
                .collect(),
            _ => state
                .proposals
                .iter()
                .rev()
                .filter(|proposal| {
                    let status_filter = match &status {
                        Some(s) => proposal.status == *s,
                        None => true,
                    };
                    let result_filter = match &result {
                        Some(r) => {
                            proposal.result.is_some() && proposal.result.clone().unwrap() == *r
                        }
                        None => true,
                    };
                    status_filter && result_filter
                })
                .skip(page * size)
                .take(size)
                .cloned()
                .collect(),
        }
    }))
}

#[query]
fn voted(proposal_id: u64) -> bool {
    STATE.with(|state| {
        let state = state.borrow();
        match state.votes.get(&proposal_id) {
            Some(votes) => votes.contains_key(&caller()),
            None => false,
        }
    })
}

#[update]
async fn submit_comment(
    proposal_id: u64,
    parent_id: Option<u64>,
    nft_id: u64,
    contents: String,
) -> Result<u64> {
    let now: u64 = ic_cdk::api::time();
    let created_at: u64 = now / 1_000_000_000;
    let commenter: Principal = caller();
    // Check NFT Owner
    match owner_of(nft_id) {
        Ok(owner) => {
            if owner != commenter {
                return Error::NotAnTokenOwner.err();
            }
        }
        Err(_) => return Error::NotFoundToken.err(),
    }
    let (id, created_at) = STATE.with(|state| {
        let mut state = state.borrow_mut();

        let selected_nft_ids = state
            .user_selected_nfts
            .get_or_insert_with(HashMap::new)
            .entry(proposal_id)
            .or_insert_with(HashMap::new);

        if let Some(nft) = selected_nft_ids.get(&commenter) {
            if *nft != nft_id {
                return Error::UnmodifiableNftId.err();
            }
        } else {
            if selected_nft_ids.values().any(|id| *id == nft_id) {
                return Error::DuplicateNftId.err();
            }
            selected_nft_ids.insert(commenter, nft_id);
        }

        if let Some(proposal) = state.proposals.get_mut(proposal_id as usize) {
            if let Some(ref mut total_comments) = proposal.total_comments {
                *total_comments += 1;
            } else {
                proposal.total_comments = Some(1);
            }
        } else {
            return Error::NotFoundProposal.err();
        }

        let comments = state
            .comments
            .get_or_insert(HashMap::new())
            .entry(proposal_id)
            .or_insert(Vec::new());

        let id = comments.len() as u64;

        if let Some(comment_parent_id) = parent_id {
            if id >= comment_parent_id {
                comments[comment_parent_id as usize].total_replies += 1;
            } else {
                return Error::NotFoundComment.err();
            }
        }
        comments.push(CommentData {
            id,
            commenter,
            proposal_id,
            parent_id,
            nft_id,
            created_at,
            contents,
            total_likes: 0,
            total_replies: 0,
            user_liked: false,
        });
        Ok((id, created_at))
    })?;
    match get_address(ByteBuf::from(commenter.as_slice().to_vec())) {
        Ok(address) => {
            BackendApi::new()
                .update_reward(
                    address,
                    10,
                    created_at,
                    proposal_id,
                    Some(id),
                    "Comment Reward".to_string(),
                )
                .await;
        }
        Err(_) => println!(
            "Failed to get address for target: {:?}",
            commenter.to_string()
        ),
    }
    Ok(id)
}

fn has_liked_to(proposal_id: ProposalId, comment_id: CommentId) -> bool {
    let owner = caller();
    STATE.with(|state| {
        let state = state.borrow();
        if let Some(likes) = &state.likes {
            if let Some(comment_like) = likes.get(&(proposal_id, comment_id)) {
                return comment_like.contains_key(&owner);
            }
        }
        false
    })
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub enum CommentType {
    Comment,
    Reply,
}

#[query]
fn list_comments_by_type(
    target: CommentType,
    proposal_id: ProposalId,
    parent_id: Option<CommentId>,
    size: u64,
    page: u64,
    sort: Option<Sort>,
) -> Result<Vec<CommentData>> {
    ic_cdk::api::print(format!(
        "target={} proposal_id={} parent_id={} size={} page={}",
        match target {
            CommentType::Comment => 0,
            _ => 1,
        },
        proposal_id,
        parent_id.unwrap_or(0),
        size,
        page
    ));
    let filter = |comment: &&CommentData| -> bool {
        let target_filter: bool = match target {
            CommentType::Comment => comment.parent_id.is_none(),
            CommentType::Reply => comment.parent_id == parent_id,
        };
        target_filter
    };

    STATE.with(|state| {
        let state = state.borrow();
        let comments: Vec<CommentData> = if let Some(comments) = &state.comments {
            if let Some(proposal_comments) = comments.get(&proposal_id) {
                proposal_comments.iter().filter(filter).cloned().collect()
            } else {
                vec![]
            }
        } else {
            vec![]
        };
        let mut comments = comments;
        match sort {
            Some(Sort::Latest) | None => {
                comments.sort_by_key(|comment: &CommentData| Reverse(comment.id))
            }
            Some(Sort::Oldest) => comments.sort_by_key(|comment: &CommentData| comment.id),

            Some(Sort::MostLiked) => {
                comments.sort_by_key(|comment: &CommentData| Reverse(comment.total_likes))
            }
            Some(Sort::MostCommented) => {
                comments.sort_by_key(|comment: &CommentData| Reverse(comment.total_replies))
            }
        };
        let paged_comments: Vec<CommentData> = comments
            .into_iter()
            .skip((page * size) as usize)
            .take(size as usize)
            .collect();

        let result: Vec<CommentData> = paged_comments
            .into_iter()
            .map(|mut comment| {
                comment.user_liked = has_liked_to(proposal_id, comment.id);
                comment
            })
            .collect();
        Ok(result)
    })
}

#[query]
fn get_comment(proposal_id: ProposalId, comment_id: CommentId) -> Result<CommentData> {
    STATE.with(|state| {
        let state = state.borrow();
        if let Some(comments) = &state.comments {
            if let Some(proposal_comments) = comments.get(&proposal_id) {
                if let Some(comment) = proposal_comments.get(comment_id as usize) {
                    let mut comment = comment.clone();
                    comment.user_liked = has_liked_to(proposal_id, comment_id);
                    return Ok(comment);
                }
            }
            return Error::NotFoundComment.err();
        } else {
            return Error::NotFoundComment.err();
        }
    })
}

#[update]
fn toggle_comment_like(proposal_id: ProposalId, comment_id: CommentId) -> Result<(bool, u64)> {
    let user = caller();
    STATE.with(|state: &RefCell<State>| {
        let mut state = state.borrow_mut();

        let likes = state.likes.get_or_insert_with(HashMap::new);
        let proposal_likes = likes
            .entry((proposal_id, comment_id))
            .or_insert_with(HashMap::new);
        let flag = if let Some(_) = proposal_likes.remove(&user) {
            false
        } else {
            proposal_likes.insert(
                user,
                LikeData {
                    id: proposal_likes.len() as u64,
                    comment_id,
                    user,
                },
            );
            true
        };
        let comment = state
            .comments
            .as_mut()
            .unwrap()
            .get_mut(&proposal_id)
            .unwrap()
            .get_mut(comment_id as usize)
            .unwrap();
        if flag {
            comment.total_likes += 1;
        } else {
            comment.total_likes -= 1;
        }
        Ok((flag, comment.total_likes))
    })
}

#[update]
fn set_adopted_comment(proposal_id: u64, comment_id: u64) -> () {
    match STATE.with(|state| {
        let mut state = state.borrow_mut();
        let mut rewards: Vec<(Principal, u64)> = Vec::new();

        let comment = if let Some(comment) = state
            .comments
            .as_ref()
            .and_then(|comments| comments.get(&proposal_id))
            .and_then(|proposal_comment| proposal_comment.get(comment_id as usize))
        {
            comment.clone()
        } else {
            return Error::NotFoundComment.err();
        };

        let summary = match state.proposals.get(proposal_id as usize) {
            Some(summary) => {
                if summary.status != ProposalStatus::Finished {
                    return Error::ActiveProposal.err();
                } else if ProposalType::from_u64(summary.proposal_type).unwrap()
                    != ProposalType::DiscussionProposalType
                {
                    return Error::NotDiscussionProposal.err();
                }
                summary.clone()
            }
            _ => {
                return Error::NotFoundProposal.err();
            }
        };

        // - Replyer
        //  10 EXP
        for comment in state
            .comments
            .as_ref()
            .and_then(|comments| comments.get(&proposal_id))
            .unwrap()
            .into_iter()
            .filter(|comment| comment.parent_id.is_some_and(|id| id == comment_id))
        {
            rewards.push((comment.commenter.clone(), 10));
        }

        // - Liker
        //  10 EXP
        match state
            .likes
            .as_ref()
            .and_then(|likes| likes.get(&(proposal_id, comment_id)))
        {
            Some(likes) => {
                for (liker, _) in likes {
                    rewards.push((liker.clone(), 10));
                }
            }
            _ => {}
        };

        let mut commenter_reward_amount: u64 = 0;
        //Set Adopted Comment

        match state.proposal_details.get(&proposal_id) {
            Some(detail) => {
                if detail.adopted_comment_id.is_some() {
                    return Error::AlreadyFinished.err();
                }
                if let ProposalMetadata::DiscussionProposal(metadata) = &detail.metadata {
                    if let Some(DAOReward::Experience(experience)) = &metadata.reward {
                        commenter_reward_amount = experience.exp as u64;
                    }
                }
            }
            _ => {
                return Error::NotFoundProposal.err();
            }
        }
        let commenter_reward = (comment.commenter, commenter_reward_amount);

        if let Some(proposal_detail) = state.proposal_details.get_mut(&proposal_id) {
            proposal_detail.adopted_comment_id = Some(comment_id);
        }

        Ok((
            ProposalDto {
                summary: summary.clone(),
                detail: state.proposal_details.get(&proposal_id).unwrap().clone(),
                comments: Some(vec![comment]),
            },
            rewards,
            commenter_reward,
        ))
    }) {
        Ok((proposal, targets, (commenter, amount))) => {
            let mut rewards: Vec<Reward> = Vec::new();
            let commenter_reward: Option<Reward> =
                match get_address(ByteBuf::from(commenter.as_slice().to_vec())) {
                    Ok(address) => Some(Reward { address, amount }),
                    Err(_) => {
                        println!(
                            "Failed to get address for target: {:?}",
                            commenter.to_string()
                        );
                        None
                    }
                };

            for (target, amount) in targets {
                match get_address(ByteBuf::from(target.as_slice().to_vec())) {
                    Ok(address) => rewards.push(Reward { address, amount }),
                    Err(_) => {
                        println!("Failed to get address for target: {:?}", target.to_string())
                    }
                }
            }
            BackendApi::new().notify_proposal_action(
                2,
                NotifyProposalActionRequest {
                    proposal,
                    rewards: Some((
                        rewards,
                        commenter_reward.unwrap_or(Reward {
                            address: "".to_string(),
                            amount: 0,
                        }),
                    )),
                },
            )
        }
        _ => {}
    };
}

#[query]
fn get_selected_nft_id(proposal_id: u64) -> Result<Option<u64>> {
    let owner = caller();
    // state.user_selected_nfts = Option<HashMap<u64, HashMap<Principal, u64>>>
    Ok(STATE.with(|state| {
        let state = state.borrow();
        state
            .user_selected_nfts
            .as_ref()
            .and_then(|nfts| nfts.get(&proposal_id))
            .and_then(|nft_map| nft_map.get(&owner).copied())
    }))
}
