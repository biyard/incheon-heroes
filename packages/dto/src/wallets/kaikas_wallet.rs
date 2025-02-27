use std::sync::Arc;

use async_trait::async_trait;
use ethers::{
    providers::{Http, Provider},
    types::Signature,
};

use crate::contracts::klaytn_transaction::KlaytnTransaction;

use super::KaiaWallet;
use crate::Result;

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub struct KaikasWallet {
    pub chain_id: u64,
    pub address: String,
}

#[cfg(feature = "web")]
use super::kaikas_browser::*;
#[cfg(feature = "web")]
use wasm_bindgen_futures::JsFuture;

#[cfg(not(feature = "web"))]
impl KaikasWallet {
    pub async fn new(_provider: Arc<Provider<Http>>) -> Result<Self> {
        Ok(Self::default())
    }

    pub async fn switch_chain(_chain_id: u64) -> Result<()> {
        Ok(())
    }
}

#[cfg(feature = "web")]
impl KaikasWallet {
    pub async fn new(provider: Arc<Provider<Http>>) -> Result<Self> {
        use ethers::providers::Middleware;

        let chain_id = provider
            .get_chainid()
            .await
            .map_err(|e| crate::Error::Klaytn(e.to_string()))?;

        let k = klaytn()?;
        let accounts: Vec<String> = match JsFuture::from(k.enable()).await {
            Ok(v) => serde_wasm_bindgen::from_value(v).unwrap_or_default(),
            Err(e) => {
                tracing::error!("failed to get accounts {e:?}");
                vec![]
            }
        };

        tracing::debug!("accounts : {accounts:?}");
        let address = accounts[0].clone();

        Self::switch_chain(chain_id.as_u64()).await?;

        tracing::debug!("selected address: {address}");

        Ok(Self {
            address,
            chain_id: chain_id.as_u64(),
        })
    }

    pub async fn switch_chain(chain_id: u64) -> Result<()> {
        let chain_id = format!("0x{chain_id:x}");
        let k = klaytn()?;
        let req = KaikasRequest {
            method: "wallet_switchKlaytnChain".to_string(),
            params: vec![SwitchChainRequest { chain_id: chain_id }],
        };

        let req = serde_wasm_bindgen::to_value(&req).unwrap();
        web_sys::console::log_1(&req);

        let _ = JsFuture::from(k.request(&req)).await;

        Ok(())
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwitchChainRequest {
    chain_id: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct KaikasRequest<T> {
    pub method: String,
    pub params: Vec<T>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    tx_type: u8,
    to: String,
    from: String,
    gas: String,
    gas_price: String,
    value: String,
    nonce: String,
    input: String,
}

#[cfg_attr(not(feature = "server"), async_trait(?Send))]
#[cfg_attr(feature = "server", async_trait)]
impl KaiaWallet for KaikasWallet {
    fn address(&self) -> ethers::types::H160 {
        unimplemented!()
    }

    #[cfg(not(feature = "web"))]
    async fn sign_transaction(&self, _tx: &KlaytnTransaction) -> Result<Signature> {
        unimplemented!()
    }

    #[cfg(feature = "web")]
    async fn sign_transaction(&self, tx: &KlaytnTransaction) -> Result<Signature> {
        use ethers::abi::AbiEncode;
        use ethers::types::U256;
        let req = KaikasRequest {
            method: "klay_signTransaction".to_string(),
            params: vec![Transaction {
                tx_type: tx.tx_type.to_tx_type_code(),
                to: tx.to.unwrap_or_default().encode_hex(),
                from: tx.from.unwrap_or_default().encode_hex(),
                gas: tx.gas.unwrap_or_default().encode_hex(),
                gas_price: tx.gas_price.unwrap_or_default().encode_hex(),
                value: tx.value.unwrap_or_default().encode_hex(),
                nonce: tx.nonce.unwrap_or_default().encode_hex(),
                input: format!("0x{}", hex::encode(tx.input.clone().unwrap_or_default())),
            }],
        };
        let req = serde_wasm_bindgen::to_value(&req).unwrap();
        web_sys::console::log_1(&req);
        let k = klaytn()?;
        let _res = match JsFuture::from(k.request(&req)).await {
            Ok(res) => {
                web_sys::console::log_1(&res);
                res
            }
            Err(e) => {
                web_sys::console::log_1(&e);
                return Err(crate::Error::SignError);
            }
        };

        Ok(Signature {
            r: U256::from(0),
            s: U256::from(0),
            v: 0,
        })
    }
}
