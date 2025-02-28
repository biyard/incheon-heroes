use dioxus_translate::*;

translate! {
    ContentsByIdTranslate;

    title: {
        ko: "CONTENTS_BY_ID",
        en: "CONTENTS_BY_ID",
    },

    more_contents_lead: {
        ko: "",
        en: "More from ",
    }

    more_contents_tail: {
        ko: "작가의 다른 작품",
        en: "",
    }
}

translate! {
    MintingPopupTranslate;


    confirm_text : {
        ko: "확인",
        en: "Confirm",
    }

    title : {
        ko: "Minting",
        en: "Minting",
    }

    description : {
        ko: "민팅을 진행함으로써 지갑에 해당 NFT 이미지의 사본을 추가로 소유하고 무료로 사용할 수 있습니다. NFT는 지적 재산권, 로열티 및 블록체인 거래 수수료가 적용될 수 있는 디지털 자산으므로, 향후 2차 판매 시 오리지널 제작자에 대한 로열티와 거래 수수료가 발생할 수 있으니 참고하시기 바랍니다.",
        en: "By proceeding with minting, you will own an additional copy of this NFT image in your wallet and be able to use it for free. Please note that royalties to the original creator and blockchain transaction fees may apply in future secondary sales.",
    }

    agreement : {
        ko: "NFT 민팅과 관련된 이용 약관 및 스마트 컨트랙트 조건에 동의합니다.",
        en: "I agree to the terms of use and smart contract terms related to NFT minting.",
    }

    loading_text : {
        ko: "NFT를 민팅 중",
        en: "Minting of NFTs is in progress.",
    }

    complete_text : {
        ko: "NFT가 성공적으로 민팅되었습니다!",
        en: "NFT has been minted successfully!",
    }
}
translate! {
    NftDescriptionTranslate;

    downloads: {
        ko: "NFT 발행",
        en: "Minted",
    },

    button_text : {
        ko: "Mint Now",
        en: "Mint Now",
    }
}
