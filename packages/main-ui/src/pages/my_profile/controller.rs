use by_macros::*;
use dioxus::prelude::*;
use dioxus_translate::Language;

use crate::{route::Route, services::user_service::UserService};

use super::models::ProfileTabs;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    pub lang: Language,
    pub selected_tab: Signal<ProfileTabs>,
    pub user_service: UserService,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let ctrl = Self {
            lang,
            selected_tab: use_signal(|| ProfileTabs::MissionHistory),
            user_service: use_context(),
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
}
