// use abi::AbiEncode;
// use ethers::prelude::*;
// use hex;
// use rlp::{RlpEncoder, RlpStream};
// use std::convert::TryFrom;

// #[derive(Debug, Clone, Copy, PartialEq)]
// enum TransactionType {
//     Legacy,
//     ValueTransfer,
//     FeeDelegatedValueTransfer,
//     ValueTransferMemo,
//     FeeDelegatedValueTransferMemo,
//     SmartContractDeploy,
//     FeeDelegatedSmartContractDeploy,
//     SmartContractExecution,
//     FeeDelegatedSmartContractExecution,
//     AccountUpdate,
//     FeeDelegatedAccountUpdate,
//     Cancel,
//     FeeDelegatedCancel,
// }

// impl TransactionType {
//     fn to_tx_type_string(&self) -> String {
//         match self {
//             TransactionType::Legacy => "LEGACY".to_string(),
//             TransactionType::ValueTransfer => "VALUE_TRANSFER".to_string(),
//             TransactionType::FeeDelegatedValueTransfer => {
//                 "FEE_DELEGATED_VALUE_TRANSFER".to_string()
//             }
//             TransactionType::ValueTransferMemo => "VALUE_TRANSFER_MEMO".to_string(),
//             TransactionType::FeeDelegatedValueTransferMemo => {
//                 "FEE_DELEGATED_VALUE_TRANSFER_MEMO".to_string()
//             }
//             TransactionType::SmartContractDeploy => "SMART_CONTRACT_DEPLOY".to_string(),
//             TransactionType::FeeDelegatedSmartContractDeploy => {
//                 "FEE_DELEGATED_SMART_CONTRACT_DEPLOY".to_string()
//             }
//             TransactionType::SmartContractExecution => "SMART_CONTRACT_EXECUTION".to_string(),
//             TransactionType::FeeDelegatedSmartContractExecution => {
//                 "FEE_DELEGATED_SMART_CONTRACT_EXECUTION".to_string()
//             }
//             TransactionType::AccountUpdate => "ACCOUNT_UPDATE".to_string(),
//             TransactionType::FeeDelegatedAccountUpdate => {
//                 "FEE_DELEGATED_ACCOUNT_UPDATE".to_string()
//             }
//             TransactionType::Cancel => "CANCEL".to_string(),
//             TransactionType::FeeDelegatedCancel => "FEE_DELEGATED_CANCEL".to_string(),
//         }
//     }

//     fn to_tx_type_code(&self) -> u8 {
//         match self {
//             TransactionType::Legacy => 0x0,
//             TransactionType::ValueTransfer => 0x8,
//             TransactionType::FeeDelegatedValueTransfer => 0x9,
//             TransactionType::ValueTransferMemo => 0x10,
//             TransactionType::FeeDelegatedValueTransferMemo => 0x11,
//             TransactionType::AccountUpdate => 0x20,
//             TransactionType::FeeDelegatedAccountUpdate => 0x21,
//             TransactionType::SmartContractDeploy => 0x28,
//             TransactionType::FeeDelegatedSmartContractDeploy => 0x29,
//             TransactionType::SmartContractExecution => 0x30,
//             TransactionType::FeeDelegatedSmartContractExecution => 0x31,
//             TransactionType::Cancel => 0x38,
//             TransactionType::FeeDelegatedCancel => 0x39,
//         }
//     }
// }

// impl TryFrom<&str> for TransactionType {
//     type Error = String;

//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         match value {
//             "VALUE_TRANSFER" => Ok(TransactionType::ValueTransfer),
//             "FEE_DELEGATED_VALUE_TRANSFER" => Ok(TransactionType::FeeDelegatedValueTransfer),
//             "VALUE_TRANSFER_MEMO" => Ok(TransactionType::ValueTransferMemo),
//             "FEE_DELEGATED_VALUE_TRANSFER_MEMO" => {
//                 Ok(TransactionType::FeeDelegatedValueTransferMemo)
//             }
//             "SMART_CONTRACT_DEPLOY" => Ok(TransactionType::SmartContractDeploy),
//             "FEE_DELEGATED_SMART_CONTRACT_DEPLOY" => {
//                 Ok(TransactionType::FeeDelegatedSmartContractDeploy)
//             }
//             "SMART_CONTRACT_EXECUTION" => Ok(TransactionType::SmartContractExecution),
//             "FEE_DELEGATED_SMART_CONTRACT_EXECUTION" => {
//                 Ok(TransactionType::FeeDelegatedSmartContractExecution)
//             }
//             "ACCOUNT_UPDATE" => Ok(TransactionType::AccountUpdate),
//             "FEE_DELEGATED_ACCOUNT_UPDATE" => Ok(TransactionType::FeeDelegatedAccountUpdate),
//             "CANCEL" => Ok(TransactionType::Cancel),
//             "FEE_DELEGATED_CANCEL" => Ok(TransactionType::FeeDelegatedCancel),
//             "LEGACY" => Ok(TransactionType::Legacy),
//             _ => Err(format!("Unknown transaction type: {}", value)),
//         }
//     }
// }

// #[derive(Debug, Clone)]
// struct MsgSignature {
//     v: u64,
//     r: U256,
//     s: U256,
// }

// #[derive(Debug, Clone)]
// struct KlaytnTransaction {
//     tx_type: TransactionType,
//     nonce: Option<U256>,
//     gas_price: Option<U256>,
//     gas: Option<U256>,
//     from: Option<Address>,
//     to: Option<Address>,
//     value: Option<U256>,
//     fee_payer: Option<Address>,
//     input: Option<Vec<u8>>,
//     signature: Option<MsgSignature>,
// }

// impl KlaytnTransaction {
//     fn new(
//         tx_type: TransactionType,
//         from: Option<Address>,
//         to: Option<Address>,
//         fee_payer: Option<Address>,
//         gas: Option<U256>,
//         gas_price: Option<U256>,
//         value: Option<U256>,
//         input: Option<Vec<u8>>,
//         nonce: Option<U256>,
//         signature: Option<MsgSignature>,
//     ) -> Self {
//         KlaytnTransaction {
//             tx_type,
//             nonce,
//             gas_price,
//             gas,
//             from,
//             to,
//             value,
//             fee_payer,
//             input,
//             signature,
//         }
//     }

//     fn value_with_default_zero(&self) -> U256 {
//         self.value.unwrap_or(U256::from(0))
//     }

//     fn to_tx_hash_rlp(&self, chain_id: u64) -> Vec<u8> {
//         let mut rlp = RlpStream::new();
//         match self.tx_type {
//             TransactionType::ValueTransfer => {
//                 rlp.begin_list(8);
//                 rlp.append(&self.tx_type.to_tx_type_code());
//                 rlp_opt(&mut rlp, &self.nonce);
//                 rlp_opt(&mut rlp, &self.gas_price);
//                 rlp_opt(&mut rlp, &self.gas);
//                 rlp_opt(&mut rlp, &self.to);
//                 rlp_opt(&mut rlp, &self.value);
//                 rlp_opt(&mut rlp, &self.from);

//                 rlp_sig(&mut rlp, &self.signature);
//             }
//             TransactionType::FeeDelegatedSmartContractExecution => {
//                 rlp.begin_list(10);
//                 rlp.append(&self.tx_type.to_tx_type_code());
//                 rlp_opt(&mut rlp, &self.nonce);
//                 rlp_opt(&mut rlp, &self.gas_price);
//                 rlp_opt(&mut rlp, &self.gas);
//                 rlp_opt(&mut rlp, &self.to);
//                 rlp_opt(&mut rlp, &self.value);
//                 rlp_opt(&mut rlp, &self.from);
//                 rlp_opt(&mut rlp, &self.input);

//                 rlp_sig(&mut rlp, &self.signature);

//                 rlp.append(&self.fee_payer.unwrap_or(Address::zero()));

//                 rlp.append(
//                     &RlpStream::new_list(1)
//                         .append(
//                             &RlpStream::new_list(3)
//                                 .append(&U64::from(0))
//                                 .append(&vec![0u8; 32])
//                                 .append(&vec![0u8; 32])
//                                 .out(),
//                         )
//                         .out(),
//                 );
//             }
//             _ => unimplemented!("unsupported type"),
//         }
//         rlp.out().to_vec()
//     }

//     fn to_sig_rlp(&self, chain_id: u64) -> Vec<u8> {
//         let mut rlp = RlpStream::new();
//         match self.tx_type {
//             TransactionType::ValueTransfer => {
//                 rlp.begin_list(2);
//                 rlp.begin_list(7);
//                 rlp.append(&self.tx_type.to_tx_type_code());

//                 rlp_opt(&mut rlp, &self.nonce);
//                 rlp_opt(&mut rlp, &self.gas_price);
//                 rlp_opt(&mut rlp, &self.gas);
//                 rlp_opt(&mut rlp, &self.to);
//                 rlp_opt(&mut rlp, &self.value);
//                 rlp_opt(&mut rlp, &self.from);
//                 rlp_opt(&mut rlp, &self.input);

//                 rlp.append(&chain_id);
//                 rlp.append(&0u8);
//                 rlp.append(&0u8);
//             }
//             TransactionType::FeeDelegatedSmartContractExecution => {
//                 rlp.begin_list(2);
//                 rlp.begin_list(8);

//                 rlp.append(&self.tx_type.to_tx_type_code());
//                 rlp_opt(&mut rlp, &self.nonce);
//                 rlp_opt(&mut rlp, &self.gas_price);
//                 rlp_opt(&mut rlp, &self.gas);
//                 rlp_opt(&mut rlp, &self.to);
//                 rlp_opt(&mut rlp, &self.value);
//                 rlp_opt(&mut rlp, &self.from);
//                 rlp_opt(&mut rlp, &self.input);

//                 rlp.append(&chain_id);
//                 rlp.append(&0u8);
//                 rlp.append(&0u8);
//             }
//             _ => unimplemented!("unsupported type"),
//         }
//         rlp.out().to_vec()
//     }

//     async fn sign(
//         &mut self,
//         wallet: &LocalWallet,
//         chain_id: u64,
//     ) -> Result<(), Box<dyn std::error::Error>> {
//         let sig_rlp = self.to_sig_rlp(chain_id);
//         let hash = ethers::utils::keccak256(&sig_rlp);
//         let signature = wallet.sign_hash(&hash.into()).await?;

//         self.signature = Some(MsgSignature {
//             v: signature.v as u64,
//             r: signature.r,
//             s: signature.s,
//         });
//         Ok(())
//     }
// }

// pub fn rlp_opt<T: rlp::Encodable>(rlp: &mut rlp::RlpStream, opt: &Option<T>) {
//     if let Some(inner) = opt {
//         rlp.append(inner);
//     } else {
//         rlp.append(&"");
//     }
// }

// pub fn rlp_sig(rlp: &mut rlp::RlpStream, signature: &Option<MsgSignature>) {
//     if let Some(sig) = signature {
//         let mut sig_rlp = RlpStream::new_list(3);
//         sig_rlp.append(&sig.v);
//         sig_rlp.append(&sig.r);
//         sig_rlp.append(&sig.s);
//         let sig_rlp = sig_rlp.out();

//         let mut sig = RlpStream::new_list(1);
//         sig.append(&sig_rlp);

//         rlp.append(&sig.out());
//     } else {
//         rlp.append(&vec![]);
//     }
// }

// pub fn rlp_empty_sig(rlp: &mut rlp::RlpStream) {
//     if let Some(sig) = signature {
//         let mut sig_rlp = RlpStream::new_list(3);
//         sig_rlp.append(&sig.v);
//         sig_rlp.append(&sig.r);
//         sig_rlp.append(&sig.s);
//         let sig_rlp = sig_rlp.out();

//         let mut sig = RlpStream::new_list(1);
//         sig.append(&sig_rlp);

//         rlp.append(&sig.out());
//     } else {
//         rlp.append(&vec![]);
//     }
// }
