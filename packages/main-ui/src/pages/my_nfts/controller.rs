use dioxus::prelude::*;
use dioxus_popup::PopupService;
use dioxus_translate::Language;

use crate::{
    pages::NftSelectModal,
    route::Route,
    services::{klaytn::Klaytn, user_service::UserService},
};

#[derive(Clone, Copy)]
pub struct Controller {
    pub user_service: UserService,
    popup_service: Signal<PopupService>,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let popup_service: PopupService = use_context();

        let ctrl = Self {
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

    pub async fn distribute_exp(&self, id: u64, exp: u64) {
        let mut user: UserService = use_context();
        let klaytn: Klaytn = use_context();
        let account = klaytn.account.cloned();

        match account.distribute_account_exp(id as i64, exp as i64).await {
            Ok(_) => {
                tracing::debug!("success to distribute account exp");
                user.account_exp.restart();
            }
            Err(e) => {
                tracing::error!("distribute exp failed: {:?}", e);
            }
        }
    }

    pub fn open_nft_select_modal(&self, lang: Language, account_exp: u64, total_ids: Vec<u64>) {
        let mut popup_service = (self.popup_service)();
        let ctrl = self.clone();

        popup_service
            .open(rsx! {
                NftSelectModal {
                    lang,
                    total_nfts: total_ids,
                    account_exp,
                    oncancel: move |_| {
                        popup_service.close();
                    },
                    ondistibute: move |(id, exp): (u64, u64)| async move {
                        ctrl.distribute_exp(id, exp).await;
                        popup_service.close();
                    },
                }
            })
            .with_id("nft_select")
            .without_close();
    }
}
