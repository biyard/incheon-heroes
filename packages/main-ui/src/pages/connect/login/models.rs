use dto::UserAuthProvider;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    by_macros::EnumProp,
    Default,
)]
#[serde(rename_all = "kebab-case")]
pub enum LoginProvider {
    #[default]
    Kakao,
    Google,
    Kaia,
}

impl Into<UserAuthProvider> for LoginProvider {
    fn into(self) -> UserAuthProvider {
        match self {
            Self::Kakao => UserAuthProvider::Kakao,
            Self::Google => UserAuthProvider::Google,
            Self::Kaia => UserAuthProvider::Kaia,
        }
    }
}
