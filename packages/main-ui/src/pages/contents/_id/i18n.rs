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
    MintNowButtonTranslate;

    button_text : {
        ko: "확인",
        en: "Confirm",
    }

    title : {
        ko: "Minting",
        en: "Minting",
    }

    description : {
        ko: "민팅을 진행함으로써 스마트 컨트랙트에 명시된 조건에 동의하게 됩니다. NFT는 지적 재산권, 로열티 및 블록체인 거래 수수료가 적용될 수 있는 디지털 자산입니다. 민팅을 진행하기 전에 법적 사항을 충분히 이해했는지 확인하세요.",
        en: "By minting, you agree to the terms and conditions specified in the smart contract. NFTs are digital assets that may be subject to intellectual property rights, royalties, and blockchain transaction fees. Make sure you fully understand the legal aspects before proceeding with minting.",
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
