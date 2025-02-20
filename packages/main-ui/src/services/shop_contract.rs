use std::sync::Arc;

use abi::Abi;
use dto::*;
use ethers::prelude::*;

#[derive(Debug, Clone)]
pub struct ShopContract {
    pub contract: ContractInstance<Arc<Provider<Http>>, Provider<Http>>,
}

impl ShopContract {
    pub fn new(contract_address: &str, provider: Arc<Provider<Http>>) -> Self {
        let contract = Contract::new(
            contract_address.parse::<Address>().unwrap(),
            serde_json::from_str::<Abi>(include_str!("../abi/shop.json")).unwrap(),
            provider.clone(),
        );

        Self { contract }
    }

    pub async fn list_items(&self, page: u64, size: u64) -> Result<Vec<ShopItem>> {
        let items_raw: Vec<(
            U256,
            U256,
            U256,
            String,
            String,
            U256,
            U256,
            U256,
            Address,
            Address,
            U256,
            u8,
            String,
        )> = self
            .contract
            .method("listItems", (U256::from(page), U256::from(size)))
            .map_err(|e| Error::Klaytn(e.to_string()))?
            .call()
            .await
            .map_err(|e| Error::Klaytn(e.to_string()))?;

        let items = items_raw
            .into_iter()
            .map(|item| ShopItem {
                id: item.0,
                price: item.1,
                token_id: item.2,
                name: item.3,
                image: item.4,
                supply: item.5,
                likes: item.6,
                reports: item.7,
                contract_address: item.8,
                creator: item.9,
                remaining: item.10,
                level: item.11,
                metadata: item.12,
            })
            .collect();

        Ok(items)
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ShopItem {
    id: U256,
    price: U256,
    token_id: U256,
    name: String,
    image: String,
    supply: U256,
    likes: U256,
    reports: U256,
    contract_address: Address,
    creator: Address,
    remaining: U256,
    level: u8,
    metadata: String,
}
