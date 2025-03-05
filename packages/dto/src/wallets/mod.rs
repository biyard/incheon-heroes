#[cfg(feature = "web")]
pub mod kaikas_browser;
pub mod kaikas_wallet;
pub mod local_fee_payer;
pub mod remote_fee_payer;
pub mod wallet;
pub mod kaikas_browser;

use async_trait::async_trait;
use ethers::types::{Signature, H160};

use crate::contracts::klaytn_transaction::KlaytnTransaction;

#[cfg_attr(not(feature = "server"), async_trait(?Send))]
#[cfg_attr(feature = "server", async_trait)]
pub trait KaiaWallet {
    fn address(&self) -> H160;
    async fn sign_transaction(&self, tx: &KlaytnTransaction) -> Result<Signature, crate::Error>;
}
