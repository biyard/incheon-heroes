use dioxus_translate::Translate;

#[derive(
    Debug, Clone, Copy, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default, Translate,
)]
#[serde(rename_all = "snake_case")]
pub enum ProfileTabs {
    #[default]
    #[translate(ko = "미션 진행 내역", en = "Mission History")]
    MissionHistory,
    #[translate(ko = "경험치 획득 내역", en = "Experience History")]
    ExperienceHistory,
    #[translate(ko = "NFT 전송 내역", en = "NFT Transfer History")]
    NftTransferHistory,
    #[translate(ko = "상품 구매 내역", en = "Goods Purchase History")]
    GoodsPurchaseHistory,
}
