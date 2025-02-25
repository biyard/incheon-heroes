use std::{str::FromStr, sync::Arc};

use abi::Abi;
use dto::*;
use ethers::prelude::*;

#[derive(Debug, Clone)]
pub struct MissionContract {
    pub contract: ContractInstance<Arc<Provider<Http>>, Provider<Http>>,
    pub nft_address: String,
    pub experience_address: String,
}

impl MissionContract {
    pub fn new(
        contract_address: &str,
        nft_address: &str,
        experience_address: &str,
        provider: Arc<Provider<Http>>,
    ) -> Self {
        let contract = Contract::new(
            contract_address.parse::<Address>().unwrap(),
            serde_json::from_str::<Abi>(include_str!("../abi/mission.json")).unwrap(),
            provider.clone(),
        );

        Self {
            contract,
            nft_address: nft_address.to_string(),
            experience_address: experience_address.to_string(),
        }
    }

    pub async fn list_daily_missions(&self, id: i64) -> Result<(Vec<Mission>, Vec<Mission>)> {
        let nft_addr = self
            .nft_address
            .parse::<Address>()
            .map_err(|e| Error::Klaytn(e.to_string()))?;

        let exp_addr = self
            .experience_address
            .parse::<Address>()
            .map_err(|e| Error::Klaytn(e.to_string()))?;
        let items_raw: (
            Vec<(String, String, U256, Vec<U256>)>,
            Vec<(String, String, U256, Vec<U256>)>,
        ) = self
            .contract
            .method("listDailyMissions", (exp_addr, nft_addr, U256::from(id)))
            .map_err(|e| Error::Klaytn(e.to_string()))?
            .call()
            .await
            .map_err(|e| Error::Klaytn(e.to_string()))?;

        let mut mission_ko: Vec<Mission> = vec![];
        let mut mission_en: Vec<Mission> = vec![];

        for item in items_raw.0 {
            mission_ko.push(Mission {
                mission: item.0,
                mission_description: item.1,
                exp: item.2,
                start_date: if item.3.len() == 2 {
                    item.3[0]
                } else {
                    U256::from_str("0").unwrap_or_default()
                },
                end_date: if item.3.len() == 2 {
                    item.3[0]
                } else {
                    U256::from_str("0").unwrap_or_default()
                },
            });
        }

        for item in items_raw.1 {
            mission_en.push(Mission {
                mission: item.0,
                mission_description: item.1,
                exp: item.2,
                start_date: if item.3.len() == 2 {
                    item.3[0]
                } else {
                    U256::from_str("0").unwrap_or_default()
                },
                end_date: if item.3.len() == 2 {
                    item.3[0]
                } else {
                    U256::from_str("0").unwrap_or_default()
                },
            });
        }

        Ok((mission_ko, mission_en))
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Default, PartialEq)]
pub struct Mission {
    pub mission: String,
    pub mission_description: String,
    pub exp: U256,
    pub start_date: U256,
    pub end_date: U256,
}
