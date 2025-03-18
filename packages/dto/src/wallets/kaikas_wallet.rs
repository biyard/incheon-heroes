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

#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

#[cfg(not(feature = "web"))]
impl KaikasWallet {
    pub async fn new(_provider: Arc<Provider<Http>>) -> Result<Self> {
        Ok(Self::default())
    }

    pub async fn switch_chain(_chain_id: u64) -> Result<()> {
        Ok(())
    }

    pub async fn sign_message(&self, _message: &str) -> Result<Signature> {
        Err(crate::Error::Unknown(
            "sign_message is not supported in non-web environments".to_string(),
        ))
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

    pub async fn listen_for_account_changes(callback: impl Fn(String) + 'static) -> Result<()> {
        let k = klaytn()?;
        let callback = Closure::wrap(Box::new(callback) as Box<dyn Fn(String)>);
        let _ = k.on("accountsChanged", callback.as_ref().unchecked_ref());
        callback.forget();
        Ok(())
    }

    pub async fn sign_message(&self, message: &str) -> Result<Signature, crate::Error> {
        let req = KaikasRequest {
            method: "klay_sign".to_string(),
            params: vec![message.to_string()],
        };
        let req = serde_wasm_bindgen::to_value(&req).unwrap();
        web_sys::console::log_1(&req);
        let k = klaytn()?;
        let res = match JsFuture::from(k.request(&req)).await {
            Ok(res) => {
                web_sys::console::log_1(&res);
                res
            }
            Err(e) => {
                web_sys::console::log_1(&e);
                return Err(crate::Error::SignError);
            }
        };

        let sig: KaikasSignature = serde_wasm_bindgen::from_value(res).unwrap();

        Ok(Signature {
            r: U256::from_str_radix(&sig.r[2..], 16).expect("can't decode sig.r"),
            s: U256::from_str_radix(&sig.s, 16).expect("can't decode sig.s"),
            v: u64::from_str_radix(sig.v.trim_start_matches("0x"), 16).expect("can't decode sig.v"),
        })
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
    r#type: String,
    to: String,
    from: String,
    gas: String,
    gas_price: String,
    value: String,
    data: String,
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
        tracing::debug!("signing transaction: {tx:?}");
        use ethers::abi::AbiEncode;
        use ethers::types::U256;
        let to = tx.to.unwrap_or_default().encode();
        let to = format!("0x{}", hex::encode(&to[12..32]));
        let from = tx.from.unwrap_or_default().encode();
        let from = format!("0x{}", hex::encode(&from[12..32]));
        // NOTE: nonce in integer normally works. Sometime kaia wallet requires string but never accept.
        //       omitted nonce only works fine.
        //       But, concrete logic for this, tx argument should be mutable.
        //       Now it is negligible due to the nonce is gotten right before this.
        let req = KaikasRequest {
            method: "klay_signTransaction".to_string(),
            params: vec![Transaction {
                r#type: tx.tx_type.to_tx_type_string(),
                to,
                from,
                gas: tx.gas.unwrap_or_default().encode_hex(),
                gas_price: tx.gas_price.unwrap_or_default().encode_hex(),
                value: tx.value.unwrap_or_default().encode_hex(),
                data: format!("0x{}", hex::encode(tx.input.clone().unwrap_or_default())),
            }],
        };
        let req = serde_wasm_bindgen::to_value(&req).unwrap();
        web_sys::console::log_1(&req);
        let k = klaytn()?;
        let res = match JsFuture::from(k.request(&req)).await {
            Ok(res) => {
                web_sys::console::log_1(&res);
                res
            }
            Err(e) => {
                web_sys::console::log_1(&e);
                return Err(crate::Error::SignError);
            }
        };

        let sig: KaikasSignature = serde_wasm_bindgen::from_value(res).unwrap();

        Ok(Signature {
            r: U256::from_str_radix(&sig.r[2..], 16).expect("can't decode sig.r"),
            s: U256::from_str_radix(&sig.s, 16).expect("can't decode sig.s"),
            v: u64::from_str_radix(sig.v.trim_start_matches("0x"), 16).expect("can't decode sig.v"),
        })
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KaikasSignature {
    r: String,
    s: String,
    v: String,
}
