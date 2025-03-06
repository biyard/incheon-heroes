use by_macros::*;
use dioxus::prelude::*;
use dioxus_popup::PopupService;
use dioxus_translate::Language;

use crate::{
    config,
    models::history::{
        AccountExperienceHistorys, AccountMissionHistorys, AccountTokenHistory,
        AccountTokenHistorys, ExperienceHistory, MissionHistory,
    },
    pages::NftClaimModal,
    route::Route,
    services::{
        account_contract::AccountActivity, goods_contract::GoodsItem, klaytn::Klaytn,
        user_service::UserService,
    },
};

use super::models::ProfileTabs;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    pub lang: Language,
    pub selected_tab: Signal<ProfileTabs>,
    pub user_service: UserService,
    pub popup_service: Signal<PopupService>,

    pub mission_histories: Resource<Vec<MissionHistory>>,
    pub experience_histories: Resource<Vec<ExperienceHistory>>,
    pub token_histories: Resource<Vec<AccountTokenHistory>>,
    pub goods_info: Resource<Vec<GoodsItem>>,

    pub account_exp: Resource<u64>,
    pub account_activities: Resource<Vec<AccountActivity>>,

    klaytn_scope_endpoint: Signal<String>,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let popup_service: PopupService = use_context();
        let mut user_service: UserService = use_context();

        let klaytn: Klaytn = use_context();

        let klaytn_scope_endpoint = config::get().klaytn_scope_endpoint;

        let mission_histories = use_server_future(move || {
            let account = user_service.evm_address().unwrap_or_default();
            async move {
                match AccountMissionHistorys::fetch(account).await {
                    Ok(res) => res.mission_infos,
                    Err(e) => {
                        tracing::error!("Failed to get mission histories: {:?}", e);
                        vec![]
                    }
                }
            }
        })?;

        let account_exp = use_resource(move || async move {
            let w = user_service.wallet();
            let address = match w.evm_address() {
                Some(address) => address,
                None => return 0,
            };

            match (klaytn.account)().get_account_exp(address).await {
                Ok(exp) => exp.as_u64(),
                Err(e) => {
                    tracing::error!("Failed to get exp: {e:?}");
                    0
                }
            }
        });

        let account_activities = use_resource(move || async move {
            let w = user_service.wallet();
            let address = match w.evm_address() {
                Some(address) => address,
                None => return vec![],
            };

            match (klaytn.account)().get_account_activities(&address).await {
                Ok(activities) => activities,
                Err(e) => {
                    tracing::error!("Failed to get activities: {e:?}");
                    vec![]
                }
            }
        });

        let experience_histories = use_server_future(move || {
            let account = user_service.evm_address().unwrap_or_default();
            async move {
                match AccountExperienceHistorys::fetch(account).await {
                    Ok(res) => res.experience_infos,
                    Err(e) => {
                        tracing::error!("Failed to get experience histories: {:?}", e);
                        vec![]
                    }
                }
            }
        })?;

        let token_histories = use_server_future(move || {
            let account = user_service.evm_address().unwrap_or_default();
            async move {
                match AccountTokenHistorys::fetch(account).await {
                    Ok(res) => res.items,
                    Err(e) => {
                        tracing::error!("Failed to get token histories: {:?}", e);
                        vec![]
                    }
                }
            }
        })?;

        let goods_info = use_server_future(move || {
            let account = user_service.evm_address().unwrap_or_default();
            let goods = (klaytn.goods)();
            async move {
                match goods.list_user_items(account).await {
                    Ok(res) => res,
                    Err(e) => {
                        tracing::error!("Failed to get goods data: {:?}", e);
                        vec![]
                    }
                }
            }
        })?;

        let ctrl = Self {
            lang,
            selected_tab: use_signal(|| ProfileTabs::MissionHistory),
            user_service: use_context(),
            popup_service: use_signal(|| popup_service),
            mission_histories,
            experience_histories,
            account_exp,
            account_activities,
            token_histories,
            goods_info,

            klaytn_scope_endpoint: use_signal(|| klaytn_scope_endpoint.to_string()),
        };

        let nav = use_navigator();
        use_effect(move || {
            if !ctrl.user_service.is_logined() {
                nav.push(Route::ConnectPage { lang });
            }
        });

        spawn(async move {
            user_service.listen_for_account_changes().await;
        });

        Ok(ctrl)
    }

    pub fn get_goods_info(&self) -> Vec<GoodsItem> {
        match self.goods_info.value()() {
            Some(v) => v,
            None => vec![],
        }
    }

    pub fn get_mission_historys(&self) -> Vec<MissionHistory> {
        match self.mission_histories.value()() {
            Some(v) => v,
            None => vec![],
        }
    }

    pub fn get_scope_endpoint(&self) -> String {
        (self.klaytn_scope_endpoint)()
    }

    pub fn get_experience_histories(&self) -> Vec<ExperienceHistory> {
        match self.experience_histories.value()() {
            Some(v) => v,
            None => vec![],
        }
    }

    pub fn get_token_histories(&self) -> Vec<AccountTokenHistory> {
        match self.token_histories.value()() {
            Some(v) => v,
            None => vec![],
        }
    }

    pub fn login_type(&self) -> &'static str {
        self.user_service.wallet().translate(&self.lang)
    }

    pub async fn claim_exp(&mut self) {
        let klaytn: Klaytn = use_context();
        let account = klaytn.account.cloned();

        match account.claim_account_exp().await {
            Ok(_) => {
                btracing::debug!("success to claim account exp");
                self.account_exp.restart();
                self.account_activities.restart();
            }
            Err(e) => {
                btracing::error!("claim exp failed: {:?}", e);
            }
        }
    }

    pub fn open_claim_modal(&self, lang: Language, account_activities: Vec<AccountActivity>) {
        let mut popup_service = (self.popup_service)();
        let mut ctrl = self.clone();

        popup_service
            .open(rsx! {
                NftClaimModal {
                    lang,
                    account_activities,
                    ondistribute: move |_| async move {
                        ctrl.claim_exp().await;
                        popup_service.close();
                    },
                    oncancel: move |_| {
                        popup_service.close();
                    },
                }
            })
            .with_id("claim")
            .without_close();
    }
}

#[derive(Clone, Copy, DioxusController)]
pub struct MissionHistoryController {}

impl MissionHistoryController {
    pub fn new() -> std::result::Result<Self, RenderError> {
        let ctrl = Self {};

        Ok(ctrl)
    }

    pub fn mission_name(
        &self,
        lang: Language,
        mission_name_ko: String,
        mission_name_en: String,
    ) -> Result<String, RenderError> {
        match lang {
            Language::Ko => Ok(mission_name_ko),
            Language::En => Ok(mission_name_en),
        }
    }
}

#[derive(Clone, Copy, DioxusController)]
pub struct GoodsPurchaseHistoryController {}

impl GoodsPurchaseHistoryController {
    pub fn new() -> std::result::Result<Self, RenderError> {
        let ctrl = Self {};

        Ok(ctrl)
    }

    pub fn goods_name(
        &self,
        lang: Language,
        goods_name_ko: String,
        goods_name_en: String,
    ) -> Result<String, RenderError> {
        match lang {
            Language::Ko => Ok(goods_name_ko),
            Language::En => Ok(goods_name_en),
        }
    }
}

#[derive(Clone, Copy, DioxusController)]
pub struct ExperienceHistoryController {}

impl ExperienceHistoryController {
    pub fn new() -> std::result::Result<Self, RenderError> {
        let ctrl = Self {};

        Ok(ctrl)
    }

    pub fn event_name(
        &self,
        lang: Language,
        event_name_ko: String,
        event_name_en: String,
    ) -> Result<String, RenderError> {
        match lang {
            Language::Ko => Ok(event_name_ko),
            Language::En => Ok(event_name_en),
        }
    }
}
