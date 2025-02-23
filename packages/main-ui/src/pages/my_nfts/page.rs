#![allow(non_snake_case)]
use crate::components::headings::Heading1;

use super::controller::*;
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn MyNftsPage(lang: Language) -> Element {
    let ctrl = Controller::new()?;
    let tr: MyNftsTranslate = translate(&lang);

    rsx! {
        div { id: "my-nfts", class: "flex flex-col gap-[80px]",
            Heading1 { lang, "{tr.title}" }
            div {
                for sbt in ctrl.user_service.sbts()? {
                    "{sbt:?}"
                }

                for nft in ctrl.user_service.evm_nfts()? {
                    "{nft:?}"
                }
            }
        }
    }
}
