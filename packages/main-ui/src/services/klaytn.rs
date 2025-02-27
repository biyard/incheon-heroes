use dto::wallets::remote_fee_payer::RemoteFeePayer;
use dto::wallets::wallet::KaiaLocalWallet;
use dto::wallets::KaiaWallet;
use ethers::providers::Http;
use ethers::providers::Provider;
use std::sync::Arc;

use dioxus::prelude::*;

use crate::config;

use super::experience_contract::ExperienceContract;
use super::holder_contract::HolderContract;
use super::mission_contract::MissionContract;
use super::sbt_contract::SbtContract;
use super::shop_contract::ShopContract;
use super::user_service::UserService;

#[derive(Clone, Copy)]
pub struct Klaytn {
    pub shop: Signal<ShopContract<RemoteFeePayer, UserService>>,
    pub holder: Signal<HolderContract>,
    pub sbt: Signal<SbtContract>,
    pub experience: Signal<ExperienceContract>,
    pub mission: Signal<MissionContract>,

    pub provider: Signal<Arc<Provider<Http>>>,
}

impl Klaytn {
    pub fn init() {
        let conf = config::get();
        let provider = Provider::<Http>::try_from(conf.klaytn.endpoint).unwrap();
        let provider: Arc<Provider<Http>> = Arc::new(provider);

        let holder = HolderContract::new(conf.contracts.holder, provider.clone());
        let sbt = SbtContract::new(conf.contracts.sbt, provider.clone());
        let experience = ExperienceContract::new(
            conf.contracts.experience,
            conf.contracts.nft,
            provider.clone(),
        );
        let mission = MissionContract::new(
            conf.contracts.mission,
            conf.contracts.nft,
            conf.contracts.experience,
            provider.clone(),
        );

        let shop = ShopContract::new(conf.contracts.shop, provider.clone());

        let srv = Self {
            shop: use_signal(move || shop),
            holder: use_signal(move || holder),
            sbt: use_signal(move || sbt),
            experience: use_signal(move || experience),
            mission: use_signal(move || mission),

            provider: use_signal(move || provider),
        };

        use_context_provider(move || srv);
    }

    pub async fn set_signer(&mut self, signer: UserService) {
        let conf = config::get();
        let api_endpoint = conf.new_api_endpoint;
        let feepayer = match RemoteFeePayer::new(api_endpoint).await {
            Ok(feepayer) => feepayer,
            Err(e) => {
                tracing::error!("Failed to create fee payer: {}", e);
                return;
            }
        };

        let mut shop = self.shop.write();
        shop.set_wallet(signer);
        shop.set_fee_payer(feepayer);
    }
}
