#![allow(non_snake_case)]
use crate::components::headings::Heading1;

use super::controller::*;
use super::i18n::*;
use super::models::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn MyProfilePage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: MyProfileTranslate = translate(&lang);
    let tab = match ctrl.selected_tab() {
        ProfileTabs::MissionHistory => rsx! {
            MissionHistory { lang }
        },
        ProfileTabs::ExperienceHistory => rsx! {
            ExperienceHistory { lang }
        },
        ProfileTabs::NftTransferHistory => rsx! {
            NftTransferHistory { lang }
        },
        ProfileTabs::GoodsPurchaseHistory => rsx! {
            GoodsPurchaseHistory { lang }
        },
    };

    rsx! {
        by_components::meta::MetaPage { title: "{tr.title}" }

        div {
            id: "my-page",
            class: "w-full flex flex-col gap-[40px] items-center justify-start p-[20px]",
            Heading1 { lang, "{tr.title}" }
            div { class: "flex flex-col w-full gap-[20px]  max-w-[1200px]",
                div { class: "flex flex-col bg-white/60 rounded-[20px] p-[20px] w-full",

                    div { class: "w-full grid grid-cols-5 rounded-[10px] border-[1px] border-gray-200 gap-[10px] text-[#636363] text-[16px]",
                        ProfileLine { title: "{tr.login}", "{ctrl.login_type()}" }
                        if let Some(ref address) = ctrl.user_service.evm_address() {
                            ProfileLine { title: "{tr.wallet}", "{address}" }
                        }

                        if let Some(ref address) = ctrl.user_service.icp_address() {
                            ProfileLine { title: "{tr.icp_wallet}", "{address}" }
                        }
                        ProfileLine {
                            title: "{tr.wallet_experience}",
                            bottom_border: false,
                            ""
                        }
                    }
                }
                div { class: "flex flex-row w-full items-center justify-between bg-white/60 rounded-[10px] p-[15px] text-[16px] font-semibold",
                    for tab in ProfileTabs::VARIANTS.iter() {
                        div {
                            class: "w-full flex flex-row items-center justify-center gap-[10px] p-[10px] cursor-pointer text-[16px] hover:text-[#4C9682]",
                            color: if ctrl.selected_tab() == *tab { "#16775D" } else { "#636363" },
                            onclick: move |_| ctrl.selected_tab.set(*tab),

                            "{tab.translate(&lang)}"
                        }
                    }
                }
                div { class: "w-full bg-white/60 rounded-[10px] p-[20px]", {tab} }
            }

        } // end of this page
    }
}

#[component]
pub fn MissionHistory(lang: Language) -> Element {
    rsx! { "Mission History" }
}

#[component]
pub fn ExperienceHistory(lang: Language) -> Element {
    rsx! { "ExperienceHistory" }
}

#[component]
pub fn NftTransferHistory(lang: Language) -> Element {
    rsx! { "NftTransferHistory" }
}

#[component]
pub fn GoodsPurchaseHistory(lang: Language) -> Element {
    rsx! { "GoodsPurchaseHistory" }
}

#[component]
pub fn ProfileLine(
    title: String,
    children: Element,
    #[props(default = true)] bottom_border: bool,
) -> Element {
    let bottom_border = if bottom_border {
        "border-b-[1px] border-gray-200"
    } else {
        ""
    };

    rsx! {
        div { class: "w-full col-span-1 flex flex-row items-center justify-start py-[10px] px-[20px] {bottom_border}",
            "{title}"
        }
        div { class: "w-full col-span-4 flex flex-row items-center justify-start py-[10px] px-[20px] {bottom_border} text-[14px]",
            {children}
        }
    }
}
