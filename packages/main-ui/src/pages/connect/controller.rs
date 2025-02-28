use by_macros::*;
use dioxus::prelude::*;
use dioxus_oauth::prelude::FirebaseService;
use dioxus_translate::Language;
use dto::User;

use crate::{
    config,
    models::user_wallet::UserWallet,
    pages::LoginProvider,
    route::Route,
    services::{backend_api::BackendApi, user_service::UserService},
};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    pub backend_api: BackendApi,
    pub nav: Navigator,
    pub lang: Language,
    pub firebase: FirebaseService,
    pub user: UserService,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let ctrl = Self {
            backend_api: use_context(),
            nav: use_navigator(),
            lang,
            firebase: use_context(),
            user: use_context(),
        };

        Ok(ctrl)
    }

    pub async fn handle_google(&self) {
        let cred = self
            .firebase
            .sign_in_with_popup(vec![
                "https://www.googleapis.com/auth/drive.appdata".to_string()
            ])
            .await;

        tracing::debug!("cred: {:?}", cred);
        let conf = &config::get().kakao;

        let hint = match self
            .backend_api
            .get_account_hint("google", &cred.id_token, conf.redirect_uri)
            .await
        {
            Ok(hint) => hint,
            Err(e) => {
                tracing::error!("Failed to get account hint: {:?}", e);
                return;
            }
        };

        self.nav.replace(Route::LoginPage {
            lang: self.lang,
            provider: LoginProvider::Google,
            id: hint.id,
            hint: hint.private_key_hint,
            address: hint.address.unwrap_or_default(),
            email: cred.email,
            picture: cred.photo_url,
        });
    }

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
            provider: LoginProvider::Kakao,
            id: hint.id,
            hint: hint.private_key_hint,
            address: hint.address.unwrap_or_default(),
            email: oid_response.email.unwrap_or_default(),
            picture: oid_response.picture.unwrap_or_default(),
        });
    }

    pub async fn handle_kaia(&mut self) {
        let conf = config::get();
        let provider =
            ethers::providers::Provider::<ethers::providers::Http>::try_from(conf.klaytn.endpoint)
                .unwrap();
        let provider = std::sync::Arc::new(provider);

        let w = match dto::wallets::kaikas_wallet::KaikasWallet::new(provider).await {
            Ok(wallet) => wallet,
            Err(e) => {
                tracing::error!("Failed to get kaikas wallet: {:?}", e);
                return;
            }
        };

        self.user.set_wallet(UserWallet::KaiaWallet(w)).await;
        let endpoint = conf.new_api_endpoint;

        match User::get_client(endpoint)
            .register_or_login(
                self.user.evm_address().unwrap_or_default(),
                dto::UserAuthProvider::Kaia,
            )
            .await
        {
            Ok(user) => {
                self.user.set_user(user);
            }
            Err(e) => {
                tracing::error!("Failed to register or login: {:?}", e);
            }
        };

        if self.nav.can_go_back() {
            self.nav.go_back();
        } else {
            tracing::debug!("replace home page");
            self.nav.replace(Route::HomePage { lang: self.lang });
        }
    }

    pub async fn handle_internet_identity(&self) {}
}
