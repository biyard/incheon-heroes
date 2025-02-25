use std::sync::Arc;

use abi::Abi;
use ethers::prelude::*;

use crate::{Error, Result};

#[derive(Debug, Clone)]
pub struct IncheonContentsContract {
    pub contract: ContractInstance<Arc<Provider<Http>>, Provider<Http>>,
}

impl IncheonContentsContract {
    pub fn new(contract_address: &str, provider: Arc<Provider<Http>>) -> Self {
        let contract = Contract::new(
            contract_address.parse::<Address>().unwrap(),
            serde_json::from_str::<Abi>(include_str!("./abi/incheon-contents.json")).unwrap(),
            provider.clone(),
        );

        Self { contract }
    }

    pub async fn mint_batch(&self, addr: String, ids: Vec<u64>, values: Vec<u64>) -> Result<()> {
        let addr = addr
            .parse::<Address>()
            .map_err(|e| Error::Klaytn(e.to_string()))?;

        let ids: Vec<U256> = ids.into_iter().map(|e| U256::from(e)).collect();
        let values: Vec<U256> = values.into_iter().map(|e| U256::from(e)).collect();

        let _: () = self
            .contract
            .method("mintBatch", (addr, ids, values))
            .map_err(|e| Error::Klaytn(e.to_string()))?
            .call()
            .await
            .map_err(|e| Error::Klaytn(e.to_string()))?;

        Ok(())
    }
}
