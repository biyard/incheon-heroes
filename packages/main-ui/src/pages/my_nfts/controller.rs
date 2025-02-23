use dioxus::prelude::*;
use dioxus_translate::Language;

use crate::{route::Route, services::user_service::UserService};

#[derive(Clone, Copy)]
pub struct Controller {
    pub user_service: UserService,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let ctrl = Self {
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
}
