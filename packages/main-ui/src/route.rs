use crate::pages::*;
use dioxus::prelude::*;
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

    #[end_layout]
    #[end_nest]

    #[redirect("/", || Route::HomePage { lang: Language::Ko })]
    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}
