#![allow(non_snake_case)]
use crate::pages::ColGridCards;

use super::controller::*;
use super::i18n::*;
#[allow(unused_imports)]
use by_components::icons;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn ContentsByIdPage(id: i64, lang: Language) -> Element {
    let mut _ctrl = Controller::new(lang, id)?;
    let tr: ContentsByIdTranslate = translate(&lang);
    let (ref content, ref user) = _ctrl.rsc()?;

    rsx! {
        by_components::meta::MetaPage { title: "{tr.title}" }

        div { id: "contents-by-id", class: "flex flex-col gap-[40px]",
            p { "{content:?}" }
            p { "{user:?}" }

            div { class: "flex flex-row gap-[30px]",
                img {
                    src: "{content.thumbnail_image}",
                    alt: "{content.title}",
                    class: "w-full h-[300px] object-cover",
                }
            }

            p {
                {
                    format!(
                        "{} {} {}",
                        tr.more_contents_lead,
                        user.evm_address,
                        tr.more_contents_tail,
                    )
                }
            }
            ColGridCards { lang, contents: user.contents.clone() }
        } // end of this page
    }
}
