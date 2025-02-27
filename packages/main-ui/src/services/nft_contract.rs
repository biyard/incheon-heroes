use std::sync::Arc;

use dto::{contracts::common_contract::CommonContract, wallets::KaiaWallet, *};
use ethers::prelude::*;

#[derive(Debug, Clone)]
pub struct NftContract<FeePayerWallet: KaiaWallet, UserWallet: KaiaWallet> {
    pub contract: CommonContract<FeePayerWallet, UserWallet>,
}

impl<T: KaiaWallet, W: KaiaWallet> NftContract<T, W> {
    pub fn new(contract_address: &str, provider: Arc<Provider<Http>>) -> Self {
        let contract =
            CommonContract::new(contract_address, include_str!("../abi/nft.json"), provider);

        Self { contract }
    }

    pub fn set_wallet(&mut self, wallet: W) {
        self.contract.set_wallet(wallet);
    }

    pub fn set_fee_payer(&mut self, fee_payer: T) {
        self.contract.set_fee_payer(fee_payer);
    }

    pub async fn safe_transfer_from(&self, from: &str, to: &str, token_id: i64) -> Result<String> {
        let from = from
            .parse::<Address>()
            .map_err(|e| Error::Klaytn(e.to_string()))?;
        let to = to
            .parse::<Address>()
            .map_err(|e| Error::Klaytn(e.to_string()))?;
        let token_id = U256::from(token_id);

        let input = self
            .contract
            .contract
            .method::<(Address, Address, U256), ()>("safeTransferFrom", (from, to, token_id))?
            .calldata()
            .ok_or(Error::Klaytn("calldata error".to_string()))?;

        let tx_hash = self
            .contract
            .sign_and_send_transaction_with_feepayer(input)
            .await?;

        Ok(tx_hash)
    }
}
