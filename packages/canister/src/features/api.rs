use std::collections::HashMap;

use candid::CandidType;
use ic_cdk::query;
use serde::{Deserialize, Serialize};

use crate::version;


#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct HttpApiResponse {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

#[derive(CandidType, Debug, Clone, Serialize, Deserialize)]
pub struct HttpApiRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl HttpApiResponse {
    pub fn build<T>(body: T) -> Self
    where
        T: serde::Serialize,
    {
        let body = serde_json::to_vec(&body).unwrap();

        HttpApiResponse {
            status_code: 200,
            headers: vec![
                ("Content-Type".to_string(), "application/json".to_string()),
                ("Content-Length".to_string(), body.len().to_string()),
            ],
            body,
        }
    }

    pub fn with_status(mut self, status: u16) -> Self {
        self.status_code = status;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionResponse {
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: u32,
    pub message: String,
}

impl ErrorResponse {
    pub fn new(code: u32, message: String) -> Self {
        ErrorResponse { code, message }
    }
}

#[query]
async fn http_request(req: HttpApiRequest) -> HttpApiResponse {
    if req.url == "/version" {
        HttpApiResponse::build(VersionResponse {
            version: version(),
        })
    } else if let (true, m) = matcher(req.url.clone(), "/dao/proposals/:id".to_owned()) {
        let proposal_id = m.get("id").unwrap().parse().unwrap();
        match crate::features::dao::get_proposal(proposal_id, None, None, None) {
            Ok(proposal) => HttpApiResponse::build(proposal),
            Err(_) => HttpApiResponse::build(ErrorResponse::new(404, format!("Not Found: #{} Proposal", proposal_id)))
            .with_status(404),
        }
    } else {
        HttpApiResponse::build(ErrorResponse::new(404, format!("Not Found: {}", req.url)))
            .with_status(404)
    }
}

fn matcher(url: String, route: String) -> (bool, HashMap<String, String>) {
    let parts:Vec<_> = url.split("?").collect(); 
    let url = parts[0].split("/");
    let route = route.split("/");
    let mut m = HashMap::new();

    for (u, r) in url.zip(route) {
        if r.starts_with(":") {
            m.insert(r.replace(":", ""), u.to_string());
        } else if u != r {
            return (false, m);
        }
    }

    (true, m)
}
