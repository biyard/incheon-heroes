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
    InternetIdentity,
}

impl Into<UserAuthProvider> for LoginProvider {
    fn into(self) -> UserAuthProvider {
        match self {
            Self::Kakao => UserAuthProvider::Kakao,
            Self::Google => UserAuthProvider::Google,
            Self::Kaia => UserAuthProvider::Kaia,
            Self::InternetIdentity => UserAuthProvider::InternetIdentity,
        }
    }
}
