use dioxus_translate::*;

translate! {
    NewContentsTranslate;

    title: {
        ko: "NFT 생성하기",
        en: "Create Your NFT",
    },

    description : {
        ko:"나만의 NFT를 만들어 블록체인에 안전하게 등록하세요. 간편한 업로드와 커스터마이징으로 몇 번의 클릭만으로 완성할 수 있습니다.\nNFT를 등록하고 민팅이 완료될 때마다 포인트를 받을 수 있습니다. 자세한 내용은 이벤트 페이지에서 확인하세요.",
        en:"Create your own NFT and safely register it on the blockchain. You can complete it with just a few clicks with easy upload and customization.\nYou can receive points every time you register an NFT and mint it. For more information, please check the event page.",
    }

    label_title: {
        ko: "제목",
        en: "Title",
    },

    label_thumbnail: {
        ko: "썸네일",
        en: "Thumbnail",
    },

    label_description: {
        ko: "설명",
        en: "Description",
    },

    label_fileupload: {
        ko: "파일 업로드",
        en: "File Upload",
    },

    btn_delete: {
        ko: "삭제",
        en: "Remove",
    },

    label_source: {
        ko: "원본 파일",
        en: "Source File",
    },

    placeholder_source: {
        ko: "원본 파일(SVG, AI 등)을 업로드하세요",
        en: "Upload the source file like SVG, AI, etc.",
    },

    placeholder_title: {
        ko: "제목을 입력하세요",
        en: "Enter the title",
    },

    placeholder_fileupload: {
        ko: "클릭하여 파일 선택하기 또는 여기로 파일 드래그",
        en: "Click to select a file or drag a file here",
    }

    placeholder_description: {
        ko: "설명은 검색 태그에 도움이 됩니다. 검색 가시성을 향상시키기 위해 관련 키워드를 포함해 주세요.",
        en: "Description helps with search tags. Please include relevant keywords to improve search visibility.",
    },

    note_fileupload: {
        ko: "파일은 PNG, JPG, GIF, MP4, WEBM, MP3, WAV, GLB, GLTF 등을 지원합니다. 최대 100MB까지 업로드 가능합니다.",
        en: "Maximum file size is 100MB. Files exceeding this limit cannot be uploaded. (PNG, JPG, GIF, MP4, WEBM, MP3, WAV, GLB, GLTF)",
    },

    btn_add_nft: {
        ko: "NFT 추가하기",
        en: "Add NFT",
    },

    btn_submit_nft: {
        ko: "NFT 만들기",
        en: "Create your NFT",
    },

    btn_cancel: {
        ko: "취소",
        en: "Cancel",
    },
}

translate! {
    ConsentPopupTranslate;

    title: {
        ko: "NFT 민팅 및 사용권 동의",
        en: "NFT Minting and Usage Agreement",
    },

    minting_consent: {
        ko: "인천히어로즈에서 NFT로 민팅된 모든 이미지는 오픈소스로 제공되며, 저작권 및 사용 권한이 공개되어 전 세계 누구나 제한 없이 자유롭게 활용, 수정, 배포할 수 있음을 확인하며 이에 동의합니다.",
        en: "All images minted as NFTs by Incheon Heroes are provided as open source, with copyright and usage rights publicly disclosed, allowing anyone worldwide to freely use, modify, and distribute them without restrictions. I acknowledge and agree to these terms",
    },

    accept: {
        ko: "동의 및 NFT 생성",
        en: "Agree and Create NFT",
    },
    decline: {
        ko: "거부하기",
        en: "Decline",
    },
}
