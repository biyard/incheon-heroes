use dioxus::prelude::*;

use crate::pages::controller::*;
use by_macros::DioxusController;

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    pub s1_data: Signal<LeaderboardItems>,
    pub s2_data: Signal<LeaderboardItems>,
}

impl Controller {
    pub fn new() -> std::result::Result<Self, RenderError> {
        let ctrl = Self {
            s1_data: use_signal(|| LeaderboardItems::Level(Self::s1_level_data())),
            s2_data: use_signal(|| LeaderboardItems::Level(Self::s2_level_data())),
        };

        Ok(ctrl)
    }

    pub fn handle_select_s1_type(&mut self, t: LeaderboardType) {
        tracing::info!("handle_select_s1_type: {:?}", t);
        self.s1_data.set(match t {
            LeaderboardType::Level => LeaderboardItems::Level(Self::s1_level_data()),
            LeaderboardType::Experience => LeaderboardItems::Experience(Self::s1_exp_data()),
            LeaderboardType::Daily => LeaderboardItems::Daily(Self::s1_daily_mission_data()),
            LeaderboardType::Voting => LeaderboardItems::Voting(Self::s1_voting_data()),
        });
    }

    pub fn handle_select_s2_type(&mut self, t: LeaderboardType) {
        tracing::info!("handle_select_s2_type: {:?}", t);
        self.s2_data.set(match t {
            LeaderboardType::Level => LeaderboardItems::Level(Self::s2_level_data()),
            LeaderboardType::Experience => LeaderboardItems::Experience(Self::s2_exp_data()),
            LeaderboardType::Daily => LeaderboardItems::Daily(Self::s2_daily_mission_data()),
            LeaderboardType::Voting => LeaderboardItems::Voting(Self::s2_voting_data()),
        });
    }

    pub fn s1_voting_data() -> Vec<LeaderboardItemVoting> {
        vec![
            LeaderboardItemVoting {
                account_address: "0x9C...126C".to_string(),
                voting_count: 710,
                rank: 1,
                version: 1,
            },
            LeaderboardItemVoting {
                account_address: "0x27...35dd".to_string(),
                voting_count: 625,
                rank: 2,
                version: 1,
            },
            LeaderboardItemVoting {
                account_address: "0xD1...9D20".to_string(),
                voting_count: 520,
                rank: 3,
                version: 1,
            },
            LeaderboardItemVoting {
                account_address: "0x0e...5851".to_string(),
                voting_count: 414,
                rank: 4,
                version: 1,
            },
            LeaderboardItemVoting {
                account_address: "0xf9...4C79".to_string(),
                voting_count: 378,
                rank: 5,
                version: 1,
            },
            LeaderboardItemVoting {
                account_address: "0xED...4040".to_string(),
                voting_count: 319,
                rank: 6,
                version: 1,
            },
            LeaderboardItemVoting {
                account_address: "0xA6...9Dae".to_string(),
                voting_count: 314,
                rank: 7,
                version: 1,
            },
            LeaderboardItemVoting {
                account_address: "0x35...4fe6".to_string(),
                voting_count: 300,
                rank: 8,
                version: 1,
            },
            LeaderboardItemVoting {
                account_address: "0x73...38e0".to_string(),
                voting_count: 274,
                rank: 9,
                version: 1,
            },
            LeaderboardItemVoting {
                account_address: "0xec...2fA7".to_string(),
                voting_count: 258,
                rank: 10,
                version: 1,
            },
        ]
    }

    pub fn s1_daily_mission_data() -> Vec<LeaderboardItemDailyMission> {
        vec![
            LeaderboardItemDailyMission {
                account_address: "0x35...4fe6".to_string(),
                daily_count: 60,
                rank: 1,
                version: 1,
            },
            LeaderboardItemDailyMission {
                account_address: "0xa6...9dae".to_string(),
                daily_count: 60,
                rank: 2,
                version: 1,
            },
            LeaderboardItemDailyMission {
                account_address: "0x0e...5851".to_string(),
                daily_count: 54,
                rank: 3,
                version: 1,
            },
            LeaderboardItemDailyMission {
                account_address: "0xf9...4c79".to_string(),
                daily_count: 49,
                rank: 4,
                version: 1,
            },
            LeaderboardItemDailyMission {
                account_address: "0xa6...5f98".to_string(),
                daily_count: 43,
                rank: 5,
                version: 1,
            },
            LeaderboardItemDailyMission {
                account_address: "0x3d...65dc".to_string(),
                daily_count: 39,
                rank: 6,
                version: 1,
            },
            LeaderboardItemDailyMission {
                account_address: "0x4d...f90f".to_string(),
                daily_count: 38,
                rank: 7,
                version: 1,
            },
            LeaderboardItemDailyMission {
                account_address: "0xe7...1778".to_string(),
                daily_count: 38,
                rank: 8,
                version: 1,
            },
            LeaderboardItemDailyMission {
                account_address: "0xef...53a5".to_string(),
                daily_count: 36,
                rank: 9,
                version: 1,
            },
            LeaderboardItemDailyMission {
                account_address: "0xdb...0217".to_string(),
                daily_count: 35,
                rank: 10,
                version: 1,
            },
        ]
    }

    pub fn s1_exp_data() -> Vec<LeaderboardItemExperience> {
        vec![
            LeaderboardItemExperience {
                rank: 1,
                experience: 3590,
                nft_num: 301,
                account_address: "0x0e...5851".to_string(),
                version: 1,
                character: "Ainy".to_string(),
            },
            LeaderboardItemExperience {
                rank: 2,
                experience: 3590,
                nft_num: 54,
                account_address: "0xA6...9Dae".to_string(),
                version: 1,
                character: "Comy".to_string(),
            },
            LeaderboardItemExperience {
                rank: 3,
                experience: 3590,
                nft_num: 107,
                account_address: "0x35...4fe6".to_string(),
                version: 1,
                character: "Comy".to_string(),
            },
            LeaderboardItemExperience {
                rank: 4,
                experience: 3590,
                nft_num: 1903,
                account_address: "0xc9...55Ac".to_string(),
                version: 1,
                character: "Comy".to_string(),
            },
            LeaderboardItemExperience {
                rank: 5,
                experience: 3590,
                nft_num: 538,
                account_address: "0x9C...126C".to_string(),
                version: 1,
                character: "Ainy".to_string(),
            },
            LeaderboardItemExperience {
                rank: 6,
                experience: 3590,
                nft_num: 1906,
                account_address: "0xA6...5F98".to_string(),
                version: 1,
                character: "Comy".to_string(),
            },
            LeaderboardItemExperience {
                rank: 7,
                experience: 3590,
                nft_num: 472,
                account_address: "0x4d...f90f".to_string(),
                version: 1,
                character: "Bumy".to_string(),
            },
            LeaderboardItemExperience {
                rank: 8,
                experience: 3590,
                nft_num: 383,
                account_address: "0xeF...53a5".to_string(),
                version: 1,
                character: "Ainy".to_string(),
            },
            LeaderboardItemExperience {
                rank: 9,
                experience: 3590,
                nft_num: 241,
                account_address: "0xDB...0217".to_string(),
                version: 1,
                character: "Comy".to_string(),
            },
            LeaderboardItemExperience {
                rank: 10,
                experience: 3590,
                nft_num: 685,
                account_address: "0xf9...4C79".to_string(),
                version: 1,
                character: "Ainy".to_string(),
            },
        ]
    }

    pub fn s1_level_data() -> Vec<LeaderboardItemLevel> {
        vec![
            LeaderboardItemLevel {
                rank: 1,
                level: 3,
                nft_num: 54,
                account_address: "0xA6...9Dae".to_string(),
                version: 1,
                character: "Comy".to_string(),
            },
            LeaderboardItemLevel {
                rank: 2,
                level: 3,
                nft_num: 107,
                account_address: "0x35...4fe6".to_string(),
                version: 1,
                character: "Comy".to_string(),
            },
            LeaderboardItemLevel {
                rank: 3,
                level: 3,
                nft_num: 301,
                account_address: "0x0e...5851".to_string(),
                version: 1,
                character: "Ainy".to_string(),
            },
            LeaderboardItemLevel {
                rank: 4,
                level: 3,
                nft_num: 685,
                account_address: "0xf9...4C79".to_string(),
                version: 1,
                character: "Ainy".to_string(),
            },
            LeaderboardItemLevel {
                rank: 5,
                level: 3,
                nft_num: 445,
                account_address: "0x40...e871".to_string(),
                version: 1,
                character: "Ainy".to_string(),
            },
            LeaderboardItemLevel {
                rank: 6,
                level: 3,
                nft_num: 472,
                account_address: "0x4d...f90f".to_string(),
                version: 1,
                character: "Bumy".to_string(),
            },
            LeaderboardItemLevel {
                rank: 7,
                level: 3,
                nft_num: 538,
                account_address: "0x9C...126C".to_string(),
                version: 1,
                character: "Ainy".to_string(),
            },
            LeaderboardItemLevel {
                rank: 8,
                level: 3,
                nft_num: 1903,
                account_address: "0xc9...55Ac".to_string(),
                version: 1,
                character: "Comy".to_string(),
            },
            LeaderboardItemLevel {
                rank: 9,
                level: 3,
                nft_num: 1921,
                account_address: "0x3d...65Dc".to_string(),
                version: 1,
                character: "Comy".to_string(),
            },
            LeaderboardItemLevel {
                rank: 10,
                level: 3,
                nft_num: 1906,
                account_address: "0xA6...5F98".to_string(),
                version: 1,
                character: "Comy".to_string(),
            },
        ]
    }

    pub fn s2_voting_data() -> Vec<LeaderboardItemVoting> {
        vec![
            LeaderboardItemVoting {
                account_address: "0x18...8edA".to_string(),
                voting_count: 541,
                rank: 1,
                version: 2,
            },
            LeaderboardItemVoting {
                account_address: "0x3d...65Dc".to_string(),
                voting_count: 540,
                rank: 2,
                version: 2,
            },
            LeaderboardItemVoting {
                account_address: "0xA6...5F98".to_string(),
                voting_count: 540,
                rank: 3,
                version: 2,
            },
            LeaderboardItemVoting {
                account_address: "0xA6...9Dae".to_string(),
                voting_count: 540,
                rank: 4,
                version: 2,
            },
            LeaderboardItemVoting {
                account_address: "0x35...4fe6".to_string(),
                voting_count: 540,
                rank: 5,
                version: 2,
            },
            LeaderboardItemVoting {
                account_address: "0xf9...4C79".to_string(),
                voting_count: 230,
                rank: 6,
                version: 2,
            },
            LeaderboardItemVoting {
                account_address: "0x73...37e0".to_string(),
                voting_count: 228,
                rank: 7,
                version: 2,
            },
            LeaderboardItemVoting {
                account_address: "0x0e...5851".to_string(),
                voting_count: 216,
                rank: 8,
                version: 2,
            },
            LeaderboardItemVoting {
                account_address: "0x2D...528b".to_string(),
                voting_count: 194,
                rank: 9,
                version: 2,
            },
            LeaderboardItemVoting {
                account_address: "0x4d...f90f".to_string(),
                voting_count: 122,
                rank: 10,
                version: 2,
            },
        ]
    }

    pub fn s2_daily_mission_data() -> Vec<LeaderboardItemDailyMission> {
        vec![
            LeaderboardItemDailyMission {
                account_address: "0xa6...9dae".to_string(),
                daily_count: 110,
                rank: 1,
                version: 2,
            },
            LeaderboardItemDailyMission {
                account_address: "0xa6...5f98".to_string(),
                daily_count: 109,
                rank: 2,
                version: 2,
            },
            LeaderboardItemDailyMission {
                account_address: "0x3d...65dc".to_string(),
                daily_count: 108,
                rank: 3,
                version: 2,
            },
            LeaderboardItemDailyMission {
                account_address: "0x18...8eda".to_string(),
                daily_count: 108,
                rank: 4,
                version: 2,
            },
            LeaderboardItemDailyMission {
                account_address: "0x35...4fe6".to_string(),
                daily_count: 108,
                rank: 5,
                version: 2,
            },
            LeaderboardItemDailyMission {
                account_address: "0xdb...0217".to_string(),
                daily_count: 94,
                rank: 6,
                version: 2,
            },
            LeaderboardItemDailyMission {
                account_address: "0x4d...f90f".to_string(),
                daily_count: 79,
                rank: 7,
                version: 2,
            },
            LeaderboardItemDailyMission {
                account_address: "0x73...37e0".to_string(),
                daily_count: 57,
                rank: 8,
                version: 2,
            },
            LeaderboardItemDailyMission {
                account_address: "0x0e...5851".to_string(),
                daily_count: 57,
                rank: 9,
                version: 2,
            },
            LeaderboardItemDailyMission {
                account_address: "0xf9...4c79".to_string(),
                daily_count: 55,
                rank: 10,
                version: 2,
            },
        ]
    }

    pub fn s2_exp_data() -> Vec<LeaderboardItemExperience> {
        vec![
            LeaderboardItemExperience {
                rank: 1,
                experience: 44090,
                nft_num: 301,
                account_address: "0x0e...5851".to_string(),
                version: 2,
                character: "Ainy".to_string(),
            },
            LeaderboardItemExperience {
                rank: 2,
                experience: 26760,
                nft_num: 174,
                account_address: "0x73...37e0".to_string(),
                version: 2,
                character: "Comy".to_string(),
            },
            LeaderboardItemExperience {
                rank: 3,
                experience: 23200,
                nft_num: 685,
                account_address: "0xf9...4C79".to_string(),
                version: 2,
                character: "Ainy".to_string(),
            },
            LeaderboardItemExperience {
                rank: 4,
                experience: 22590,
                nft_num: 107,
                account_address: "0x35...4fe6".to_string(),
                version: 2,
                character: "Comy".to_string(),
            },
            LeaderboardItemExperience {
                rank: 5,
                experience: 22590,
                nft_num: 54,
                account_address: "0xA6...9Dae".to_string(),
                version: 2,
                character: "Comy".to_string(),
            },
            LeaderboardItemExperience {
                rank: 6,
                experience: 22590,
                nft_num: 1906,
                account_address: "0xA6...5F98".to_string(),
                version: 2,
                character: "Comy".to_string(),
            },
            LeaderboardItemExperience {
                rank: 7,
                experience: 22590,
                nft_num: 1162,
                account_address: "0x35...4fe6".to_string(),
                version: 2,
                character: "Ainy".to_string(),
            },
            LeaderboardItemExperience {
                rank: 8,
                experience: 22590,
                nft_num: 1921,
                account_address: "0xA6...9Dae".to_string(),
                version: 2,
                character: "Comy".to_string(),
            },
            LeaderboardItemExperience {
                rank: 9,
                experience: 22500,
                nft_num: 241,
                account_address: "0xDB...0217".to_string(),
                version: 2,
                character: "Comy".to_string(),
            },
            LeaderboardItemExperience {
                rank: 10,
                experience: 20390,
                nft_num: 1162,
                account_address: "0xA6...5F98".to_string(),
                version: 2,
                character: "Ainy".to_string(),
            },
        ]
    }

    pub fn s2_level_data() -> Vec<LeaderboardItemLevel> {
        vec![
            LeaderboardItemLevel {
                rank: 1,
                level: 6,
                nft_num: 107,
                account_address: "0x35...4fe6".to_string(),
                version: 2,
                character: "Comy".to_string(),
            },
            LeaderboardItemLevel {
                rank: 2,
                level: 6,
                nft_num: 54,
                account_address: "0xA6...9Dae".to_string(),
                version: 2,
                character: "Comy".to_string(),
            },
            LeaderboardItemLevel {
                rank: 3,
                level: 6,
                nft_num: 1906,
                account_address: "0xA6...5F98".to_string(),
                version: 2,
                character: "Comy".to_string(),
            },
            LeaderboardItemLevel {
                rank: 4,
                level: 6,
                nft_num: 301,
                account_address: "0x0e...5851".to_string(),
                version: 2,
                character: "Ainy".to_string(),
            },
            LeaderboardItemLevel {
                rank: 5,
                level: 6,
                nft_num: 685,
                account_address: "0xf9...4C79".to_string(),
                version: 2,
                character: "Ainy".to_string(),
            },
            LeaderboardItemLevel {
                rank: 6,
                level: 6,
                nft_num: 1162,
                account_address: "0x35...4fe6".to_string(),
                version: 2,
                character: "Ainy".to_string(),
            },
            LeaderboardItemLevel {
                rank: 7,
                level: 6,
                nft_num: 1921,
                account_address: "0xA6...9Dae".to_string(),
                version: 2,
                character: "Comy".to_string(),
            },
            LeaderboardItemLevel {
                rank: 8,
                level: 6,
                nft_num: 174,
                account_address: "0x73...37e0".to_string(),
                version: 2,
                character: "Comy".to_string(),
            },
            LeaderboardItemLevel {
                rank: 9,
                level: 6,
                nft_num: 241,
                account_address: "0xDB...0217".to_string(),
                version: 2,
                character: "Comy".to_string(),
            },
            LeaderboardItemLevel {
                rank: 10,
                level: 5,
                nft_num: 901,
                account_address: "0x18...4C26".to_string(),
                version: 2,
                character: "Comy".to_string(),
            },
        ]
    }
}
