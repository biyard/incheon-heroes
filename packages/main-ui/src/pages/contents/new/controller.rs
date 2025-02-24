use by_macros::*;
use dioxus::prelude::*;
use dioxus_translate::Language;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let ctrl = Self { lang };

        Ok(ctrl)
    }
}
