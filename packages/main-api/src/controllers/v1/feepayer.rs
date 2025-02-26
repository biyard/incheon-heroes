use std::sync::Arc;

use by_axum::{
    auth::Authorization,
    axum::{extract::State, routing::post, Extension, Json},
};
use dto::*;
use ethers::providers::{Http, Provider};
use wallets::{local_fee_payer::LocalFeePayer, KaiaWallet};

use crate::config::KlaytnConfig;

#[derive(Clone, Debug)]
pub struct FeePayerController {
    feepayer: LocalFeePayer,
}

impl FeePayerController {
    async fn sign_transaction(
        &self,
        _auth: Option<Authorization>,
        FeePayerSignatureSignTransactionRequest { tx }: FeePayerSignatureSignTransactionRequest,
    ) -> Result<FeePayerSignature> {
        let tx = tx.into();
        let sig = self.feepayer.sign_transaction(&tx).await?;
        Ok(sig.into())
    }
}

impl FeePayerController {
    pub async fn new(
        &KlaytnConfig {
            feepayer_address,
            feepayer_key,
            ..
        }: &KlaytnConfig,
        provider: Arc<Provider<Http>>,
    ) -> Result<Self> {
        let feepayer = LocalFeePayer::new(&feepayer_address, feepayer_key, provider).await?;

        Ok(Self { feepayer })
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act_fee_payer).get(Self::get_fee_payer))
            .with_state(self.clone()))
    }

    pub async fn act_fee_payer(
        State(ctrl): State<FeePayerController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<FeePayerSignatureAction>,
    ) -> Result<Json<FeePayerSignature>> {
        match body {
            FeePayerSignatureAction::SignTransaction(param) => {
                let res = ctrl.sign_transaction(_auth, param).await?;
                Ok(Json(res))
            }
        }
    }

    pub async fn get_fee_payer(
        State(ctrl): State<FeePayerController>,
        Extension(_auth): Extension<Option<Authorization>>,
    ) -> Result<Json<FeePayerAddress>> {
        Ok(Json(FeePayerAddress {
            address: ctrl.feepayer.address().to_string(),
        }))
    }
}
