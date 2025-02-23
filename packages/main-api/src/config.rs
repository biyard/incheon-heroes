use by_types::config::*;

#[derive(Debug)]
pub struct Config {
    pub env: &'static str,
    pub aws: AwsConfig,
    pub database: DatabaseConfig,
    pub bucket: BucketConfig,
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
            aws: AwsConfig::default(),
            database: DatabaseConfig::default(),
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
