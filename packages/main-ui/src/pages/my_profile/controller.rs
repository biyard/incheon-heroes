use by_macros::*;
use dioxus::prelude::*;
use dioxus_popup::PopupService;
use dioxus_translate::Language;

use crate::{
    models::history::{
        AccountExperienceHistorys, AccountMissionHistorys, AccountTokenHistorys, ExperienceHistory,
        MissionHistory, TokenHistory,
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
    pub token_histories: Resource<Vec<TokenHistory>>,
    pub goods_info: Resource<Vec<GoodsItem>>,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let popup_service: PopupService = use_context();
        let user_service: UserService = use_context();
        let klaytn: Klaytn = use_context();

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
            token_histories,
            goods_info,
        };

        let nav = use_navigator();
        use_effect(move || {
            if !ctrl.user_service.is_logined() {
                nav.push(Route::ConnectPage { lang });
            }
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

    pub fn get_experience_histories(&self) -> Vec<ExperienceHistory> {
        match self.experience_histories.value()() {
            Some(v) => v,
            None => vec![],
        }
    }

    pub fn get_token_histories(&self) -> Vec<TokenHistory> {
        match self.token_histories.value()() {
            Some(v) => v,
            None => vec![],
        }
    }

    pub fn login_type(&self) -> &'static str {
        self.user_service.wallet().translate(&self.lang)
    }

    pub async fn claim_exp(&self) {
        let mut user: UserService = use_context();
        let klaytn: Klaytn = use_context();
        let account = klaytn.account.cloned();

        match account.claim_account_exp().await {
            Ok(_) => {
                tracing::debug!("success to claim account exp");
                user.account_exp.restart();
                user.account_activities.restart();
            }
            Err(e) => {
                tracing::error!("claim exp failed: {:?}", e);
            }
        }
    }

    pub fn open_claim_modal(&self, lang: Language, account_activities: Vec<AccountActivity>) {
        let mut popup_service = (self.popup_service)();
        let ctrl = self.clone();

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
