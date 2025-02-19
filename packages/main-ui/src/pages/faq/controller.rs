use by_macros::DioxusController;
use dioxus::prelude::*;
use dioxus_translate::Language;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Faq {
    pub question: String,
    pub answer: String,
}

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    pub faqs: Resource<Vec<Faq>>,
    pub selected: Signal<Option<usize>>,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let faqs = use_server_future(move || async move {
            if lang == Language::Ko {
                vec![
                    Faq {
                        question: "인천유니버스는 무슨 프로젝트인가요?".to_string(),
                        answer: "인천의 과거와 현재, 미래의 모습이 여러 가지 가상세계에서 공유되고 이를 체험할 수 있도록 만든 인천시의 단일 세계관입니다. 블록체인 기술을 활용한 NFT를 연계하여 더 다양한 메타버스와 가상세계로 점차 확장될 예정입니다.".to_string(),
                    },
                    Faq {
                        question: "인천히어로즈는 어떤 프로젝트인가요?".to_string(),
                        answer: "인천유니버스의 중심이 되는 Membership NFT 입니다. 인천시의 대표 캐릭터인 점박이 물범을 작품화하여 인천을 사랑하고 인천의 도시가치에 공감하는 사람들을 소유자로 초대합니다. 멸종위기종인 점박이 물범들이 슈퍼히어로가 되어 지구와 바다의 환경을 지키고 기후변화에 동참하는 시민들과 함께하는 스토리를 담고 있습니다.".to_string(),
                    },
                    Faq {
                        question: "인천유니버스의 NFT는 어떻게 거래할 수 있나요?".to_string(),
                        answer: "민팅 일정과 상세정보가 공개되면 해당 페이지에서 확인할 수 있습니다 :)".to_string(),
                    },
                    Faq {
                        question: "인천유니버스의 멤버가 되면 어떤 혜택이 있나요?".to_string(),
                        answer: "마이크로버스마다 각자의 색깔이 있는 다양한 이벤트와 권한, 혜택이 준비되어 있어요! 자세한 사항은 각 마이크로버스 이벤트 페이지를 확인해 주세요 :)".to_string(),
                    },
                    Faq {
                        question: "마이크로버스는 무엇인가요?".to_string(),
                        answer: "우리 인천의 캐릭터들이 살아 숨쉬는 하나의 작은 세상이라고 할까요?\n각 마이크로버스마다 다양한 세계관, 캐릭터들이 살아 숨쉬고 있어요!".to_string(),
                    },
                    Faq {
                        question: "프로젝트에 참여하고 싶습니다. 어떻게 신청하고 받을 수 있나요?".to_string(),
                        answer: "인천유니버스 커뮤니티, SNS를 통해 프로젝트의 다양한 정보를 확인할 수 있어요! 자세한 정보는 인천유니버스에서 웹페이지에서도 확인 가능합니다:)".to_string(),
                    },
                    Faq {
                        question: "지갑은 어떻게 설치하나요?".to_string(),
                        answer: "1) 카이카스 지갑으로 로그인 하려는 경우 구글에서 kaikas를 검색 후 구글 webstore를 통해 지갑을 설치해주세요.\n2) 구글이나 카카오를 활용한 소셜 로그인도 가능합니다. 우측 상단의 클립 모양을 클릭해주세요.".to_string(),
                    }
                ]
            } else {
                vec![
                    Faq {
                        question: "What kind of project is Incheon Universe?".to_string(),
                        answer: "Incheon's past, present, and future are represented as a unified metaverse by the city, allowing various virtual worlds to be shared and experienced. Leveraging blockchain technology and NFTs, Incheon aims to expand into even more diverse metaverses and virtual realms over time.".to_string(),
                    },
                    Faq {
                        question: "What is IncheonHeroes?".to_string(),
                        answer: "IncheonHeroes is a Membership NFT at the core of Incheon Universe. It invites people who love Incheon and resonate with the city's values by transforming Incheon's representative character, Spotty Seal, into artwork. This NFT project features endangered Spotty Seals turning into superheroes, joining citizens in protecting the environment and combating climate change on Earth and in the oceans.".to_string(),
                    },
                    Faq {
                        question: "How can I trade Incheon Universe NFT?".to_string(),
                        answer: "minting schedule and detailed information are released, you can check it on the page :)".to_string(),
                    },
                    Faq {
                        question: "What benefits do I get from becoming a member of Incheon Universe?".to_string(),
                        answer: "There are various events and benefits prepared for each microverse. Check out each microverse event page for details :)".to_string(),
                    },
                    Faq {
                        question: "What is a microverse?".to_string(),
                        answer: "It's a small world where the characters of Incheon live and breathe?\nEach microverse has a variety of worldviews and characters living and breathing!".to_string(),
                    },
                    Faq {
                        question: "I want to join in the project. How can I apply?".to_string(),
                        answer: "You can check various information about the project through the Incheon Universe Community and SNS! For more information, you can also check the Incheon Universe website. :)".to_string(),
                    },
                    Faq {
                        question: "How do I install a wallet?".to_string(),
                        answer: "1) If you wish to log in with the Kaikas wallet, please search for 'Kaikas' on Google and install the wallet through the Google Web Store. \n2) You can also use social login options through Google or Kakao. Please click on the clip icon in the upper right corner.".to_string(),
                    }
                ]
            }
        })?;
        let ctrl = Self {
            faqs,
            selected: use_signal(|| None),
        };

        Ok(ctrl)
    }
}
