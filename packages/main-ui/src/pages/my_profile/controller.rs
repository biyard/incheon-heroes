use by_macros::*;
use dioxus::prelude::*;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {}

impl Controller {
    pub fn new() -> std::result::Result<Self, RenderError> {
        let ctrl = Self {};

        Ok(ctrl)
    }
}
