use dioxus::prelude::*;
use dioxus_popup::PopupService;
use dioxus_translate::Language;

use crate::{
    config,
    models::{
        history::{MissionHistory, MissionHistorys, TokenHistory, TokenHistorys},
        nft_metadata::NftMetadata,
        post_result::{DailyMission, PostResult},
    },
    pages::my_nfts::_id::{exchange_popup::ExchangePopup, send_popup::SendPopup},
    route::Route,
    services::{klaytn::Klaytn, mission_contract::Mission, user_service::UserService},
};

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    metadata: Resource<NftMetadata>,
    level_info: Resource<(u64, u64, u64, u64)>,
    mission: Resource<(Vec<Mission>, Vec<Mission>)>,

    mission_histories: Resource<Vec<MissionHistory>>,
    token_histories: Resource<Vec<TokenHistory>>,

    klaytn_scope_endpoint: Signal<String>,
    popup_service: Signal<PopupService>,

    token_id: Signal<i64>,
    progress_daily_missions: Signal<Vec<String>>,
}

impl Controller {
    pub fn new(id: i64) -> std::result::Result<Self, RenderError> {
        let popup_service: PopupService = use_context();

        let klaytn_scope_endpoint = config::get().klaytn_scope_endpoint;
        let contract_address = config::get().contracts.nft;
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
            match (klaytn.mission)().list_daily_missions(id).await {
                Ok(res) => res,
                Err(e) => {
                    tracing::error!("get mission failed: {:?}", e);
                    (vec![], vec![])
                }
            }
        })?;

        let level_info = use_server_future(move || async move {
            match (klaytn.experience)().get_nft_exp(id).await {
                Ok(res) => {
                    tracing::debug!("{:?}", res);
                    let level = res.0.as_u64();
                    let elevel = res.1.as_u64();
                    let exp = res.2.as_u64();

                    match (klaytn.experience)()
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

        let token_histories = use_server_future(move || async move {
            match TokenHistorys::fetch(contract_address.to_string(), id).await {
                Ok(res) => res.items,
                Err(e) => {
                    tracing::error!("Failed to get token histories: {:?}", e);
                    vec![]
                }
            }
        })?;

        let mut ctrl = Self {
            metadata,
            level_info,
            mission,

            mission_histories,
            token_histories,

            klaytn_scope_endpoint: use_signal(|| klaytn_scope_endpoint.to_string()),
            popup_service: use_signal(|| popup_service),

            token_id: use_signal(|| id),
            progress_daily_missions: use_signal(|| vec![]),
        };

        use_effect(move || {
            let historys = ctrl.get_mission_historys();

            let titles: Vec<String> = historys.iter().map(|v| v.mission_name.clone()).collect();
            ctrl.progress_daily_missions.set(titles);
        });

        Ok(ctrl)
    }

    pub async fn send_discord_channel(&self, lang: Language, index: i64, uri: String) {
        let user: UserService = use_context();
        let mission_ko = self
            .get_mission(Language::Ko)
            .get(index as usize)
            .unwrap()
            .clone();
        let mission_en = self
            .get_mission(Language::En)
            .get(index as usize)
            .unwrap()
            .clone();
        let token_id = (self.token_id)();
        let evm_address = user.evm_address().unwrap_or_default();

        let req = DailyMission {
            mission_title: mission_ko.mission,
            mission_title_en: mission_en.mission,
            mission_description: mission_ko.mission_description,
            mission_description_en: mission_en.mission_description,
            lang: match lang {
                Language::Ko => "ko_KR".to_string(),
                Language::En => "en_US".to_string(),
            },
            mission_experience: mission_ko.exp.as_u32() as i32,
            token_id: token_id as i32,
            image_url: uri,
            account: evm_address.clone(),
            version: 2,
        };

        tracing::debug!("send discord request: {:?} {:?}", req, index);
        let mut ctrl = self.clone();

        match PostResult::send_discord_channel(req).await {
            Ok(_) => {
                tracing::debug!("success to send discord message");
                ctrl.mission_histories.restart();
            }
            Err(e) => {
                tracing::debug!("failed to send discord channel: {:?}", e);
            }
        }
    }

    pub async fn send_nft(&self, lang: Language, to: String) {
        let mut user: UserService = use_context();
        let klaytn: Klaytn = use_context();
        let nft = klaytn.nft.cloned();
        let nav = use_navigator();

        let from = user.evm_address().unwrap_or_default();
        let token_id = (self.token_id)();

        match nft.safe_transfer_from(&from, &to, token_id).await {
            Ok(_) => {
                user.evm_nfts.restart();
                user.sbts.restart();
                user.icp_nfts.restart();

                nav.push(Route::MyNftsPage { lang });
            }
            Err(e) => {
                tracing::error!("send failed: {:?}", e);
            }
        }
    }

    pub fn get_progress_daily_missions(&self) -> Vec<String> {
        (self.progress_daily_missions)()
    }

    pub fn get_scope_endpoint(&self) -> String {
        (self.klaytn_scope_endpoint)()
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

    pub fn get_mission_ko(&self) -> Vec<Mission> {
        match self.mission.value()() {
            Some(v) => v.0,
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

    pub fn open_send_modal(&self, lang: Language) {
        let mut popup_service = (self.popup_service)();
        let ctrl = self.clone();

        popup_service
            .open(rsx! {
                SendPopup {
                    lang,
                    onsend: move |to: String| async move {
                        ctrl.send_nft(lang, to).await;
                        popup_service.close();
                    },
                    oncancel: move |_| {
                        popup_service.close();
                    },
                }
            })
            .with_id("send")
            .without_close();
    }

    pub fn open_swap_modal(&self, lang: Language) {
        let mut popup_service = (self.popup_service)();

        popup_service
            .open(rsx! {
                ExchangePopup {
                    lang,
                    onexchange: move |_| {
                        tracing::debug!("call exchange function");
                    },
                    oncancel: move |_| {
                        popup_service.close();
                    },
                }
            })
            .with_id("swap")
            .without_close();
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HistoryController {}

impl HistoryController {
    pub fn new() -> std::result::Result<Self, RenderError> {
        let ctrl = Self {};
        Ok(ctrl)
    }

    pub fn translate_mission_title(
        &self,
        lang: Language,
        mission_ko: String,
        mission_en: String,
    ) -> String {
        match lang {
            Language::Ko => mission_ko,
            Language::En => mission_en,
        }
    }
}
