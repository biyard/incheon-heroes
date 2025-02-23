use candid::Principal;
use dto::{dao::ProposalResult, nft::Metadata};
use ic_cdk::{
    api::{
        call::notify_with_payment128,
        management_canister::http_request::{
            http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse,
            TransformArgs, TransformContext,
        },
    },
    query, trap,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{Error, ProposalDto};

const MAX_CYCLES: u128 = 20_851_000_000;
const POST_MAX_CYCLES: u128 = 20_949_816_000;

#[derive(serde::Deserialize, Debug)]
pub struct GetShopDataResponse {
    #[serde(rename = "item")]
    pub item: ShopData,
}

#[derive(serde::Serialize)]
pub struct ChangeShopItemRequest {
    #[serde(rename = "itemId")]
    pub item_id: u64,
    #[serde(rename = "price")]
    pub price: u64,
    #[serde(rename = "tokenId")]
    pub token_id: u64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "image")]
    pub image: String,
    #[serde(rename = "supply")]
    pub supply: u64,
    #[serde(rename = "likes")]
    pub likes: u64,
    #[serde(rename = "reports")]
    pub reports: u64,
    #[serde(rename = "contractAddress")]
    pub contract_address: String,
    #[serde(rename = "creator")]
    pub creator: String,
    #[serde(rename = "remaining")]
    pub remaining: u64,
    #[serde(rename = "level")]
    pub level: u8,
    #[serde(rename = "metadata")]
    pub metadata: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct ChangeShopItemResponse {
    #[serde(rename = "status")]
    pub status: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ShopData {
    #[serde(rename = "ContractAddress")]
    pub contract_address: String,
    #[serde(rename = "Creator")]
    pub creator: String,
    #[serde(rename = "Id")]
    pub id: u64,
    #[serde(rename = "Image")]
    pub image: String,
    #[serde(rename = "Level")]
    pub level: u8,
    #[serde(rename = "Likes")]
    pub likes: u64,
    #[serde(rename = "Metadata")]
    pub metadata: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Price")]
    pub price: u64,
    #[serde(rename = "Remaining")]
    pub remaining: u64,
    #[serde(rename = "Reports")]
    pub reports: u64,
    #[serde(rename = "Supply")]
    pub supply: u64,
    #[serde(rename = "TokenId")]
    pub token_id: u64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct CheckOwnerResponse {
    verified: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GetLevelResponse {
    level: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyProposalActionRequest {
    pub proposal: ProposalDto,
    pub rewards: Option<(Vec<Reward>, Reward)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reward {
    pub address: String,
    pub amount: u64,
}

#[derive(Default, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRewardRequest {
    pub code: String,
    pub address: String,
    #[serde(rename = "rewardXP")]
    pub reward_xp: i64,
    pub progress_date: u64,
    pub proposal_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<u64>,
    pub msg: String,
}

pub struct BackendApi {
    iuniverse: String,
    metadata_endpoint: String,
}

impl BackendApi {
    pub fn new() -> Self {
        Self {
            iuniverse: env!("IUNIVERSE_API").to_string(),
            metadata_endpoint: env!("METADATA_ENDPOINT").to_string(),
        }
    }

    pub fn update_multiple_rewards(
        &self,
        rewards: Vec<Reward>,
        progress_date: u64,
        proposal_id: u64,
        msg: String,
    ) -> () {
        let url = self.iuniverse.clone() + "/m1/icp/rewards";
        let mut headers: Vec<HttpHeader> = vec![];
        let icp_code = env!("ICP_CODE").to_string();
        if cfg!(mainnet) {
            headers.push(HttpHeader {
                name: "Strict-Transport-Security".to_string(),
                value: "max-age=31536000; includeSubDomains".to_string(),
            })
        }

        let bytes = serde_json::to_vec(&serde_json::json!({
            "code": icp_code,
            "rewards": rewards,
            "progressDate": progress_date,
            "proposalId": proposal_id,
            "msg": msg,
        }))
        .expect("Failed to serialize rewards request");

        let request = CanisterHttpRequestArgument {
            url,
            max_response_bytes: None, //optional for request
            method: HttpMethod::POST,
            headers,
            body: Some(bytes),
            transform: Some(TransformContext::from_name(
                "transform_metadata".to_string(),
                vec![],
            )),
        };

        match notify_with_payment128(
            Principal::management_canister(),
            "http_request",
            (request,),
            MAX_CYCLES,
        ) {
            Ok(_) => (),
            Err(code) => trap(format!("Failed to reward comments action: {code:?}").as_str()),
        }
    }

    pub async fn update_reward(
        &self,
        address: String,
        amount: i64,
        progress_date: u64,
        proposal_id: u64,
        comment_id: Option<u64>,
        msg: String,
    ) -> bool {
        let url = self.iuniverse.clone() + "/m1/icp/reward";
        let body = serde_json::json!(UpdateRewardRequest {
            code: env!("ICP_CODE").to_string(),
            address,
            reward_xp: amount,
            progress_date,
            msg,
            proposal_id,
            comment_id,
        });
        let bytes = serde_json::to_vec(&body.clone()).expect("Failed to serialize reward request");
        let headers = vec![HttpHeader {
            name: "Content-Length".to_string(),
            value: format!("{}", bytes.len()),
        }];
        let request = CanisterHttpRequestArgument {
            url,
            max_response_bytes: None, //optional for request
            method: HttpMethod::POST,
            headers,
            body: Some(bytes),
            transform: Some(TransformContext::from_name(
                "transform_metadata".to_string(),
                vec![],
            )),
        };
        match http_request(request, POST_MAX_CYCLES).await {
            // Ok(_) => true,
            Ok((response,)) => i32::try_from(response.status.clone().0).unwrap() < 400,
            Err((r, m)) => {
                trap(
                    format!(
                        "The http_request resulted into error. RejectionCode: {r:?}, Error: {m}"
                    )
                    .as_str(),
                );
            }
        }
    }

    pub async fn check_owner_of(&self, token_id: u64, address: String) -> bool {
        let url = format!(
            "{}/m1/icp/nft/owner/{}/{}",
            self.iuniverse.clone(),
            address,
            token_id
        )
        .to_string();

        let mut headers = vec![];
        if cfg!(mainnet) {
            headers.push(HttpHeader {
                name: "Strict-Transport-Security".to_string(),
                value: "max-age=31536000; includeSubDomains".to_string(),
            })
        }

        let request = CanisterHttpRequestArgument {
            url,
            max_response_bytes: Some(10000),
            method: HttpMethod::GET,
            headers,
            body: None,
            transform: Some(TransformContext::from_name(
                "transform_metadata".to_string(),
                vec![],
            )),
        };

        match http_request(request, MAX_CYCLES).await {
            Ok((response,)) => {
                let res = serde_json::from_slice::<CheckOwnerResponse>(&response.body)
                    .expect("Failed to deserialize JSON-RPC response");
                res.verified
            }

            Err((r, m)) => {
                trap(
                    format!(
                        "The http_request resulted into error. RejectionCode: {r:?}, Error: {m}"
                    )
                    .as_str(),
                );
            }
        }
    }

    pub async fn get_nft_level(&self, token_id: u64) -> u64 {
        let url = format!("{}/m1/icp/nft/{}/level", self.iuniverse.clone(), token_id).to_string();

        let mut headers = vec![];
        if cfg!(mainnet) {
            headers.push(HttpHeader {
                name: "Strict-Transport-Security".to_string(),
                value: "max-age=31536000; includeSubDomains".to_string(),
            })
        }

        let request = CanisterHttpRequestArgument {
            url,
            max_response_bytes: Some(10000),
            method: HttpMethod::GET,
            headers,
            body: None,
            transform: Some(TransformContext::from_name(
                "transform_metadata".to_string(),
                vec![],
            )),
        };

        match http_request(request, MAX_CYCLES).await {
            Ok((response,)) => {
                let res = serde_json::from_slice::<GetLevelResponse>(&response.body)
                    .expect("Failed to deserialize JSON-RPC response");
                res.level
            }

            Err((r, m)) => {
                trap(
                    format!(
                        "The http_request resulted into error. RejectionCode: {r:?}, Error: {m}"
                    )
                    .as_str(),
                );
            }
        }
    }

    pub fn bridge_endpoint(&self) -> String {
        self.iuniverse.clone() + "/m1/icp/bridge/nft"
    }

    pub fn bridge_nft(&self, token_id: u64, addr: String, signature: String) -> () {
        let url = self.iuniverse.clone() + "/m1/icp/bridge/nft";

        let bytes = serde_json::to_vec(&serde_json::json!({
            "tokenId": token_id,
            "address": addr,
            "signature": signature,
        }))
        .expect("Failed to serialize bridge nft request");

        let mut headers = vec![
            HttpHeader {
                name: "Content-Type".to_string(),
                value: "application/json".to_string(),
            },
            HttpHeader {
                name: "idempotency-key".to_string(),
                value: token_id.to_string(),
            },
        ];

        if cfg!(mainnet) {
            headers.push(HttpHeader {
                name: "Strict-Transport-Security".to_string(),
                value: "max-age=31536000; includeSubDomains".to_string(),
            })
        }

        let request = CanisterHttpRequestArgument {
            url,
            max_response_bytes: None,
            method: HttpMethod::POST,
            headers,
            body: Some(bytes),
            transform: Some(TransformContext::from_name(
                "transform_metadata".to_string(),
                vec![],
            )),
        };

        match notify_with_payment128(
            Principal::management_canister(),
            "http_request",
            (request,),
            MAX_CYCLES,
        ) {
            Ok(_) => (),
            Err(code) => trap(format!("Failed to notify proposal action: {code:?}").as_str()),
        }

        // match http_request(request, MAX_CYCLES).await {
        //     Ok((response,)) => {
        //         let _res = serde_json::from_slice::<serde_json::Value>(&response.body)
        //             .expect("Failed to deserialize JSON-RPC response");
        //         // TODO: status code check
        //         // if response.status != candid::Nat(200) {
        //         //     trap(format!("The http_request resulted into error. RejectionCode: {}: {}", response.status, res.to_string()).as_str());
        //         // }
        //         ()
        //     }
        //     Err((r, m)) => {
        //         trap(
        //             format!(
        //                 "The http_request resulted into error. RejectionCode: {r:?}, Error: {m}"
        //             )
        //             .as_str(),
        //         );
        //     }
        // }
    }

    pub async fn metadata(&self, token_id: u64) -> Metadata {
        let request = CanisterHttpRequestArgument {
            url: format!("{}/{}.json", self.metadata_endpoint.clone(), token_id),
            max_response_bytes: None, //optional for request
            method: HttpMethod::GET,
            headers: vec![],
            body: None,
            transform: Some(TransformContext::from_name(
                "transform_metadata".to_string(),
                vec![],
            )),
        };

        match http_request(request, MAX_CYCLES).await {
            Ok((response,)) => serde_json::from_slice::<Metadata>(&response.body)
                .expect("Failed to deserialize JSON-RPC response"),
            Err((r, m)) => trap(
                format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}")
                    .as_str(),
            ),
        }
    }

    // action_type: 1 - create, 2 - finish
    pub fn notify_proposal_action(&self, action_type: u32, req: NotifyProposalActionRequest) -> () {
        let url = self.iuniverse.clone() + "/m1/icp/discord/proposals";
        let mut req_body = serde_json::json!({
            "action": action_type,
            "proposalId": req.proposal.summary.id,
            "title": req.proposal.summary.title,
            "description": req.proposal.detail.description,
            "deadline": req.proposal.summary.deadline,
            "proposalType": req.proposal.summary.proposal_type,
            "result": match req.proposal.summary.result {
                Some(ProposalResult::Accepted) => "ACCEPTED",
                Some(ProposalResult::Rejected) => "REJECTED",
                _ => "PENDING",
            }
        });

        if let Some((rewards, proposer_reward)) = req.rewards {
            merge_values(
                &mut req_body,
                serde_json::json!({
                    "rewards": rewards,
                    "proposerReward": proposer_reward,
                }),
            );
        }

        if let Some(comment) = req.proposal.comments {
            merge_values(
                &mut req_body,
                serde_json::json!({
                    "commentId" : comment[0].id,
                    "commentNftId": comment[0].nft_id,
                    "commentContent": comment[0].contents.clone(),
                }),
            );
        };
        let bytes =
            serde_json::to_vec(&req_body).expect("Failed to serialize proposal action request");

        let mut headers = vec![
            HttpHeader {
                name: "Content-Type".to_string(),
                value: "application/json".to_string(),
            },
            HttpHeader {
                name: "idempotency-key".to_string(),
                value: req.proposal.summary.id.to_string(),
            },
        ];

        if cfg!(mainnet) {
            headers.push(HttpHeader {
                name: "Strict-Transport-Security".to_string(),
                value: "max-age=31536000; includeSubDomains".to_string(),
            })
        }

        let request = CanisterHttpRequestArgument {
            url,
            max_response_bytes: Some(2000),
            method: HttpMethod::POST,
            headers,
            body: Some(bytes),
            transform: Some(TransformContext::from_name(
                "transform_metadata".to_string(),
                vec![],
            )),
        };

        match notify_with_payment128(
            Principal::management_canister(),
            "http_request",
            (request,),
            MAX_CYCLES,
        ) {
            Ok(_) => (),
            Err(code) => trap(format!("Failed to notify proposal action: {code:?}").as_str()),
        }

        // match http_request(request, MAX_CYCLES).await {
        //     Ok((response,)) => {
        //         let _res = serde_json::from_slice::<serde_json::Value>(&response.body)
        //             .expect("Failed to deserialize JSON-RPC response");
        //     }
        //     Err((r, m)) => {
        //         trap(
        //             format!(
        //                 "The http_request resulted into error. RejectionCode: {r:?}, Error: {m}"
        //             )
        //             .as_str(),
        //         );
        //     }
        // }
    }

    pub async fn get_shop_data(&self) -> Result<GetShopDataResponse, Error> {
        let request = CanisterHttpRequestArgument {
            url: (self.iuniverse.clone() + "/m1/icp/shop/item").clone(),
            max_response_bytes: None, //optional for request
            method: HttpMethod::GET,
            headers: vec![],
            body: None,
            transform: Some(TransformContext::from_name(
                "transform_metadata".to_string(),
                vec![],
            )),
        };

        match http_request(request, MAX_CYCLES).await {
            Ok((response,)) => {
                let res = serde_json::from_slice::<GetShopDataResponse>(&response.body)
                    .expect("Failed to deserialize JSON-RPC response");
                Ok(res)
            }
            Err((r, m)) => {
                trap(
                    format!(
                        "The http_request resulted into error. RejectionCode: {r:?}, Error: {m}"
                    )
                    .as_str(),
                );
            }
        }
    }

    pub async fn change_shop_item(
        &self,
        item: ChangeShopItemRequest,
    ) -> Result<ChangeShopItemResponse, Error> {
        let request = CanisterHttpRequestArgument {
            url: (self.iuniverse.clone() + "/m1/shop").clone(),
            max_response_bytes: None, //optional for request
            method: HttpMethod::POST,
            headers: vec![],
            body: Some(serde_json::to_vec(&item).expect("Failed to serialize shop item request")),
            transform: Some(TransformContext::from_name(
                "transform_metadata".to_string(),
                vec![],
            )),
        };

        match http_request(request, POST_MAX_CYCLES).await {
            Ok((response,)) => {
                let res = serde_json::from_slice::<ChangeShopItemResponse>(&response.body)
                    .expect("Failed to deserialize JSON-RPC response");
                Ok(res)
            }
            Err((r, m)) => {
                trap(
                    format!(
                        "The http_request resulted into error. RejectionCode: {r:?}, Error: {m}"
                    )
                    .as_str(),
                );
            }
        }
    }
}
#[query]
fn transform_metadata(raw: TransformArgs) -> HttpResponse {
    let mut response = raw.response;
    response.headers.clear();
    response
}

fn merge_values(a: &mut Value, b: Value) {
    match (a, b) {
        (Value::Object(map_a), Value::Object(map_b)) => {
            for (k, v) in map_b {
                merge_values(map_a.entry(k).or_insert(Value::Null), v);
            }
        }
        (Value::Array(vec_a), Value::Array(vec_b)) => {
            vec_a.extend(vec_b);
        }
        (a, b) => {
            *a = b;
        }
    }
}
