use by_macros::*;
use dioxus::prelude::*;
use dioxus_popup::PopupService;
use dioxus_translate::Language;

use crate::{
    pages::NftClaimModal,
    route::Route,
    services::{account_contract::AccountActivity, klaytn::Klaytn, user_service::UserService},
};

use super::models::ProfileTabs;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    pub lang: Language,
    pub selected_tab: Signal<ProfileTabs>,
    pub user_service: UserService,
    pub popup_service: Signal<PopupService>,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let popup_service: PopupService = use_context();

        let ctrl = Self {
            lang,
            selected_tab: use_signal(|| ProfileTabs::MissionHistory),
            user_service: use_context(),
            popup_service: use_signal(|| popup_service),
        };

        let nav = use_navigator();
        use_effect(move || {
            if !ctrl.user_service.is_logined() {
                nav.push(Route::ConnectPage { lang });
            }
        });

        Ok(ctrl)
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
