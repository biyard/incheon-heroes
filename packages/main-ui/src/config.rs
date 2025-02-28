#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct FirebaseConfig {
    pub api_key: String,
    pub auth_domain: String,
    pub project_id: String,
    pub storage_bucket: String,
    pub messaging_sender_id: String,
    pub app_id: String,
    pub measurement_id: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct KakaoConfig {
    pub client_id: &'static str,
    pub redirect_uri: &'static str,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct KlaytnConfig {
    pub endpoint: &'static str,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct IcpConfig {
    pub endpoint: &'static str,
    pub canister_id: &'static str,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ContractConfig {
    pub shop: &'static str,
    pub holder: &'static str,
    pub sbt: &'static str,
    pub experience: &'static str,
    pub nft: &'static str,
    pub goods: &'static str,
    pub mission: &'static str,
    pub account: &'static str,
    pub incheon_contents: &'static str,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub env: &'static str,
    pub domain: &'static str,
    pub main_api_endpoint: &'static str,
    pub new_api_endpoint: &'static str,
    pub discord_api_endpoint: &'static str,
    pub logs_api_endpoint: &'static str,
    pub nft_metadata_base_url: &'static str,
    pub klaytn_scope_endpoint: &'static str,

    pub opensea_endpoint: &'static str,
    pub firebase: FirebaseConfig,
    pub klaytn: KlaytnConfig,
    pub contracts: ContractConfig,
    pub kakao: KakaoConfig,
    pub icp: IcpConfig,

    pub discord_mission_url: &'static str,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            env: option_env!("ENV").expect("You must set ENV"),
            domain: option_env!("DOMAIN").expect("You must set DOMAIN"),
            new_api_endpoint: option_env!("NEW_API_ENDPOINT")
                .unwrap_or("https://api.incheonheroes.world"),
            main_api_endpoint: option_env!("MAIN_API_ENDPOINT")
                .unwrap_or("https://api.incheon.world"),
            discord_api_endpoint: option_env!("DISCORD_API_ENDPOINT")
                .unwrap_or("https://api.incheon.world"),
            logs_api_endpoint: option_env!("LOGS_API_ENDPOINT")
                .unwrap_or("https://logs-api.incheon.world"),
            klaytn_scope_endpoint: option_env!("KLAYTN_SCOPE_ENDPOINT")
                .unwrap_or("https://scope.klaytn.com/tx"),
            nft_metadata_base_url: option_env!("NFT_BASE_URI").expect("You must set NFT_BASE_URI"),
            opensea_endpoint: option_env!("OPENSEA_ENDPOINT")
                .expect("You must set OPENSEA_ENDPOINT"),
            firebase: FirebaseConfig {
                api_key: option_env!("FIREBASE_API_KEY")
                    .expect("You must set FIREBASE_API_KEY")
                    .to_string(),
                auth_domain: option_env!("FIREBASE_AUTH_DOMAIN")
                    .expect("You must set FIREBASE_AUTH_DOMAIN")
                    .to_string(),
                project_id: option_env!("FIREBASE_PROJECT_ID")
                    .expect("You must set FIREBASE_PROJECT_ID")
                    .to_string(),
                storage_bucket: option_env!("FIREBASE_STORAGE_BUCKET")
                    .expect("You must set FIREBASE_STORAGE_BUCKET")
                    .to_string(),
                messaging_sender_id: option_env!("FIREBASE_MESSAGING_SENDER_ID")
                    .expect("You must set FIREBASE_MESSAGING_SENDER_ID")
                    .to_string(),
                app_id: option_env!("FIREBASE_APP_ID")
                    .expect("You must set FIREBASE_APP_ID")
                    .to_string(),
                measurement_id: option_env!("FIREBASE_MEASUREMENT_ID")
                    .expect("You must set FIREBASE_MEASUREMENT_ID")
                    .to_string(),
            },
            klaytn: KlaytnConfig {
                endpoint: option_env!("KLAYTN_ENDPOINT").expect("You must set KLAYTN_ENDPOINT"),
            },
            contracts: ContractConfig {
                shop: option_env!("CONTRACT_SHOP").expect("You must set CONTRACT_SHOP"),
                account: option_env!("CONTRACT_ACCOUNT").expect("You must set CONTRACT_ACCOUNT"),
                holder: option_env!("CONTRACT_HOLDER").expect("You must set CONTRACT_HOLDER"),
                sbt: option_env!("CONTRACT_SBT").expect("You must set CONTRACT_SBT"),
                experience: option_env!("CONTRACT_EXPERIENCE")
                    .expect("You must set CONTRACT_EXPERIENCE"),
                nft: option_env!("CONTRACT_NFT").expect("You must set CONTRACT_NFT"),
                mission: option_env!("CONTRACT_MISSION").expect("You must set CONTRACT_MISSION"),
                goods: option_env!("CONTRACT_GOODS").expect("You must set CONTRACT_GOODS"),
                incheon_contents: option_env!("CONTRACT_INCHEON_CONTENTS")
                    .expect("You must set CONTRACT_INCHEON_CONTENTS"),
            },
            kakao: KakaoConfig {
                client_id: option_env!("KAKAO_CLIENT_ID").expect("You must set KAKAO_CLIENT_ID"),
                redirect_uri: option_env!("KAKAO_REDIRECT_URI")
                    .expect("You must set KAKAO_REDIRECT_URI"),
            },
            icp: IcpConfig {
                endpoint: option_env!("ICP_ENDPOINT").expect("You must set ICP_ENDPOINT"),
                canister_id: option_env!("ICP_CANISTER_ID").expect("You must set ICP_CANISTER_ID"),
            },
            discord_mission_url: option_env!("DISCORD_MISSION_URL")
                .expect("You must set DISCORD_MISSION_URL"),
        }
    }
}

static mut CONFIG: Option<Config> = None;

#[allow(static_mut_refs)]
pub fn get() -> &'static Config {
    unsafe {
        if CONFIG.is_none() {
            CONFIG = Some(Config::default());
        }
        CONFIG.as_ref().unwrap()
    }
}

pub fn log_level() -> tracing::Level {
    match option_env!("RUST_LOG") {
        Some("trace") => tracing::Level::TRACE,
        Some("debug") => tracing::Level::DEBUG,
        Some("info") => tracing::Level::INFO,
        Some("warn") => tracing::Level::WARN,
        Some("error") => tracing::Level::ERROR,
        _ => tracing::Level::INFO,
    }
}
