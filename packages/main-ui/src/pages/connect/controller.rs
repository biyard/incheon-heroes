use by_macros::*;
use dioxus::prelude::*;
use dioxus_translate::Language;

use crate::{config, route::Route, services::backend_api::BackendApi};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    pub backend_api: BackendApi,
    pub nav: Navigator,
    pub lang: Language,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let ctrl = Self {
            backend_api: use_context(),
            nav: use_navigator(),
            lang,
        };

        Ok(ctrl)
    }

    pub async fn handle_google(&self) {}

    pub async fn handle_kakao(&self) {
        let conf = &config::get().kakao;

        let client = dioxus_oauth::prelude::OAuthClient::new(
            conf.client_id,
            conf.redirect_uri,
            "https://kauth.kakao.com/oauth/authorize",
            "https://kauth.kakao.com/oauth/token",
        )
        .set_openid_url("https://kauth.kakao.com/oauth/tokeninfo");

        let code: String = match client.get_auth_code().await {
            Ok(code) => code,
            Err(e) => {
                tracing::error!("Auth code failed: {:?}", e);
                return;
            }
        };

        let token_response: dioxus_oauth::prelude::TokenResponse =
            match client.get_token(code.as_str()).await {
                Ok(token_response) => token_response,
                Err(e) => {
                    tracing::error!("Token response failed: {:?}", e);
                    return;
                }
            };
        tracing::debug!("Token response: {:?}", token_response);

        let oid_response: dioxus_oauth::prelude::OpenIdResponse =
            match client.get_openid(&token_response.id_token).await {
                Ok(oid_response) => oid_response,
                Err(e) => {
                    tracing::error!("Token response failed: {:?}", e);
                    return;
                }
            };

        tracing::debug!("OID response: {:?}", oid_response);

        let hint = match self
            .backend_api
            .get_account_hint("kakao", &token_response.id_token, conf.redirect_uri)
            .await
        {
            Ok(hint) => hint,
            Err(e) => {
                tracing::error!("Failed to get account hint: {:?}", e);
                return;
            }
        };

        tracing::debug!("Account hint: {:?}", hint);

        self.nav.replace(Route::LoginPage {
            lang: self.lang,
            hint,
        });
    }

    pub async fn handle_kaikas(&self) {}

    pub async fn handle_internet_identity(&self) {}
}
