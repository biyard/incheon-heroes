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

    pub async fn get_item(&self, id: U256) -> Result<ShopItem> {
        let items_raw: (
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
        ) = self
            .contract
            .method("getShopItem", (id,))
            .map_err(|e| Error::Klaytn(e.to_string()))?
            .call()
            .await
            .map_err(|e| Error::Klaytn(e.to_string()))?;

        Ok(items_raw
            .try_into()
            .map_err(|_| Error::Klaytn("Invalid item".to_string()))?)
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

        let mut items = vec![];
        for item in items_raw {
            item.try_into()
                .map(|i| items.push(i))
                .map_err(|e| Error::Klaytn(e))?;
        }

        Ok(items)
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct NameParse {
    en: String,
    ko: String,
    end_date: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ShopItem {
    pub id: U256,
    pub price: U256,
    pub token_id: U256,
    pub name_en: String,
    pub name_ko: String,
    pub image: String,
    pub supply: U256,
    pub likes: U256,
    pub reports: U256,
    pub contract_address: Address,
    pub creator: Address,
    pub remaining: U256,
    pub level: u8,
    pub metadata: String,
}

impl ShopItem {
    pub fn price(&self) -> u64 {
        self.price.as_u64()
    }
}

impl
    TryFrom<(
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
    )> for ShopItem
{
    type Error = String;

    fn try_from(
        item: (
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
        ),
    ) -> std::result::Result<Self, Self::Error> {
        let now = chrono::Utc::now().timestamp();

        let np = serde_json::from_str::<NameParse>(&item.3).unwrap();
        // FIXME: It should be filtered by contract of backend.
        // It may disturb pagination.
        if np.end_date.is_some() {
            let end_date = np.end_date.unwrap().parse::<i64>().unwrap();
            if end_date < now {
                return Err("Expired item".to_string());
            }
        }

        let item = ShopItem {
            id: item.0,
            price: item.1,
            token_id: item.2,
            name_en: if np.en.starts_with("[") {
                np.en.clone()
            } else {
                format!("[NFT] {}", np.en)
            },
            name_ko: if np.ko.starts_with("[") {
                np.ko.clone()
            } else {
                format!("[NFT] {}", np.ko)
            },
            image: item.4,
            supply: item.5,
            likes: item.6,
            reports: item.7,
            contract_address: item.8,
            creator: item.9,
            remaining: item.10,
            level: item.11,
            metadata: item.12,
        };

        Ok(item)
    }
}
