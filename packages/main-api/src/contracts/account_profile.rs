use dto::contracts::common_contract::CommonContract;
use dto::wallets::local_fee_payer::LocalFeePayer;
use dto::wallets::wallet::KaiaLocalWallet;
use ethers::prelude::*;
use std::sync::Arc;

use crate::{Error, Result};

#[derive(Debug, Clone)]
pub struct AccountProfileContract {
    pub contract: CommonContract<LocalFeePayer, KaiaLocalWallet>,
}

impl AccountProfileContract {
    pub fn new(
        contract_address: &str,
        provider: Arc<Provider<Http>>,
        owner: KaiaLocalWallet,
        feepayer: LocalFeePayer,
    ) -> Self {
        let mut contract = CommonContract::new(
            contract_address,
            include_str!("../abi/account-profile.json"),
            provider,
        );

        contract.set_wallet(owner);
        contract.set_fee_payer(feepayer);

        Self { contract }
    }

    pub async fn add_account_activity(
        &self,
        address: String,
        msg: String,
        exp: u64,
        completed_at: u64,
    ) -> Result<String> {
        let addr = address
            .parse::<Address>()
            .map_err(|e| Error::Klaytn(e.to_string()))?;

        let key = format!("{}-{}", msg, completed_at);

        tracing::debug!("key: {}", key);

        let activity = Activity {
            key,
            mission_info: msg,
            exp: U256::from(exp),
            progress_date: U256::from(completed_at),
            start_date: U256::from(0),
            end_date: U256::from(10000000000 as u64),
        };

        let input = self
            .contract
            .contract
            .method::<(H160, Activity), ()>("addAccountActivities", (addr, activity))?
            .calldata()
            .ok_or(Error::Klaytn("calldata error".to_string()))?;

        let tx_hash = self
            .contract
            .sign_and_send_transaction_with_feepayer(input)
            .await?;

        Ok(tx_hash)
    }
}

#[derive(Debug, Clone, EthAbiType)]
struct Activity {
    key: String,
    mission_info: String,
    exp: U256,
    progress_date: U256,
    start_date: U256,
    end_date: U256,
}
