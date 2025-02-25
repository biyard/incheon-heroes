use crate::pages::*;
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

    #[route("/shop")]
    ShopPage { lang: Language },
    #[route("/shop/:id")]
    ShopByIdPage { lang: Language, id: String },

    #[route("/faq")]
    FaqPage { lang: Language },

    #[route("/my-nfts")]
    MyNftsPage { lang: Language },
    #[route("/my-nfts/:id")]
    MyNftsByIdPage { lang: Language, id: String },

    #[route("/connect")]
    ConnectPage { lang: Language },

    #[route("/connect/login?:id&:provider&:hint&:address&:email&:picture")]
    LoginPage { lang: Language, id:String, provider: LoginProvider, hint: String, address: String, email: String, picture: String },

    #[route("/my-profile")]
    MyProfilePage { lang: Language },

    #[route("/contents")]
    ContentsPage { lang: Language },
    #[route("/contents/new")]
    NewContentsPage { lang: Language },

    #[route("/contents/:id")]
    ContentsByIdPage { id: i64, lang: Language },

    #[end_layout]
    #[end_nest]

    #[route("/oauth/kakao")]
    OAuthPopup { },

    #[redirect("/", || Route::HomePage { lang: Language::Ko })]
    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}

impl Route {
    pub fn should_hide_footer(&self) -> bool {
        match self {
            Route::NewContentsPage { .. } => true,
            Route::ContentsPage { .. } => true,
            _ => false,
        }
    }

    pub fn switch_lang(self) -> Self {
        match self {
            Route::ContentsByIdPage { id, lang } => Route::ContentsByIdPage {
                id,
                lang: lang.switch(),
            },
            Route::NewContentsPage { lang } => Route::NewContentsPage {
                lang: lang.switch(),
            },
            Route::ContentsPage { lang } => Route::ContentsPage {
                lang: lang.switch(),
            },
            Route::MyProfilePage { lang } => Route::MyProfilePage {
                lang: lang.switch(),
            },
            Route::LoginPage {
                lang,
                id,
                provider,
                hint,
                address,
                email,
                picture,
            } => Route::LoginPage {
                lang: lang.switch(),
                provider,
                id,
                hint,
                address,
                email,
                picture,
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
