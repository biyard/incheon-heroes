use std::sync::Arc;

use abi::Abi;
use dto::*;
use ethers::prelude::*;

#[derive(Debug, Clone)]
pub struct SbtContract {
    pub contract: ContractInstance<Arc<Provider<Http>>, Provider<Http>>,
}

impl SbtContract {
    pub fn new(contract_address: &str, provider: Arc<Provider<Http>>) -> Self {
        let contract = Contract::new(
            contract_address.parse::<Address>().unwrap(),
            serde_json::from_str::<Abi>(include_str!("../abi/holder.json")).unwrap(),
            provider.clone(),
        );

        Self { contract }
    }

    pub async fn list_token_ids_by_address(&self, addr: String) -> Result<Vec<U256>> {
        let addr = addr
            .parse::<Address>()
            .map_err(|e| Error::Klaytn(e.to_string()))?;

        let items_raw: Vec<U256> = self
            .contract
            .method("getUserTokenIdList", (addr,))
            .map_err(|e| Error::Klaytn(e.to_string()))?
            .call()
            .await
            .map_err(|e| Error::Klaytn(e.to_string()))?;

        Ok(items_raw)
    }

    pub async fn get_token_uri(&self, token_id: U256) -> Result<String> {
        let uri: String = self
            .contract
            .method("uri", (token_id,))
            .map_err(|e| Error::Klaytn(e.to_string()))?
            .call()
            .await
            .map_err(|e| Error::Klaytn(e.to_string()))?;

        Ok(uri)
    }

    pub async fn balance_of(&self, addr: String) -> Result<U256> {
        let addr = addr
            .parse::<Address>()
            .map_err(|e| Error::Klaytn(e.to_string()))?;

        let balance: U256 = self
            .contract
            .method("balanceOf", (addr,))
            .map_err(|e| Error::Klaytn(e.to_string()))?
            .call()
            .await
            .map_err(|e| Error::Klaytn(e.to_string()))?;

        Ok(balance)
    }
}
