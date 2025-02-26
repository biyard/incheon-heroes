pub mod local_fee_payer;
pub mod remote_fee_payer;
pub mod wallet;

use async_trait::async_trait;
use ethers::types::{Signature, H160};

use crate::contracts::klaytn_transaction::KlaytnTransaction;

#[async_trait]
pub trait KaiaWallet {
    fn address(&self) -> H160;
    async fn sign_transaction(&self, tx: &KlaytnTransaction) -> Result<Signature, crate::Error>;
}
