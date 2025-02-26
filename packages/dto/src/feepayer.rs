#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use ethers::types::{Signature, U256};

use crate::contracts::klaytn_transaction::KlaytnTransaction;

#[api_model(base = "/v1/fee-payer", read_action = get_fee_payer, database = skip)]
pub struct FeePayerAddress {
    pub address: String,
}

#[api_model(base = "/v1/fee-payer", action = sign_transaction(tx = KlaytnTransaction) database = skip)]
pub struct FeePayerSignature {
    pub r: Vec<u64>,
    pub s: Vec<u64>,
    pub v: u64,
}

impl From<Signature> for FeePayerSignature {
    fn from(signature: Signature) -> Self {
        let r = signature.r.as_ref().to_vec();
        let s = signature.s.as_ref().to_vec();

        Self {
            r,
            s,
            v: signature.v,
        }
    }
}

impl Into<Signature> for FeePayerSignature {
    fn into(self) -> Signature {
        Signature {
            r: U256(self.r.try_into().expect("invalid r")),
            s: U256(self.s.try_into().expect("invalid s")),
            v: self.v,
        }
    }
}
