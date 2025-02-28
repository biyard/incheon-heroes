use std::sync::Arc;

use dto::{contracts::common_contract::CommonContract, wallets::KaiaWallet, *};
use ethers::prelude::*;

use crate::utils::address;

#[derive(Debug, Clone)]
pub struct AccountContract<FeePayerWallet: KaiaWallet, UserWallet: KaiaWallet> {
    pub contract: CommonContract<FeePayerWallet, UserWallet>,
    pub nft_address: String,
}

impl<T: KaiaWallet, W: KaiaWallet> AccountContract<T, W> {
    pub fn new(contract_address: &str, nft_address: &str, provider: Arc<Provider<Http>>) -> Self {
        let contract = CommonContract::new(
            contract_address,
            include_str!("../abi/account_profile.json"),
            provider,
        );

        Self {
            contract,
            nft_address: nft_address.to_string(),
        }
    }

    pub fn set_wallet(&mut self, wallet: W) {
        self.contract.set_wallet(wallet);
    }

    pub fn set_fee_payer(&mut self, fee_payer: T) {
        self.contract.set_fee_payer(fee_payer);
    }

    pub async fn get_account_exp(&self, address: String) -> Result<U256> {
        let addr = address
            .parse::<Address>()
            .map_err(|e| Error::Klaytn(e.to_string()))?;

        let account_exp: U256 = self
            .contract
            .contract
            .method("getAccountEXP", addr)
            .map_err(|e| Error::Klaytn(e.to_string()))?
            .call()
            .await
            .map_err(|e| Error::Klaytn(e.to_string()))?;

        Ok(account_exp)
    }

    pub async fn distribute_account_exp(&self, id: i64, exp: i64) -> Result<String> {
        let addr = self
            .nft_address
            .parse::<Address>()
            .map_err(|e| Error::Klaytn(e.to_string()))?;

        let id = U256::from(id);
        let exp = U256::from(exp);

        let input = self
            .contract
            .contract
            .method::<(Address, U256, U256), ()>("distributeAccountEXP", (addr, id, exp))?
            .calldata()
            .ok_or(Error::Klaytn("calldata error".to_string()))?;

        let tx_hash = self
            .contract
            .sign_and_send_transaction_with_feepayer(input)
            .await?;

        Ok(tx_hash)
    }

    pub async fn claim_account_exp(&self) -> Result<String> {
        let input = self
            .contract
            .contract
            .method::<(), ()>("claimAccountEXPs", ())?
            .calldata()
            .ok_or(Error::Klaytn("calldata error".to_string()))?;

        let tx_hash = self
            .contract
            .sign_and_send_transaction_with_feepayer(input)
            .await?;

        Ok(tx_hash)
    }
}
