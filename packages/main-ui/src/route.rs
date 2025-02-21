use crate::{pages::*, services::backend_api::AccountHint};
use dioxus::prelude::*;
use dioxus_oauth::component::OAuthPopup;
use dioxus_translate::Language;

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/:lang")]
    #[layout(RootLayout)]

    #[route("/")]
    HomePage { lang: Language },

    #[route("/notices")]
    NoticesPage { lang: Language },

    #[route("/stories")]
    StoriesPage { lang: Language },

    #[route("/contributors")]
    ContributorsPage { lang: Language },

    #[nest("/events")]
    #[route("/calendar")]
    CalendarPage { lang: Language },
    #[route("/songs")]
    SongsPage { lang: Language },
    #[route("/songs/:id")]
    SongsByIdPage { lang: Language, id: String },
    #[end_nest]

    #[nest("/shop")]
    #[route("/")]
    ShopPage { lang: Language },
    #[route("/:id")]
    ShopByIdPage { lang: Language, id: String },
    #[end_nest]

    #[route("/faq")]
    FaqPage { lang: Language },

    #[nest("/my-nfts")]
    #[route("/")]
    MyNftsPage { lang: Language },
    #[route("/:id")]
    MyNftsByIdPage { lang: Language, id: String },
    #[end_nest]

    #[route("/connect")]
    ConnectPage { lang: Language },

    #[route("/connect/login")]
    LoginPage { lang: Language, hint: AccountHint },

    #[end_layout]
    #[end_nest]

    #[route("/oauth/kakao")]
    OAuthPopup { },

    #[redirect("/", || Route::HomePage { lang: Language::Ko })]
    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}

impl Route {
    pub fn switch_lang(self) -> Self {
        match self {
            Route::LoginPage { lang, hint } => Route::LoginPage {
                lang: lang.switch(),
                hint,
            },
            Route::ConnectPage { lang } => Route::ConnectPage {
                lang: lang.switch(),
            },
            Route::HomePage { lang } => Route::HomePage {
                lang: lang.switch(),
            },
            Route::NoticesPage { lang } => Route::NoticesPage {
                lang: lang.switch(),
            },
            Route::StoriesPage { lang } => Route::StoriesPage {
                lang: lang.switch(),
            },
            Route::ContributorsPage { lang } => Route::ContributorsPage {
                lang: lang.switch(),
            },
            Route::CalendarPage { lang } => Route::CalendarPage {
                lang: lang.switch(),
            },
            Route::SongsPage { lang } => Route::SongsPage {
                lang: lang.switch(),
            },
            Route::SongsByIdPage { lang, id } => Route::SongsByIdPage {
                lang: lang.switch(),
                id,
            },
            Route::ShopPage { lang } => Route::ShopPage {
                lang: lang.switch(),
            },
            Route::ShopByIdPage { lang, id } => Route::ShopByIdPage {
                lang: lang.switch(),
                id,
            },
            Route::FaqPage { lang } => Route::FaqPage {
                lang: lang.switch(),
            },
            Route::MyNftsPage { lang } => Route::MyNftsPage {
                lang: lang.switch(),
            },
            Route::MyNftsByIdPage { lang, id } => Route::MyNftsByIdPage {
                lang: lang.switch(),
                id,
            },
            Route::NotFoundPage { route } => Route::NotFoundPage { route },
            Route::OAuthPopup {} => Route::OAuthPopup {},
        }
    }
}
