#![allow(non_snake_case)]
use super::controller::*;
use super::i18n::*;
use super::models::*;
use crate::components::headings::Heading1;
use crate::components::icons::LinkIcon;
use crate::components::icons::Mint;
use crate::components::icons::Transfer;
use crate::models::history::AccountTokenHistory;
use crate::models::history::ExperienceHistory;
use crate::models::history::MissionHistory;
use crate::services::account_contract::AccountActivity;
use crate::services::goods_contract::GoodsItem;
use crate::utils::address::parse_address;
use crate::utils::constant::SHIPPING_FORM_URL;
use crate::utils::constant::ZERO_ADDRESS;
use crate::utils::time::formatted_timestamp;
use by_components::responsive::ResponsiveService;
use dioxus::prelude::*;
use dioxus_translate::*;
use ethers::types::U256;

#[component]
pub fn MyProfilePage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;

    let mission_histories = ctrl.get_mission_historys();
    let experience_histories = ctrl.get_experience_histories();
    let token_histories = ctrl.get_token_histories();
    let goods_info = ctrl.get_goods_info();

    let klaytn_scope = ctrl.get_scope_endpoint();

    let responsive: ResponsiveService = use_context();

    let tr: MyProfileTranslate = translate(&lang);
    let tab = match ctrl.selected_tab() {
        ProfileTabs::MissionHistory => rsx! {
            MissionHistoryComponent { lang, histories: mission_histories }
        },
        ProfileTabs::ExperienceHistory => rsx! {
            ExperienceHistoryComponent { lang, histories: experience_histories }
        },
        ProfileTabs::NftTransferHistory => rsx! {
            NftTransferHistoryComponent { lang, histories: token_histories, klaytn_scope }
        },
        ProfileTabs::GoodsPurchaseHistory => rsx! {
            GoodsPurchaseHistoryComponent { lang, histories: goods_info }
        },
    };

    let account_exp = ctrl.account_exp()?;

    let account_activities = ctrl.account_activities()?;

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
                    div { class: if responsive.width() > 1200.0 { "w-full grid grid-cols-5 rounded-[10px] border-[1px] border-gray-200 text-[#636363] text-[16px]" } else { "w-full flex flex-col rounded-[10px] border-[1px] border-gray-200 gap-[10px] text-[#636363] text-[16px]" },
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
                div {
                    class: "w-full bg-white/60 rounded-[10px]",
                    style: if responsive.width() > 1200.0 { "padding: 20px;" } else { "" },
                    {tab}
                }
            }
        }
    } // end of this page
}

#[component]
pub fn MissionHistoryComponent(lang: Language, histories: Vec<MissionHistory>) -> Element {
    let tr: MissionHistoryComponentTranslate = translate(&lang);
    let ctrl = MissionHistoryController::new()?;

    rsx! {
        div { class: "flex flex-col justify-start items-start bg-transparent border border-[#e0e0e0] rounded-[12px] overflow-x-auto",
            div { class: "w-full min-w-[300px] table",
                // Table Header
                div { class: "table-header-group border-b border-b-[#e0e0e0]",
                    div { class: "table-row font-semibold text-[16px] text-[#636363]",
                        div { class: "table-cell px-[10px] py-[10px] min-w-[150px] border-b border-b-[#e0e0e0]",
                            "{tr.mission_name}"
                        }
                        div { class: "table-cell px-[10px] py-[10px] min-w-[150px] border-b border-b-[#e0e0e0]",
                            "{tr.progress_date}"
                        }
                        div { class: "table-cell px-[10px] py-[10px] min-w-[150px] border-b border-b-[#e0e0e0]",
                            "{tr.verification}"
                        }
                        div { class: "table-cell px-[10px] py-[10px] min-w-[150px] border-b border-b-[#e0e0e0]",
                            "{tr.gained_experience}"
                        }
                    }
                }

                // Table Rows
                div { class: "table-row-group",
                    for history in histories {
                        div { class: "table-row text-[16px] text-[#636363]",
                            div { class: "table-cell px-[10px] py-[10px] min-w-[150px] border-b border-b-[#e0e0e0] whitespace-pre-line",
                                {
                                    ctrl.mission_name(
                                            lang,
                                            history.mission_name.clone(),
                                            history.mission_name_en.clone(),
                                        )
                                        .unwrap_or_default()
                                }
                            }
                            div { class: "table-cell px-[10px] py-[10px] min-w-[150px] border-b border-b-[#e0e0e0]",
                                {
                                    formatted_timestamp(
                                        history.mission_start_date.parse::<i64>().unwrap_or_default(),
                                    )
                                }
                            }
                            div { class: "table-cell px-[10px] py-[10px] min-w-[150px] border-b border-b-[#e0e0e0]",
                                {
                                    if history.progress == "Inprogress" {
                                        tr.verification_in_progress
                                    } else if history.progress == "accepted" {
                                        tr.accepted
                                    } else {
                                        tr.rejected
                                    }
                                }
                            }
                            div { class: "table-cell px-[10px] py-[10px] min-w-[150px] border-b border-b-[#e0e0e0]",
                                {
                                    if history.experience <= 0 {
                                        "0 EXP".to_string()
                                    } else {
                                        format!("{} EXP", history.experience)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ExperienceHistoryComponent(lang: Language, histories: Vec<ExperienceHistory>) -> Element {
    let ctrl = ExperienceHistoryController::new()?;
    let tr: ExperienceHistoryComponentTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col justify-start items-start bg-transparent border border-[#e0e0e0] rounded-[12px] overflow-x-auto",
            div { class: "w-full min-w-[300px] table",
                // Table Header
                div { class: "table-header-group border-b border-b-[#e0e0e0]",
                    div { class: "table-row font-semibold text-[16px] text-[#636363]",
                        div { class: "table-cell px-[10px] py-[10px] min-w-[150px] border-b border-b-[#e0e0e0]",
                            "{tr.participation_event}"
                        }
                        div { class: "table-cell px-[10px] py-[10px] min-w-[150px] border-b border-b-[#e0e0e0]",
                            "{tr.acquired_nft}"
                        }
                        div { class: "table-cell px-[10px] py-[10px] min-w-[150px] border-b border-b-[#e0e0e0]",
                            "{tr.progress_date}"
                        }
                        div { class: "table-cell px-[10px] py-[10px] min-w-[150px] border-b border-b-[#e0e0e0]",
                            "{tr.gained_experience}"
                        }
                    }
                }

                // Table Rows
                div { class: "table-row-group",
                    for history in histories {
                        div { class: "table-row text-[16px] text-[#636363]",
                            div { class: "table-cell px-[10px] py-[10px] min-w-[150px] border-b border-b-[#e0e0e0] whitespace-pre-line",
                                {
                                    ctrl.event_name(lang, history.event_name, history.event_name_en)
                                        .unwrap_or_default()
                                }
                            }
                            div { class: "table-cell px-[10px] py-[10px] min-w-[150px] border-b border-b-[#e0e0e0]",
                                "#{history.token_id}"
                            }
                            div { class: "table-cell px-[10px] py-[10px] min-w-[150px] border-b border-b-[#e0e0e0]",
                                {
                                    formatted_timestamp(
                                        history.mission_reward_date.parse::<i64>().unwrap_or_default(),
                                    )
                                }
                            }
                            div { class: "table-cell px-[10px] py-[10px] min-w-[150px] border-b border-b-[#e0e0e0]",
                                "{history.experience} EXP"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn NftTransferHistoryComponent(
    lang: Language,
    histories: Vec<AccountTokenHistory>,
    klaytn_scope: String,
) -> Element {
    let navigator = use_navigator();

    rsx! {
        div { class: "flex flex-col justify-start items-start bg-transparent border border-[#e0e0e0] rounded-[12px] overflow-x-auto",
            div { class: "w-full min-w-[300px] table",
                // Table Header
                div { class: "table-header-group border-b border-b-[#e0e0e0]",
                    div { class: "table-row font-semibold text-[16px] text-[#636363]",
                        div { class: "table-cell px-[10px] py-[10px] min-w-[150px] border-b border-b-[#e0e0e0]",
                            "Event"
                        }
                        div { class: "table-cell px-[10px] py-[10px] min-w-[150px] border-b border-b-[#e0e0e0]",
                            "NFT"
                        }
                        div { class: "table-cell px-[10px] py-[10px] min-w-[150px] border-b border-b-[#e0e0e0]",
                            "From"
                        }
                        div { class: "table-cell px-[10px] py-[10px] min-w-[150px] border-b border-b-[#e0e0e0]",
                            "To"
                        }
                        div { class: "table-cell px-[10px] py-[10px] min-w-[150px] border-b border-b-[#e0e0e0]",
                            "Transaction"
                        }
                    }
                }

                // Table Rows
                div { class: "table-row-group",
                    for history in histories {
                        div { class: "table-row text-[16px] text-[#636363]",
                            div { class: "table-cell px-[10px] py-[10px] min-w-[150px] border-b border-b-[#e0e0e0] whitespace-pre-line",
                                {
                                    if history.from == ZERO_ADDRESS {
                                        rsx! {
                                            Mint {}
                                            div { "Mint" }
                                        }
                                    } else {
                                        rsx! {
                                            Transfer {}
                                            div { "Transfer" }
                                        }
                                    }
                                }
                            }
                            div { class: "table-cell px-[10px] py-[10px] min-w-[150px] border-b border-b-[#e0e0e0]",
                                "#{history.token_id}"
                            }
                            div { class: "table-cell px-[10px] py-[10px] min-w-[150px] border-b border-b-[#e0e0e0]",
                                {
                                    if history.from == ZERO_ADDRESS {
                                        "NullAddress".to_string()
                                    } else {
                                        parse_address(history.from.clone())
                                    }
                                }
                            }
                            div { class: "table-cell px-[10px] py-[10px] min-w-[150px] border-b border-b-[#e0e0e0]",
                                {
                                    if history.to == ZERO_ADDRESS {
                                        "NullAddress".to_string()
                                    } else {
                                        parse_address(history.to.clone())
                                    }
                                }
                            }
                            div { class: "table-cell px-[10px] py-[10px] min-w-[150px] border-b border-b-[#e0e0e0]",
                                div {
                                    class: "flex flex-row w-fit h-fit cursor-pointer",
                                    onclick: {
                                        let history = history.clone();
                                        let klaytn_scope_endpoint = klaytn_scope.clone();
                                        move |_| {
                                            navigator
                                                .push(format!("{}/{}", klaytn_scope_endpoint, history.transaction_hash));
                                        }
                                    },
                                    LinkIcon {}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn GoodsPurchaseHistoryComponent(lang: Language, histories: Vec<GoodsItem>) -> Element {
    let navigator = use_navigator();
    let ctrl = GoodsPurchaseHistoryController::new()?;
    let tr: GoodsPurchaseHistoryTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col justify-start items-start bg-transparent border border-[#e0e0e0] rounded-[12px] overflow-x-auto",
            div { class: "w-full min-w-[300px] table",
                // Table Header
                div { class: "table-header-group border-b border-b-[#e0e0e0]",
                    div { class: "table-row font-semibold text-[16px] text-[#636363]",
                        div { class: "table-cell px-[10px] py-[10px] min-w-[150px] border-b border-b-[#e0e0e0]",
                            "Event"
                        }
                        div { class: "table-cell px-[10px] py-[10px] min-w-[150px] border-b border-b-[#e0e0e0]",
                            "{tr.purchase_date}"
                        }
                        div { class: "table-cell px-[10px] py-[10px] min-w-[150px] border-b border-b-[#e0e0e0]",
                            "{tr.goods_name}"
                        }
                        div { class: "table-cell px-[10px] py-[10px] min-w-[150px] border-b border-b-[#e0e0e0]",
                            "{tr.directed_information}"
                        }
                    }
                }

                // Table Rows
                div { class: "table-row-group",
                    for goods in histories {
                        div { class: "table-row text-[16px] text-[#636363]",
                            div { class: "table-cell px-[10px] py-[10px] min-w-[150px] border-b border-b-[#e0e0e0] whitespace-pre-line gap-[10px]",
                                "{tr.purchase_product}"
                            }
                            div { class: "table-cell px-[10px] py-[10px] min-w-[150px] border-b border-b-[#e0e0e0]",
                                {formatted_timestamp(goods.buy_date.as_u64() as i64)}
                            }
                            div { class: "table-cell px-[10px] py-[10px] min-w-[150px] gap-[5px] border-b border-b-[#e0e0e0]",
                                div {
                                    {ctrl.goods_name(lang, goods.name_ko, goods.name_en).unwrap_or_default()}
                                }
                            }
                            div { class: "table-cell px-[10px] py-[10px] min-w-[150px] gap-[10px] border-b border-b-[#e0e0e0]",
                                div { "{tr.enter_shipping_address}" }
                                div {
                                    class: "flex flex-row w-fit h-fit cursor-pointer",
                                    onclick: {
                                        move |_| {
                                            let shipping_form_url = SHIPPING_FORM_URL;
                                            navigator.push(shipping_form_url);
                                        }
                                    },
                                    LinkIcon {}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
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
        div { class: "break-all col-span-4 flex flex-row items-center justify-start px-[20px] {bottom_border} text-[14px]",
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
