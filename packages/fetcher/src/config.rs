use by_types::config::*;

#[derive(Debug)]
pub struct Config {
    pub env: &'static str,
    pub database: DatabaseConfig,
    pub klaytn: KlaytnConfig,
    pub contracts: ContractConfig,
}

#[derive(Debug)]
pub struct ContractConfig {
    pub incheon_contents: &'static str,
}

#[derive(Debug)]
pub struct KlaytnConfig {
    pub endpoint: &'static str,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            env: option_env!("ENV").expect("You must set ENV"),
            klaytn: KlaytnConfig {
                endpoint: option_env!("KLAYTN_ENDPOINT").expect("You must set KLAYTN_ENDPOINT"),
            },
            contracts: ContractConfig {
                incheon_contents: option_env!("CONTRACT_INCHEON_CONTENTS")
                    .expect("You must set CONTRACT_INCHEON_CONTENTS"),
            },
            database: DatabaseConfig::default(),
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
        &CONFIG.as_ref().unwrap()
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
