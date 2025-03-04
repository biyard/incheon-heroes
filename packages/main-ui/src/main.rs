pub mod assets;
pub mod components;
pub mod config;
pub mod models;
pub mod pages;
pub mod route;
pub mod services;
pub mod utils;

use crate::route::Route;
use by_components::responsive::Responsive;
use by_components::theme::{CardColorTheme, ColorTheme, TextColorTheme};
use dioxus::prelude::*;
use dioxus_popup::PopupService;
use services::google_service::GoogleService;
use services::kakao_service::KakaoService;
use services::{
    backend_api::BackendApi, icp_canister::IcpCanister, klaytn::Klaytn, user_service::UserService,
};

fn main() {
    dioxus_logger::init(config::log_level()).expect("failed to init logger");
    tracing::debug!("config: {:?}", config::get());

    dioxus_aws::launch(app);
}

fn app() -> Element {
    std::panic::set_hook(Box::new(|info| {
        tracing::error!("Panic: {}", info);
    }));

    use_context_provider(|| ColorTheme {
        background: "#E9F2EC",
        card: CardColorTheme {
            primary: "#F3F7F4",
            ..Default::default()
        },
        text: TextColorTheme {
            primary: "black",
            ..Default::default()
        },
        ..Default::default()
    });

    Klaytn::init();
    BackendApi::init();
    IcpCanister::init();
    UserService::init();
    PopupService::init();
    GoogleService::init();
    KakaoService::init();

    rsx! {
        btracing::ToastTracing {}

        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1.0",
        }

        document::Link { href: "https://fonts.googleapis.com", rel: "preconnect" }
        document::Link {
            crossorigin: "false",
            href: "https://fonts.gstatic.com",
            rel: "preconnect",
        }
        document::Script { src: "https://d3js.org/d3.v7.min.js" }
        document::Link {
            href: "https://fonts.googleapis.com/css2?family=Inter:wght@100..900&family=Russo+One&display=swap",
            rel: "stylesheet",
        }
        document::Link {
            href: "https://cdn.jsdelivr.net/gh/fonts-archive/Pretendard/Pretendard.css",
            r#type: "text/css",
            rel: "stylesheet",
        }

        document::Link {
            href: "https://fonts.googleapis.com/css2?family=Inter:wght@100..900&family=Noto+Color+Emoji&family=Russo+One&display=swap",
            rel: "stylesheet",
        }
        document::Link { id: "favicon", rel: "icon", href: "{assets::FAVICON}" }
        document::Link { rel: "stylesheet", href: asset!("/public/main.css") }
        document::Link { rel: "stylesheet", href: asset!("/public/tailwind.css") }

        document::Script { src: asset!("/public/dep.js") }
        document::Script { src: "https://cdn.tailwindcss.com/3.4.16" }
        document::Link {
            href: "https://cdn.jsdelivr.net/npm/daisyui@4.12.23/dist/full.min.css",
            r#type: "text/css",
            rel: "stylesheet",
        }
        document::Script { src: "https://cdn.tailwindcss.com" }
        Responsive { mobile_first: false, desktop: 1200.1, Router::<Route> {} }
    }
}

#[cfg(feature = "server")]
mod api {
    use dioxus::fullstack::prelude::*;
    use server_fn::codec::{GetUrl, Json};

    #[server(endpoint = "/version", input=GetUrl, output=Json)]
    pub async fn version() -> Result<String, ServerFnError> {
        Ok(match option_env!("VERSION") {
            Some(version) => match option_env!("COMMIT") {
                Some(commit) => format!("{}-{}", version, commit),
                None => format!("{}", version),
            },
            None => match option_env!("DATE") {
                Some(date) => date.to_string(),
                None => "unknown".to_string(),
            },
        }
        .to_string())
    }
}
