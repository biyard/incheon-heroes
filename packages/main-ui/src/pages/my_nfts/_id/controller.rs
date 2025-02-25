use dioxus::prelude::*;
use dioxus_translate::Language;

use crate::{
    models::{
        history::{MissionHistory, MissionHistorys, TokenHistory, TokenHistorys},
        nft_metadata::NftMetadata,
    },
    services::{klaytn::Klaytn, mission_contract::Mission, user_service::UserService},
};

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    metadata: Resource<NftMetadata>,
    level_info: Resource<(u64, u64, u64, u64)>,
    mission: Resource<(Vec<Mission>, Vec<Mission>)>,

    mission_histories: Resource<Vec<MissionHistory>>,
    token_histories: Resource<Vec<TokenHistory>>,
}

impl Controller {
    pub fn new(id: i64) -> std::result::Result<Self, RenderError> {
        let klaytn: Klaytn = use_context();
        let user_service: UserService = use_context();

        let metadata = use_server_future(move || async move {
            match NftMetadata::fetch(id).await {
                Ok(nft) => nft,
                Err(e) => {
                    tracing::error!("Failed to fetch nft({id}) metadata: {e:?}");
                    NftMetadata::default()
                }
            }
        })?;

        let mission = use_server_future(move || async move {
            match klaytn.mission().list_daily_missions(id).await {
                Ok(res) => res,
                Err(e) => {
                    tracing::error!("get mission failed: {:?}", e);
                    (vec![], vec![])
                }
            }
        })?;

        let level_info = use_server_future(move || async move {
            match klaytn.experience().get_nft_exp(id).await {
                Ok(res) => {
                    tracing::debug!("{:?}", res);
                    let level = res.0.as_u64();
                    let elevel = res.1.as_u64();
                    let exp = res.2.as_u64();

                    match klaytn
                        .experience()
                        .get_max_exp(elevel as i64, level as i64, id)
                        .await
                    {
                        Ok(max_exp) => (level, elevel, exp, max_exp.as_u64()),
                        Err(e) => {
                            tracing::error!("get max exp failed: {:?}", e);
                            (0, 0, 0, 0)
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("get nft exp failed: {:?}", e);
                    (0, 0, 0, 0)
                }
            }
        })?;

        let mission_histories = use_server_future(move || {
            let id = id.clone();
            let account = user_service.evm_address().unwrap_or_default();
            async move {
                match MissionHistorys::fetch(account, id).await {
                    Ok(res) => res.mission_infos,
                    Err(e) => {
                        tracing::error!("Failed to get mission histories: {:?}", e);
                        vec![]
                    }
                }
            }
        })?;

        let token_histories = use_server_future(move || {
            let account = user_service.evm_address().unwrap_or_default();
            async move {
                match TokenHistorys::fetch(account).await {
                    Ok(res) => res.items,
                    Err(e) => {
                        tracing::error!("Failed to get token histories: {:?}", e);
                        vec![]
                    }
                }
            }
        })?;

        let ctrl = Self {
            metadata,
            level_info,
            mission,

            mission_histories,
            token_histories,
        };

        Ok(ctrl)
    }

    pub fn get_mission_historys(&self) -> Vec<MissionHistory> {
        match self.mission_histories.value()() {
            Some(v) => v,
            None => vec![],
        }
    }

    pub fn get_token_historys(&self) -> Vec<TokenHistory> {
        match self.token_histories.value()() {
            Some(v) => v,
            None => vec![],
        }
    }

    pub fn get_mission(&self, lang: Language) -> Vec<Mission> {
        match self.mission.value()() {
            Some(v) => match lang {
                Language::En => v.1,
                _ => v.0,
            },
            None => vec![],
        }
    }

    pub fn get_level_info(&self) -> (u64, u64, u64, u64) {
        match self.level_info.value()() {
            Some(v) => v,
            None => (0, 0, 0, 0),
        }
    }

    pub fn get_metadata(&self) -> NftMetadata {
        match self.metadata.value()() {
            Some(v) => v,
            None => NftMetadata::default(),
        }
    }
}
