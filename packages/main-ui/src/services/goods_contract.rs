use std::sync::Arc;

use abi::Abi;
use dto::*;
use ethers::prelude::*;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct GoodsContract {
    pub contract: ContractInstance<Arc<Provider<Http>>, Provider<Http>>,
}

impl GoodsContract {
    pub fn new(contract_address: &str, provider: Arc<Provider<Http>>) -> Self {
        let contract = Contract::new(
            contract_address.parse::<Address>().unwrap(),
            serde_json::from_str::<Abi>(include_str!("../abi/goods.json")).unwrap(),
            provider.clone(),
        );

        Self { contract }
    }

    pub async fn list_user_items(&self, sender: String) -> Result<Vec<GoodsItem>> {
        let sender = sender
            .parse::<Address>()
            .map_err(|e| Error::Klaytn(e.to_string()))?;

        let items_raw: Vec<(U256, String, U256)> = self
            .contract
            .method("listUserItems", sender)
            .map_err(|e| Error::Klaytn(e.to_string()))?
            .call()
            .await
            .map_err(|e| Error::Klaytn(e.to_string()))?;

        let mut items = vec![];
        for item in items_raw {
            let _ = item.try_into().map(|i| items.push(i));
        }

        Ok(items)
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Default, PartialEq)]
pub enum GoodsType {
    #[default]
    Goods,
    Ticket,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Default, PartialEq)]
pub struct GoodsItem {
    pub goods_type: GoodsType,
    pub name_ko: String,
    pub name_en: String,
    pub buy_date: U256,
}

impl TryFrom<(U256, String, U256)> for GoodsItem {
    type Error = String;

    fn try_from(item: (U256, String, U256)) -> std::result::Result<Self, Self::Error> {
        let parsed: Value = serde_json::from_str(&item.1).unwrap();

        let ko = parsed["ko"].as_str().unwrap();
        let en = parsed["en"].as_str().unwrap();
        let item = GoodsItem {
            goods_type: if item.0 == U256::from(0) {
                GoodsType::Goods
            } else {
                GoodsType::Ticket
            },
            name_ko: ko.to_string(),
            name_en: en.to_string(),
            buy_date: item.2,
        };

        Ok(item)
    }
}
