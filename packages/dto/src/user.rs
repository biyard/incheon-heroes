#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::{api_model, ApiModel};
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/", table = user)]
pub struct User {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(unique)]
    pub evm_address: String,
    pub email: String,
    pub subject: String,
    pub provider: Provider,
}

#[derive(Debug, Clone, Eq, PartialEq, ApiModel, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Provider {
    #[default]
    Kakao = 1,
    Google = 2,
}
