use std::sync::Arc;

use abi::Abi;
use ethers::prelude::*;

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
}
