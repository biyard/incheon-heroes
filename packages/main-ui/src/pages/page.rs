#![allow(non_snake_case)]
use std::str::FromStr;

use super::controller::*;
use crate::components::headings::Heading1;

use super::i18n::*;
use crate::assets::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn HomePage(lang: Language) -> Element {
    let tr: MainTextTranslate = translate(&lang);

    rsx! {
        div { id: "home-page", class: "flex flex-col items-center gap-[45px]",
            VideoSection {}

            Heading1 { lang, with_symbol: false, "INCHEON UNIVERSE" }
            p { class: "text-[16px] font-bold text-center", "{tr.main_text}" }
            LoginButton { lang }
            LeaderBoard { lang }
        }
    }
}

#[component]
pub fn LeaderBoard(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    lang: Language,
) -> Element {
    let tr: LeaderBoardTranslate = translate(&lang);
    let mut ctrl = LeaderBoardController::new()?;

    rsx! {
        div { class: "flex flex-col items-center gap-[30px]",
            Heading1 { lang, "{tr.title}" }

            div { class: "w-full flex flex-col items-end gap-[5px] px-[20px] py-[10px] rounded-[12px] bg-[#FAFAFA]/40",
                if let Some(ref data) = ctrl.leaderboard.value()() {
                    div { class: "text-[10px] font-semibold", "Last updated at: {data.updated_at()}" }

                    RankingBoards {
                        lang,
                        data: data.leaderboard.clone(),
                        onchange: move |t| { ctrl.selected_leaderboard_type.set(t) },
                    }
                }
            }
        }
    }
}

#[component]
pub fn RankingBoards(
    lang: Language,
    data: LeaderboardItems,
    onchange: EventHandler<LeaderboardType>,
) -> Element {
    tracing::debug!("RankingBoards: {:?}", data);

    rsx! {
        div { class: "w-full flex flex-col items-end gap-[5px] px-[20px] py-[10px] rounded-[12px]",
            select {
                class: "bg-[#FAFAFA]/60 my-[5px] text-center py-[10px] px-[10px] flex items-center justify-center rounded-[10px] text-[#636363] font-semibold",
                onchange: move |event| {
                    onchange(LeaderboardType::from_str(&event.value()).unwrap());
                },
                for option in LeaderboardType::variants(&lang).iter() {
                    option { class: "bg-white rounded-[10px]", value: "{option}", "{option}" }
                }
            }
            match &data {
                LeaderboardItems::Level(data) => rsx! {
                    LevelBoard { data: data.clone(), lang }
                },
                LeaderboardItems::Experience(data) => rsx! {
                    ExperienceBoard { data: data.clone(), lang }
                },
                LeaderboardItems::Daily(data) => rsx! {
                    DailyMissionBoard { data: data.clone(), lang }
                },
                LeaderboardItems::Voting(data) => rsx! {
                    VotingBoard { data: data.clone(), lang }
                },
            }
        }
    }
}

#[component]
pub fn LevelBoard(data: Vec<LeaderboardItemLevel>, lang: Language) -> Element {
    let tr: LevelBoardTranslate = translate(&lang);
    let grids = vec![
        "col-span-1",
        "col-span-2",
        "col-span-2",
        "col-span-2",
        "col-span-3",
    ];
    let headers = vec![tr.no, tr.nft_id, tr.level, tr.character, tr.address];

    rsx! {
        div { class: "w-full flex-col flex gap-[5px] text-[10px] font-semibold",
            div { class: "bg-white/50 rounded-[10px] grid grid-cols-10 h-[40px]",
                for (i , h) in headers.iter().enumerate() {
                    div { class: "{grids[i]} flex items-center justify-center py-auto text-[15px] font-semibold text-[#636363]",
                        "{h}"
                    }
                }
            }

            div { class: "rounded-[10px] border-[1px] border-[#E4E7E5] text-[#636363] text-[14px] font-medium",
                for (i , h) in data.iter().enumerate() {
                    div { class: if i < data.len() - 1 { "grid grid-cols-10 h-[40px] border-b-[1px]" } else { "grid grid-cols-10 h-[40px]" },
                        div { class: "{grids[0]} flex items-center justify-center py-auto",
                            Rank { i }
                        }
                        div { class: "{grids[1]} flex items-center justify-center py-auto",
                            "#{h.nft_num}"
                        }
                        div { class: "{grids[2]} flex items-center justify-center py-auto",
                            "{h.level}"
                        }
                        div { class: "{grids[3]} flex items-center justify-center py-auto",
                            "{h.character}"
                        }
                        div { class: "{grids[4]} flex items-center justify-center py-auto",
                            "{truncate_addr(&h.account_address)}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Rank(i: usize) -> Element {
    match i {
        0 => rsx! {
            img { src: GOLD_RANK }
        },
        1 => rsx! {
            img { src: SILVER_RANK }
        },
        2 => rsx! {
            img { src: BRONZE_RANK }
        },
        _ => rsx! { "{i}" },
    }
}

#[component]
pub fn ExperienceBoard(data: Vec<LeaderboardItemExperience>, lang: Language) -> Element {
    let tr: ExperienceBoardTranslate = translate(&lang);
    let grids = vec![
        "col-span-1",
        "col-span-2",
        "col-span-2",
        "col-span-2",
        "col-span-3",
    ];
    let headers = vec![tr.no, tr.nft_id, tr.exp, tr.character, tr.address];

    rsx! {
        div { class: "w-full flex-col flex gap-[5px] text-[10px] font-semibold",
            div { class: "bg-white/50 rounded-[10px] grid grid-cols-10 h-[40px]",
                for (i , h) in headers.iter().enumerate() {
                    div { class: "{grids[i]} flex items-center justify-center py-auto text-[15px] font-semibold text-[#636363]",
                        "{h}"
                    }
                }
            }

            div { class: "rounded-[10px] border-[1px] border-[#E4E7E5] text-[#636363] text-[14px] font-medium",
                for (i , h) in data.iter().enumerate() {
                    div { class: if i < data.len() - 1 { "grid grid-cols-10 h-[40px] border-b-[1px]" } else { "grid grid-cols-10 h-[40px]" },
                        div { class: "{grids[0]} flex items-center justify-center py-auto",
                            Rank { i }
                        }
                        div { class: "{grids[1]} flex items-center justify-center py-auto",
                            "#{h.nft_num}"
                        }
                        div { class: "{grids[2]} flex items-center justify-center py-auto",
                            "{h.experience}"
                        }
                        div { class: "{grids[3]} flex items-center justify-center py-auto",
                            "{h.character}"
                        }
                        div { class: "{grids[4]} flex items-center justify-center py-auto",
                            "{truncate_addr(&h.account_address)}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn DailyMissionBoard(data: Vec<LeaderboardItemDailyMission>, lang: Language) -> Element {
    let tr: MissionBoardTranslate = translate(&lang);
    let grids = vec!["col-span-1", "col-span-4", "col-span-5"];
    let headers = vec![tr.no, tr.missions, tr.address];

    rsx! {
        div { class: "w-full flex-col flex gap-[5px] text-[10px] font-semibold",
            div { class: "bg-white/50 rounded-[10px] grid grid-cols-10 h-[40px]",
                for (i , h) in headers.iter().enumerate() {
                    div { class: "{grids[i]} flex items-center justify-center py-auto text-[15px] font-semibold text-[#636363]",
                        "{h}"
                    }
                }
            }

            div { class: "rounded-[10px] border-[1px] border-[#E4E7E5] text-[#636363] text-[14px] font-medium",
                for (i , h) in data.iter().enumerate() {
                    div { class: if i < data.len() - 1 { "grid grid-cols-10 h-[40px] border-b-[1px]" } else { "grid grid-cols-10 h-[40px]" },
                        div { class: "{grids[0]} flex items-center justify-center py-auto",
                            Rank { i }
                        }
                        div { class: "{grids[1]} flex items-center justify-center py-auto",
                            "{h.daily_count}"
                        }
                        div { class: "{grids[2]} flex items-center justify-center py-auto",
                            "{truncate_addr(&h.account_address)}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn VotingBoard(data: Vec<LeaderboardItemVoting>, lang: Language) -> Element {
    let tr: VotingBoardTranslate = translate(&lang);
    let grids = vec!["col-span-1", "col-span-4", "col-span-5"];
    let headers = vec![tr.no, tr.votes, tr.address];

    rsx! {
        div { class: "w-full flex-col flex gap-[5px] text-[10px] font-semibold",
            div { class: "bg-white/50 rounded-[10px] grid grid-cols-10 h-[40px]",
                for (i , h) in headers.iter().enumerate() {
                    div { class: "{grids[i]} flex items-center justify-center py-auto text-[15px] font-semibold text-[#636363]",
                        "{h}"
                    }
                }
            }

            div { class: "rounded-[10px] border-[1px] border-[#E4E7E5] text-[#636363] text-[14px] font-medium",
                for (i , h) in data.iter().enumerate() {
                    div { class: if i < data.len() - 1 { "grid grid-cols-10 h-[40px] border-b-[1px]" } else { "grid grid-cols-10 h-[40px]" },
                        div { class: "{grids[0]} flex items-center justify-center py-auto",
                            Rank { i }
                        }
                        div { class: "{grids[1]} flex items-center justify-center py-auto",
                            "{h.voting_count}"
                        }
                        div { class: "{grids[2]} flex items-center justify-center py-auto",
                            "{truncate_addr(&h.account_address)}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn VideoSection() -> Element {
    rsx! {
        div { id: "videosection", class: "flex justify-center items-center p-4",
            video {
                class: "w-full max-w-6xl",
                autoplay: "true",
                r#loop: "true",
                muted: "true",
                src: "{VIDEO}",
            }
        }
    }
}

#[component]
pub fn LoginButton(
    #[props(default ="login_button".to_string())] id: String,
    #[props(default ="".to_string())] class: String,

    lang: Language,
) -> Element {
    let tr: LoginButtonTranslate = translate(&lang);

    rsx! {
        div { class: "flex justify-center items-center p-4",
            button {
                onclick: |_| println!("Button clicked!"),
                class: "px-4 py-2 bg-gray-500 text-white rounded-full hover:bg-gray-600 text-xl font-bold",
                style: "width: 250px; height: 60px; padding-10 px",
                "{tr.button_text}"
            }
        }
    }
}
