#![allow(unused_variables)]
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::{api_model, ApiModel};
use validator::Validate;

use crate::ContentSummary;

#[derive(Validate)]
#[api_model(base = "/v1/users", table = users, response = [signup_or_login(UserResponse)])]
pub struct User {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(unique, action = [signup_or_login, register_or_login], read_action = get_user_by_address)]
    pub evm_address: String,
    #[api_model(unique, action = signup_or_login)]
    #[validate(email)]
    pub email: String,
    #[api_model(unique, action = signup_or_login)]
    pub subject: String,
    #[api_model(action = signup_or_login)]
    #[validate(url)]
    pub profile_url: String,
    #[api_model(action = [signup_or_login, register_or_login], type = INTEGER)]
    pub provider: UserAuthProvider,
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UserResponse {
    #[serde(flatten)]
    pub user: User,
    pub action: UserResponseType,
}

#[derive(Debug, Clone, Eq, PartialEq, ApiModel, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum UserResponseType {
    #[default]
    SignUp = 1,
    Login = 2,
}

#[derive(Debug, Clone, Eq, PartialEq, ApiModel, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum UserAuthProvider {
    #[default]
    Kakao = 1,
    Google = 2,
    Kaia = 3,
}

/// UserContents is read-only model for users table with contents.
#[api_model(base = "/v1/users/contents", table = users)]
pub struct UserContents {
    #[api_model(summary, primary_key)]
    pub id: i64,

    #[api_model(action = signup_or_login)]
    pub profile_url: String,

    #[api_model(read_action = contents_by)]
    pub evm_address: String,

    #[api_model(one_to_many = contents, foreign_key = creator_id, type = JSONB)]
    pub contents: Vec<ContentSummary>,
}
