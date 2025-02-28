#![allow(non_snake_case)]
use super::controller::*;
use super::i18n::*;
use super::models::*;
use crate::components::headings::Heading1;
use crate::services::account_contract::AccountActivity;
use crate::utils::time::formatted_timestamp;
use by_components::responsive::ResponsiveService;
use dioxus::prelude::*;
use dioxus_translate::*;
use ethers::types::U256;

#[component]
pub fn MyProfilePage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;

    let mission_histories = ctrl.get_mission_historys();
    tracing::debug!("mission histories: {:?}", mission_histories);

    let experience_histories = ctrl.get_experience_histories();
    tracing::debug!("experience histories: {:?}", experience_histories);

    let token_histories = ctrl.get_token_histories();
    tracing::debug!("token histories: {:?}", token_histories);

    let goods_info = ctrl.get_goods_info();
    tracing::debug!("goods histories: {:?}", goods_info);

    let responsive: ResponsiveService = use_context();

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

    let account_exp = ctrl
        .user_service
        .account_exp()
        .unwrap_or(U256::from(0))
        .as_u64();

    let account_activities = ctrl.user_service.account_activities().unwrap_or(vec![]);

    rsx! {
        by_components::meta::MetaPage { title: "{tr.title}" }

        div {
            id: "my-page",
            class: "w-full flex flex-col gap-[40px] items-center justify-start",
            style: if responsive.width() > 1200.0 { "padding: 20px;" } else { "" },
            Heading1 { lang, "{tr.title}" }

            div { class: "flex flex-col w-full gap-[20px] max-w-[1200px]",
                div {
                    class: "flex flex-col bg-white/60 rounded-[20px] w-full",
                    style: if responsive.width() > 1200.0 { "padding: 20px;" } else { "" },
                    div { class: if responsive.width() > 1200.0 { "w-full grid grid-cols-5 rounded-[10px] border-[1px] border-gray-200 gap-[10px] text-[#636363] text-[16px]" } else { "w-full flex flex-col rounded-[10px] border-[1px] border-gray-200 gap-[10px] text-[#636363] text-[16px]" },
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
                            div { class: "flex flex-row gap-[10px] justify-start items-center",
                                div { class: "font-medium text-[15px] text-[#636363]",
                                    "{account_exp} EXP"
                                }

                                if account_activities.len() != 0 {
                                    div {
                                        class: "cursor-pointer w-[26px] h-[26px]",
                                        onclick: move |_| {
                                            ctrl.open_claim_modal(lang, account_activities.clone());
                                        },
                                        img {
                                            src: asset!("/public/images/plus.png"),
                                            width: 26,
                                            height: 26,
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div {
                    class: "flex flex-row w-full items-center justify-between bg-white/60 rounded-[10px] text-[16px] font-semibold",
                    style: if responsive.width() > 1200.0 { "padding: 15px;" } else { "" },
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
        }
    } // end of this page
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
        div { class: "col-span-1 flex flex-row items-center justify-start py-[10px] px-[10px] {bottom_border}",
            "{title}"
        }
        div { class: "col-span-4 flex flex-row items-center justify-start px-[20px] {bottom_border} text-[14px]",
            {children}
        }
    }
}

#[component]
pub fn NftClaimModal(
    lang: Language,
    account_activities: Vec<AccountActivity>,
    ondistribute: EventHandler<MouseEvent>,
    oncancel: EventHandler<MouseEvent>,
) -> Element {
    let tr: NftClaimModalTranslate = translate(&lang);
    let mut total_exp = use_signal(|| U256::from(0));

    use_effect({
        let account_activities = account_activities.clone();
        move || {
            total_exp.set(
                account_activities
                    .iter()
                    .fold(U256::zero(), |acc, activity| acc + activity.exp),
            );
        }
    });

    rsx! {
        div { class: "flex flex-col w-[350px] md:w-[800px] max-w-[800px] h-[370px] justify-start items-center gap-[10px]",
            div { class: "font-semibold text-[15px] md:text-[20px] text-[#16775d]",
                "{tr.title}"
            }
            div { class: "flex flex-col md:w-full w-[330px] justify-center items-center bg-white border border-[#e0e0e0] rounded-[12px]",
                div { class: "flex flex-row w-full h-[40px] md:h-[50px] justify-center items-center border-b border-b-[#e0e0e0] font-semibold text-[10px] md:text-[15px] text-[#636363]",
                    div { class: "flex flex-1 w-full justify-start items-center px-[20px] ",
                        "{tr.participation_activities}"
                    }
                    div { class: "flex w-[110px] md:w-[200px] justify-start items-center px-[20px]",
                        "{tr.participation_date}"
                    }
                    div { class: "flex w-[110px] md:w-[200px] justify-start items-center px-[20px]",
                        "{tr.gained_experience}"
                    }
                }
                div { class: "flex flex-col w-full max-h-[150px] h-[150px] overflow-y-auto",
                    for (index , activity) in account_activities.iter().enumerate() {
                        div {
                            class: format!(
                                "flex flex-row w-full min-h-[40px] h-[40px] md:h-[50px] justify-center items-center {} font-medium text-[10px] md:text-[15px] text-[#636363]",
                                if index != account_activities.len() - 1 {
                                    "border-b border-b-[#e0e0e0]"
                                } else {
                                    ""
                                },
                            ),
                            div { class: "flex flex-1 w-full justify-start items-center px-[20px] ",
                                "{activity.name}"
                            }
                            div { class: "flex w-[110px] md:w-[200px] justify-start items-center px-[20px]",
                                {formatted_timestamp(activity.progress_date.as_u64() as i64)}
                            }
                            div { class: "flex w-[110px] md:w-[200px] justify-start items-center px-[20px]",
                                "{activity.exp.as_u64()} EXP"
                            }
                        }
                    }
                }
            }

            div { class: "flex flex-row w-[280px] justify-between items-center mt-[20px]",
                div { class: "font-semibold text-[10px] md:text-[15px] text-[#636363]",
                    "{tr.total_gained_experience}"
                }
                div { class: "font-medium text-[10px] md:text-[15px] text-[#636363]",
                    "{total_exp()} EXP"
                }
            }

            div { class: "flex flex-row w-full justify-center items-center gap-[10px] mt-[20px]",
                div {
                    class: "cursor-pointer flex flex-row w-[150px] h-[45px] justify-center items-center rounded-[10px] bg-white border border-[#e0e0e0] font-semibold text-[13px] md:text-[16px] text-[#16775d]",
                    onclick: move |e: Event<MouseData>| {
                        ondistribute.call(e);
                    },
                    "{tr.distribute}"
                }
                div {
                    class: "cursor-pointer flex flex-row w-[150px] h-[45px] justify-center items-center rounded-[10px] bg-white border border-[#e0e0e0] font-semibold text-[13px] md:text-[16px] text-[#636363]",
                    onclick: move |e: Event<MouseData>| {
                        oncancel.call(e);
                    },
                    "{tr.cancel}"
                }
            }
        }
    }
}
