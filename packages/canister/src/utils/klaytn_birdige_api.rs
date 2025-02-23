use candid::CandidType;
use ic_cdk::{
    api::management_canister::http_request::{
        http_request, CanisterHttpRequestArgument, HttpMethod,
    },
    trap,
};
use serde::{Deserialize, Serialize};

use crate::Result;

const MAX_CYCLES: u128 = 20_000_000;

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub result: JsonRpcParam,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub id: u32,
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<JsonRpcParam>,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonRpcParam {
    TransactionParam { input: String },
    String(String),
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct TransactionParam {
    pub input: String,
}

#[derive(serde::Serialize)]
struct SendRawTransactionRequest {
    pub rlp: String,
}

#[derive(serde::Deserialize)]
struct SendRawTransactionResponse {
    #[serde(rename = "txHash")]
    pub tx_hash: String,
}

pub struct KlaytnBridgeApi {
    endpoint: String,
}

impl KlaytnBridgeApi {
    pub fn new() -> Self {
        Self {
            endpoint: env!("KLAYTN_BRIDGE_ENDPOINT").to_string(),
        }
    }

    pub async fn send_raw_transaction(&self, rlp: String) -> Result<String, crate::types::Error> {
        let request = CanisterHttpRequestArgument {
            url: self.endpoint.clone(),
            max_response_bytes: None, //optional for request
            method: HttpMethod::POST,
            headers: vec![],
            body: Some(
                serde_json::to_vec(&SendRawTransactionRequest { rlp })
                    .expect("Failed to serialize JSON-RPC request"),
            ),
            transform: None, //optional for request
        };

        match http_request(request, MAX_CYCLES).await {
            Ok((response,)) => {
                let res = serde_json::from_slice::<SendRawTransactionResponse>(&response.body)
                    .expect("Failed to deserialize JSON-RPC response");
                Ok(res.tx_hash)
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
