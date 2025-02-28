use dioxus_translate::Translate;
use dto::wallets::kaikas_wallet::KaikasWallet;
use ethers::prelude::*;
use ethers::signers::LocalWallet;
use ethers::utils::to_checksum;
use ic_agent::identity::BasicIdentity;
use ic_agent::Identity;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default, Translate)]
#[serde(rename_all = "snake_case")]
pub enum UserWallet {
    #[translate(ko = "소셜 지갑", en = "Social Wallet")]
    SocialWallet {
        private_key: String,
        seed: String,
        checksum_address: String,
        principal: String,
    },
    #[translate(ko = "카이아 지갑", en = "Kaia Wallet")]
    KaiaWallet(KaikasWallet),

    #[default]
    #[translate(ko = "없음")]
    None,
}

impl UserWallet {
    pub fn seed(&self) -> Option<String> {
        match self {
            UserWallet::SocialWallet { seed, .. } => Some(seed.clone()),
            _ => None,
        }
    }

    pub fn can_cached(&self) -> bool {
        match self {
            UserWallet::SocialWallet { .. } => true,
            _ => false,
        }
    }

    pub fn icp_identity(&self) -> Option<BasicIdentity> {
        match self {
            UserWallet::SocialWallet { seed, .. } => Some(create_identity(seed)),
            _ => None,
        }
    }

    pub fn evm_address(&self) -> Option<String> {
        match self {
            UserWallet::SocialWallet {
                checksum_address, ..
            } => Some(checksum_address.clone()),
            UserWallet::KaiaWallet(wallet) => Some(wallet.address.clone()),
            _ => None,
        }
    }

    pub fn principal(&self) -> Option<String> {
        match self {
            UserWallet::SocialWallet { principal, .. } => Some(principal.clone()),
            _ => None,
        }
    }
}

pub fn create_identity(seed_hexstr: &str) -> BasicIdentity {
    let mut hasher = Sha256::new();
    hasher.update(format!("0x{}", seed_hexstr.trim_start_matches("0x")).as_bytes());
    let hash = hasher.finalize();
    let seed: [u8; 32] = hash.into();

    let key = ed25519_consensus::SigningKey::from(seed);
    let identity = ic_agent::identity::BasicIdentity::from_signing_key(key);
    let principal = identity.sender().unwrap();
    tracing::debug!("Principal: {principal}");

    identity
}

/// Creates a wallet from a seed by performing BIP32 derivation and then computing
/// the associated ECDSA key pair and Ethereum address. The derivation path used is "m/44'/60'/0'/0/0".
pub fn create_evm_wallet(seed: &[u8]) -> Result<EvmWallet, Box<dyn std::error::Error>> {
    use bip32::{DerivationPath, XPrv};
    use std::str::FromStr;

    // Create a BIP32 root key from the seed.
    // Derive the child key using the Ethereum derivation path.
    let derivation_path = DerivationPath::from_str("m/44'/60'/0'/0/0")?;
    let child_xprv = XPrv::derive_from_path(seed, &derivation_path)?;

    // Get the 32-byte private key.
    let private_key = hex::encode(child_xprv.private_key().to_bytes());
    let wallet: LocalWallet = private_key.parse().unwrap();

    let address = wallet.address();
    let checksum_address = to_checksum(&address, None);

    Ok(EvmWallet {
        private_key,
        seed: format!("0x{}", hex::encode(seed)),
        address: checksum_address.to_lowercase(),
        checksum_address,
    })
}

#[derive(Debug)]
pub struct EvmWallet {
    pub private_key: String,
    pub seed: String,
    pub checksum_address: String,
    pub address: String,
}
