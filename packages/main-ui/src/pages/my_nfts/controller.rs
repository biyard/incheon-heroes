use dioxus::prelude::*;

use crate::services::user_service::UserService;

#[derive(Clone, Copy)]
pub struct Controller {
    pub user_service: UserService,
}

impl Controller {
    pub fn new() -> std::result::Result<Self, RenderError> {
        let ctrl = Self {
            user_service: use_context(),
        };

        Ok(ctrl)
    }
}
