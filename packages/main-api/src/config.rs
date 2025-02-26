use by_types::config::*;

#[derive(Debug)]
pub struct Config {
    pub env: &'static str,
    pub aws: AwsConfig,
    pub auth: AuthConfig,
    pub database: DatabaseConfig,
    pub bucket: BucketConfig,
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
    pub owner_key: &'static str,
    pub owner_address: &'static str,
    pub feepayer_key: &'static str,
    pub feepayer_address: &'static str,
}

#[derive(Debug)]
pub struct BucketConfig {
    pub name: &'static str,
    pub asset_dir: &'static str,
    pub expire: u64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            env: option_env!("ENV").expect("You must set ENV"),
            // base_domain: option_env!("BASE_DOMAIN").expect("You must set BASE_DOMAIN"),
            // asset_dir: option_env!("ASSET_DIR").expect("You must set ASSET_DIR"),
            klaytn: KlaytnConfig {
                endpoint: option_env!("KLAYTN_ENDPOINT").expect("You must set KLAYTN_ENDPOINT"),
                owner_key: option_env!("KLAYTN_OWNER_KEY").expect("You must set KLAYTN_OWNER_KEY"),
                owner_address: option_env!("KLAYTN_OWNER_ADDR").expect("You must set KLAYTN_OWNER_ADDRESS"),
                feepayer_key: option_env!("KLAYTN_FEEPAYER_KEY").expect("You must set KLAYTN_FEEPAYER_KEY"),
                feepayer_address: option_env!("KLAYTN_FEEPAYER_ADDR").expect("You must set KLAYTN_FEEPAYER_ADDRESS"),
            },
            contracts: ContractConfig {
                incheon_contents: option_env!("CONTRACT_INCHEON_CONTENTS").expect("You must set CONTRACT_INCHEON_CONTENTS"),
            },
            aws: AwsConfig::default(),
            database: DatabaseConfig::default(),
            auth: AuthConfig::default(),
            bucket: BucketConfig {
                name: option_env!("BUCKET_NAME").expect("You must set BUCKET_NAME"),
                asset_dir: option_env!("ASSET_DIR").expect("You must set ASSET_DIR"),
                expire: option_env!("BUCKET_EXPIRE").unwrap_or_else(|| {
                    tracing::warn!("We recommend to set BUCKET_EXPIRE. BUCKET_EXPIRE is not set. Default is 3600.");
                    "3600"
                }) .parse()
                    .unwrap(),
            },
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
