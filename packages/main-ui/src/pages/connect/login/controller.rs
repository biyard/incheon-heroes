#![allow(dead_code)]
use crate::services::backend_api::AccountHint;
use by_macros::*;
use dioxus::prelude::*;
use dioxus_translate::Language;
use tiny_keccak::{Hasher, Keccak};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    pub lang: Language,
    pub hint: Signal<AccountHint>,
    pub password: Signal<String>,
}

impl Controller {
    pub fn new(lang: Language, hint: AccountHint) -> std::result::Result<Self, RenderError> {
        let ctrl = Self {
            lang,
            hint: use_signal(move || hint),
            password: use_signal(|| "".to_string()),
        };

        Ok(ctrl)
    }

    pub async fn handle_login(&self) {
        self.create_seed(self.password());
    }

    pub fn create_seed(&self, password: String) -> String {
        let hint = self.hint().private_key_hint.clone();

        let hex_password = hex::encode(password.as_bytes());
        let input = format!("0x{}", hex_password);

        let first_hash = keccak256(input.as_bytes());
        let seed1 = format!("0x{}", hex::encode(first_hash));

        let hint_trimmed = hint.trim_start_matches("0x");

        let concat_input = format!("{}{}", seed1, hint_trimmed);

        let second_hash = keccak256(concat_input.as_bytes());
        let final_seed = format!("0x{}", hex::encode(second_hash));

        final_seed
    }

    pub async fn handle_kakao(&self) {
        // TODO: send a message to kakao
    }

    pub async fn handle_google(&self) {
        // TODO: save the google account
    }
}

fn keccak256(input: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];
    hasher.update(input);
    hasher.finalize(&mut output);
    output
}
