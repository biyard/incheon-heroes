use std::sync::Arc;

use abi::Abi;
use dto::*;
use ethers::prelude::*;

#[derive(Debug, Clone)]
pub struct ExperienceContract {
    pub contract: ContractInstance<Arc<Provider<Http>>, Provider<Http>>,
    pub nft_address: String,
}

impl ExperienceContract {
    pub fn new(contract_address: &str, nft_address: &str, provider: Arc<Provider<Http>>) -> Self {
        let contract = Contract::new(
            contract_address.parse::<Address>().unwrap(),
            serde_json::from_str::<Abi>(include_str!("../abi/experience.json")).unwrap(),
            provider.clone(),
        );

        Self {
            contract,
            nft_address: nft_address.to_string(),
        }
    }

    pub async fn get_max_exp(&self, elevel: i64, level: i64, id: i64) -> Result<U256> {
        let addr = self
            .nft_address
            .parse::<Address>()
            .map_err(|e| Error::Klaytn(e.to_string()))?;

        let items: U256 = self
            .contract
            .method(
                "simulateExperiencePoints",
                (U256::from(elevel), U256::from(level), addr, U256::from(id)),
            )
            .map_err(|e| Error::Klaytn(e.to_string()))?
            .call()
            .await
            .map_err(|e| Error::Klaytn(e.to_string()))?;

        Ok(items)
    }

    pub async fn get_nft_exp(&self, id: i64) -> Result<(U256, U256, U256)> {
        let addr = self
            .nft_address
            .parse::<Address>()
            .map_err(|e| Error::Klaytn(e.to_string()))?;
        let items: (U256, U256, U256) = self
            .contract
            .method("getNFTEXP", (addr, U256::from(id)))
            .map_err(|e| Error::Klaytn(e.to_string()))?
            .call()
            .await
            .map_err(|e| Error::Klaytn(e.to_string()))?;

        Ok(items)
    }
}
