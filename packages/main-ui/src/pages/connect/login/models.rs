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
}
