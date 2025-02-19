pub mod assets;
pub mod components;
pub mod config;
pub mod models;
pub mod pages;
pub mod route;

use crate::route::Route;
use by_components::theme::{CardColorTheme, ColorTheme, TextColorTheme};
use dioxus::prelude::*;

fn main() {
    let conf = config::get();
    dioxus_logger::init(conf.log_level).expect("failed to init logger");
    tracing::debug!("config: {:?}", conf);

    dioxus_aws::launch(app);
}

fn app() -> Element {
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

    rsx! {
        document::Title { "Incheon Heroes" }
        document::Meta { name: "description", content: "Incheon heroes ...." }
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

        document::Script { src: "https://cdn.tailwindcss.com/3.4.16" }
        document::Link {
            href: "https://cdn.jsdelivr.net/npm/daisyui@4.12.23/dist/full.min.css",
            r#type: "text/css",
            rel: "stylesheet",
        }
        document::Script { src: "https://cdn.tailwindcss.com" }
        Router::<Route> {}
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
