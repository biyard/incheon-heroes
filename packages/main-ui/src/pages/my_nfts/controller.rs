use by_macros::DioxusController;
use dioxus::prelude::*;
use dioxus_popup::PopupService;
use dioxus_translate::Language;

use crate::{
    pages::NftSelectModal,
    route::Route,
    services::{klaytn::Klaytn, user_service::UserService},
};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    pub user_service: UserService,
    pub account_exp: Resource<u64>,
    popup_service: Signal<PopupService>,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let popup_service: PopupService = use_context();
        let user_service: UserService = use_context();
        let klaytn: Klaytn = use_context();

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

        let ctrl = Self {
            user_service,
            account_exp,
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
        let klaytn: Klaytn = use_context();
        let account = klaytn.account.cloned();

        match account.distribute_account_exp(id as i64, exp as i64).await {
            Ok(_) => {
                btracing::debug!("success to distribute account exp");
            }
            Err(e) => {
                btracing::error!("distribute exp failed: {:?}", e);
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
