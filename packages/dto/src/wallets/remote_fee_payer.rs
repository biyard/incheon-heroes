use async_trait::async_trait;
use ethers::types::{Signature, H160};

use crate::{contracts::klaytn_transaction::KlaytnTransaction, FeePayerAddress, FeePayerSignature};

use super::KaiaWallet;

#[derive(Clone, Copy)]
pub struct RemoteFeePayer {
    pub endpoint: &'static str,
    pub address: H160,
}

impl RemoteFeePayer {
    pub async fn new(endpoint: &'static str) -> crate::Result<Self> {
        let ret = FeePayerAddress::get_client(endpoint)
            .get_fee_payer()
            .await?;

        Ok(Self {
            endpoint,
            address: ret.address.parse().expect("invalid feepayer address"),
        })
    }
}

#[cfg_attr(not(feature = "server"), async_trait(?Send))]
#[cfg_attr(feature = "server", async_trait)]
impl KaiaWallet for RemoteFeePayer {
    fn address(&self) -> H160 {
        self.address
    }

    async fn sign_transaction(&self, tx: &KlaytnTransaction) -> crate::Result<Signature> {
        let sig = FeePayerSignature::get_client(self.endpoint)
            .sign_transaction(tx.clone())
            .await?;

        Ok(sig.into())
    }
}
