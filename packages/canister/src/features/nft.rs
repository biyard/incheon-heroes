use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;

use candid::{CandidType, Deserialize, Principal};
use dto::nft::{Event, Metadata, Nft};
use ic_cdk::{
    api::{self},
    query, update,
};
use ic_cdk::{caller, trap};
use serde_bytes::ByteBuf;

use crate::types::{Error, Result};
use crate::utils::backend_api::{BackendApi, ChangeShopItemRequest};
use crate::utils::klaytn_birdige_api::KlaytnBridgeApi;

use super::siwe_provider::service::get_address::get_address;

const MGMT: Principal = Principal::from_slice(&[]);

thread_local! {
    pub static STATE: RefCell<State> = RefCell::default();
}

#[derive(CandidType)]
pub struct MintResult {
    pub nft: Nft,
}

#[derive(CandidType)]
pub struct TransferResult {
    pub nft: Nft,
}

#[derive(CandidType)]
pub struct RemoveResult {
    pub user: Principal,
    pub token_id: u64,
}

#[derive(CandidType, Deserialize, Default)]
pub struct CustodianRequest {
    pub user: String,
    pub custodian: bool,
}

#[derive(CandidType, Deserialize, Default)]
pub struct RemoveNftParams {
    pub addr: String,
    pub token_id: u64,
}

macro_rules! add_balance {
    ($state:expr, $to:expr) => {
        match $state.holders.get_mut(&$to) {
            Some(holder) => {
                *holder = *holder + 1;
            }
            None => {
                $state.holders.insert($to, 1);
            }
        }
    };
}

macro_rules! add_user_nfts {
    ($state:expr, $to:expr, $nft:expr) => {
        match $state.user_nfts.get_mut(&$to) {
            Some(v) => {
                v.push($nft);
            }
            None => {
                $state.user_nfts.insert($to, vec![$nft]);
            }
        }
    };
}

macro_rules! add_token_histories {
    ($state:expr, $id:expr, $event:expr) => {
        match $state.token_history.get_mut(&$id) {
            Some(v) => {
                v.push($event);
            }
            None => {
                $state.token_history.insert($id, vec![$event]);
            }
        }
    };
}

macro_rules! remove_token_histories {
    ($state:expr, $id:expr, $event:expr) => {
        match $state.token_history.get_mut(&$id) {
            Some(v) => {
                let index = v.iter().position(|data| data.token_id == $id).unwrap();
                v.remove(index);
            }
            None => {
                trap("history is not exists");
            }
        }
    };
}

macro_rules! remove_user_nfts {
    ($state:expr, $to:expr, $nft:expr) => {
        match $state.user_nfts.get_mut(&$to) {
            Some(v) => {
                let index = v.iter().position(|data| data.id == $nft.id).unwrap();
                v.remove(index);
            }
            None => {
                trap("nft is not exists");
            }
        }
    };
}

macro_rules! sub_balance {
    ($state:expr, $from:expr) => {
        match $state.holders.get_mut(&$from) {
            Some(holder) => {
                if (*holder == 1) {
                    $state.holders.remove(&$from);
                } else {
                    *holder = *holder - 1;
                }
            }
            None => {
                assert!(false, "balance not found");
            }
        }
    };
}

// --------------
// base interface
// --------------

#[query]
fn balance_of(user: Principal) -> u64 {
    STATE.with(|state| {
        state
            .borrow()
            .nfts
            .iter()
            .filter(|n| n.owner == user)
            .count() as u64
    })
}

#[query]
fn holders_of() -> Vec<Principal> {
    STATE.with(|state| {
        let state = state.borrow();
        let holders = state.holders.keys().cloned().collect();
        holders
    })
}

#[query]
fn number_of_holders() -> u64 {
    STATE.with(|state| state.borrow().holders.len() as u64)
}

#[query]
pub fn owner_of(token_id: u64) -> Result<Principal> {
    STATE.with(|state| {
        let owner = state
            .borrow()
            .nfts
            .iter()
            .find(|n| n.id == token_id)
            .ok_or(Error::InvalidTokenId)?
            .owner;
        Ok(owner)
    })
}

#[update]
async fn safe_transfer_from(from: Principal, to: Principal, token_id: u64, rlp: String) -> Result {
    if to == MGMT {
        Error::ZeroAddress.err()
    } else {
        transfer_from(from, to, token_id, rlp).await
    }
}

#[update]
async fn transfer_from(from: Principal, to: Principal, token_id: u64, rlp: String) -> Result {
    match STATE.with(|state| {
        let mut state = state.borrow_mut();
        let state = &mut *state;
        let nft = state
            .nfts
            .get_mut(usize::try_from(token_id)?)
            .ok_or(Error::InvalidTokenId)?;
        let caller = caller();
        if nft.owner != caller
            && nft.approved != Some(caller)
            && !state
                .operators
                .get(&from)
                .map(|s| s.contains(&caller))
                .unwrap_or(false)
            && !state.custodians.contains(&caller)
        {
            Error::Unauthorized.err()
        } else if nft.owner != from {
            Error::Other.err()
        } else {
            nft.approved = None;
            sub_balance!(state, nft.owner);
            nft.owner = to;
            add_balance!(state, to);
            Ok(state.next_txid())
        }
    }) {
        Ok(txid) => match KlaytnBridgeApi::new().send_raw_transaction(rlp).await {
            Ok(_) => Ok(txid),
            Err(_) => Error::Other.err(),
        },
        Err(err) => Err(err),
    }
}

#[query]
fn supported_interfaces() -> &'static [InterfaceId] {
    &[
        InterfaceId::TransferNotification,
        // InterfaceId::Approval, // Psychedelic/DIP721#5
        InterfaceId::Burn,
        InterfaceId::Mint,
    ]
}

#[query]
fn name() -> String {
    STATE.with(|state| state.borrow().name.clone())
}

#[query]
fn symbol() -> String {
    STATE.with(|state| state.borrow().symbol.clone())
}

#[query]
fn total_supply() -> u64 {
    STATE.with(|state| state.borrow().nfts.len() as u64)
}
// ----------------------
// notification interface
// ----------------------

// #[update]
// async fn transfer_from_notify(
//     from: Principal,
//     to: Principal,
//     token_id: u64,
//     data: Vec<u8>,
// ) -> Result {
//     let res = transfer_from(from, to, token_id).await?;
//     if let Ok(arg) = Encode!(&api::caller(), &from, &token_id, &data) {
//         // Using call_raw ensures we don't need to await the future for the call to be executed.
//         // Calling an arbitrary function like this means that a malicious recipient could call
//         // transferFromNotify in their onDIP721Received function, resulting in an infinite loop.
//         // This will trap eventually, but the transfer will have already been completed and the state-change persisted.
//         // That means the original transfer must reply before that happens, or the caller will be
//         // convinced that the transfer failed when it actually succeeded. So we don't await the call,
//         // so that we'll reply immediately regardless of how long the notification call takes.
//         let _ = api::call::call_raw(to, "onDIP721Received", arg, 0);
//     }
//     Ok(res)
// }

// #[update]
// async fn safe_transfer_from_notify(
//     from: Principal,
//     to: Principal,
//     token_id: u64,
//     data: Vec<u8>,
// ) -> Result {
//     if to == MGMT {
//         Err(Error::ZeroAddress)
//     } else {
//         transfer_from_notify(from, to, token_id, data).await
//     }
// }

// ------------------
// approval interface
// ------------------

#[update]
fn approve(user: Principal, token_id: u64) -> Result {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let state = &mut *state;
        let caller = api::caller();
        let nft = state
            .nfts
            .get_mut(usize::try_from(token_id)?)
            .ok_or(Error::InvalidTokenId)?;
        if nft.owner != caller
            && nft.approved != Some(caller)
            && !state
                .operators
                .get(&user)
                .map(|s| s.contains(&caller))
                .unwrap_or(false)
            && !state.custodians.contains(&caller)
        {
            Error::Unauthorized.err()
        } else {
            nft.approved = Some(user);
            Ok(state.next_txid())
        }
    })
}

#[update]
fn set_approval_for_all(operator: Principal, is_approved: bool) -> Result {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let caller = api::caller();
        if operator != caller {
            let operators = state.operators.entry(caller).or_default();
            if operator == MGMT {
                if !is_approved {
                    operators.clear();
                } else {
                    // cannot enable everyone as an operator
                }
            } else {
                if is_approved {
                    operators.insert(operator);
                } else {
                    operators.remove(&operator);
                }
            }
        }
        Ok(state.next_txid())
    })
}

// #[query] // Psychedelic/DIP721#5
fn _get_approved(token_id: u64) -> Result<Principal> {
    STATE.with(|state| {
        let approved = state
            .borrow()
            .nfts
            .get(usize::try_from(token_id)?)
            .ok_or(Error::InvalidTokenId)?
            .approved
            .unwrap_or_else(api::caller);
        Ok(approved)
    })
}

#[query]
fn is_approved_for_all(operator: Principal) -> bool {
    STATE.with(|state| {
        state
            .borrow()
            .operators
            .get(&api::caller())
            .map(|s| s.contains(&operator))
            .unwrap_or(false)
    })
}

// --------------
// mint interface
// --------------

#[query]
fn metadata_of(token_id: u64) -> Metadata {
    STATE.with(|state| {
        state
            .borrow()
            .nfts
            .get(usize::try_from(token_id).unwrap())
            .unwrap()
            .metadata
            .clone()
    })
}

#[query]
fn list_tokens_by(addr: Principal) -> Vec<Nft> {
    STATE.with(|state| {
        state
            .borrow_mut()
            .user_nfts
            .entry(addr)
            .or_default()
            .clone()
    })
}

#[query]
fn history_of(token_id: u64) -> Vec<Event> {
    STATE.with(|state| {
        state
            .borrow_mut()
            .token_history
            .entry(token_id)
            .or_default()
            .clone()
    })
}

#[update]
async fn transfer_from_by_canister(from: Principal, to: Principal, token_id: u64) -> Result<Nft> {
    match STATE.with(|state| {
        let mut state = state.borrow_mut();
        let state = &mut *state;
        // let nft = state
        //     .nfts
        //     .get_mut(usize::try_from(token_id)?)
        //     .ok_or(Error::InvalidTokenId)?;

        let nft_lists = state.nfts.clone();

        let index = nft_lists
            .iter()
            .position(|data| data.id == token_id)
            .unwrap();
        let mut nft = nft_lists[index].clone();
        let caller = caller();
        if nft.owner != caller
            && nft.approved != Some(caller)
            && !state
                .operators
                .get(&from)
                .map(|s| s.contains(&caller))
                .unwrap_or(false)
            && !state.custodians.contains(&caller)
        {
            Error::Unauthorized.err()
        } else if nft.owner != from {
            Error::Other.err()
        } else {
            nft.approved = None;
            sub_balance!(state, nft.owner);
            nft.owner = to;
            add_balance!(state, to);
            remove_user_nfts!(state, from, nft.clone());
            add_user_nfts!(state, to, nft.clone());
            state.nfts[index] = nft.clone();

            let event = Event {
                token_id: nft.id,
                event_name: "Transfer".to_string(),
                from,
                to,
            };

            state.histories.push(event.clone());

            add_token_histories!(state, nft.id, event);
            Ok(nft.clone())
        }
    }) {
        Ok(nft) => Ok(nft.clone()),
        Err(err) => Err(err),
    }

    // let nft = match STATE.with(|state| {
    //     let mut state = state.borrow_mut();
    //     let state = &mut *state;
    //     let nft = state
    //         .nfts
    //         .get_mut(usize::try_from(token_id)?)
    //         .ok_or(Error::InvalidTokenId)?;
    //         // .get_mut(usize::try_from(token_id)).ok_or(Error::InvalidTokenId)?;
    //     let caller = caller();
    //     if nft.owner != caller
    //         && nft.approved != Some(caller)
    //         && !state
    //             .operators
    //             .get(&from)
    //             .map(|s| s.contains(&caller))
    //             .unwrap_or(false)
    //         && !state.custodians.contains(&caller)
    //     {
    //         panic!("unauthorized");
    //     } else if nft.owner != from {
    //         panic!("panic other reason");
    //     } else {
    //         nft.approved = None;
    //         sub_balance!(state, nft.owner);
    //         remove_user_nfts!(state, from, nft.clone());
    //         nft.owner = to;
    //         add_balance!(state, to);
    //         add_user_nfts!(state, to, nft.clone());
    //     }

    //     nft
    // });

    // Ok(nft.clone())
}

#[update]
fn remove_nft(data: RemoveNftParams) -> Result<RemoveResult> {
    let addr = data.addr.clone();
    let to = Principal::from_text(addr).unwrap();
    let token_id = data.token_id;
    let caller = caller();
    match STATE.with(|state| {
        let mut state = state.borrow_mut();
        if !state.custodians.contains(&caller) {
            Error::Unauthorized.err()
        } else {
            let index = state
                .nfts
                .iter()
                .position(|data| data.id == token_id)
                .unwrap();

            let nft = state.nfts[index].clone();

            state.nfts.remove(index);

            let index = state
                .histories
                .iter()
                .position(|data| data.token_id == token_id)
                .unwrap();

            state.histories.remove(index);

            sub_balance!(state, to);
            remove_user_nfts!(state, to, nft.clone());
            remove_token_histories!(state, token_id, event);
            Ok(to)
        }
    }) {
        Ok(to) => Ok(RemoveResult { user: to, token_id }),
        Err(err) => Err(err),
    }
}

#[update]
async fn buy_free_item() -> MintResult {
    let to = api::caller();
    let data = match BackendApi::new().get_shop_data().await {
        Ok(d) => d,
        Err(_) => trap("shop data query failed"),
    };
    let metadata = BackendApi::new().metadata(data.item.token_id).await;

    let nft = STATE.with(|state| {
        let mut state = state.borrow_mut();

        let check = state.check_buy.get_mut(&to);

        match check {
            Some(_) => {
                trap("already public minting user");
            }
            None => {
                state.check_buy.insert(to, true);
            }
        }

        let nft = Nft {
            owner: to,
            approved: None,
            id: data.item.token_id,
            metadata,
        };
        state.nfts.push(nft.clone());

        let event = Event {
            token_id: data.item.token_id,
            event_name: "Mint".to_string(),
            from: MGMT,
            to,
        };
        state.histories.push(event.clone());

        add_balance!(state, to);
        add_user_nfts!(state, to, nft.clone());
        add_token_histories!(state, data.item.token_id, event);
        nft
    });

    let shop_item = ChangeShopItemRequest {
        item_id: data.item.id,
        price: data.item.price,
        token_id: data.item.token_id + 1,
        name: data.item.name,
        image: data.item.image,
        supply: data.item.supply,
        likes: data.item.likes,
        reports: data.item.reports,
        contract_address: data.item.contract_address,
        creator: data.item.creator,
        remaining: data.item.remaining - 1,
        level: data.item.level,
        metadata: data.item.metadata,
    };

    match BackendApi::new().change_shop_item(shop_item).await {
        Ok(_) => {}
        Err(_) => {
            trap("shop data update failed");
        }
    }

    MintResult { nft }
}

#[update]
async fn buy_item(to: Principal) -> MintResult {
    let data = match BackendApi::new().get_shop_data().await {
        Ok(d) => d,
        Err(_) => trap("shop data query failed"),
    };
    let metadata = BackendApi::new().metadata(data.item.token_id).await;

    let nft = STATE.with(|state| {
        let mut state = state.borrow_mut();

        let check = state.check_buy.get_mut(&to);

        match check {
            Some(_) => {
                trap("already public minting user");
            }
            None => {
                state.check_buy.insert(to, true);
            }
        }

        let nft = Nft {
            owner: to,
            approved: None,
            id: data.item.token_id,
            metadata,
        };
        state.nfts.push(nft.clone());

        let event = Event {
            token_id: data.item.token_id,
            event_name: "Mint".to_string(),
            from: MGMT,
            to,
        };
        state.histories.push(event.clone());

        add_balance!(state, to);
        add_user_nfts!(state, to, nft.clone());
        add_token_histories!(state, data.item.token_id, event);
        nft
    });

    let shop_item = ChangeShopItemRequest {
        item_id: data.item.id,
        price: data.item.price,
        token_id: data.item.token_id + 1,
        name: data.item.name,
        image: data.item.image,
        supply: data.item.supply,
        likes: data.item.likes,
        reports: data.item.reports,
        contract_address: data.item.contract_address,
        creator: data.item.creator,
        remaining: data.item.remaining - 1,
        level: data.item.level,
        metadata: data.item.metadata,
    };

    match BackendApi::new().change_shop_item(shop_item).await {
        Ok(_) => {}
        Err(_) => {
            trap("shop data update failed");
        }
    }

    MintResult { nft }
}

#[update]
async fn mint(to: Principal, metadata: Metadata) -> MintResult {
    let nft = STATE.with(|state| {
        let mut state = state.borrow_mut();
        if !state.custodians.contains(&api::caller()) {
            trap("unauthorized account");
        }
        let new_id = state.nfts.len() as u64;
        let nft = Nft {
            owner: to,
            approved: None,
            id: new_id,
            metadata,
        };
        state.nfts.push(nft.clone());

        add_balance!(state, to);
        nft
    });

    // http::add_hash(nft.id);
    MintResult { nft }
}

// --------------
// burn interface
// --------------

#[update]
async fn burn(token_id: u64) -> Result {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let nft = state
            .nfts
            .get_mut(usize::try_from(token_id)?)
            .ok_or(Error::InvalidTokenId)?;

        if nft.owner != api::caller() {
            Error::Unauthorized.err()
        } else {
            let prev = nft.owner.clone();
            nft.owner = MGMT;

            sub_balance!(state, prev);
            Ok(state.next_txid())
        }
    })
}

#[derive(CandidType, Deserialize, Default)]
pub struct State {
    pub nfts: Vec<Nft>,
    pub custodians: HashSet<Principal>,
    pub operators: HashMap<Principal, HashSet<Principal>>,
    pub holders: HashMap<Principal, u64>,
    pub user_nfts: HashMap<Principal, Vec<Nft>>,
    pub check_buy: HashMap<Principal, bool>,
    pub token_history: HashMap<u64, Vec<Event>>,
    pub histories: Vec<Event>,
    pub name: String,
    pub symbol: String,
    pub txid: u128,
    pub endpoint: String,
}

impl State {
    fn next_txid(&mut self) -> u128 {
        let txid = self.txid;
        self.txid += 1;
        txid
    }
}

#[derive(CandidType, Deserialize)]
pub enum InterfaceId {
    Approval,
    TransactionHistory,
    Mint,
    Burn,
    TransferNotification,
}

#[update]
fn set_name(name: String) -> Result<()> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if state.custodians.contains(&api::caller()) {
            state.name = name;
            Ok(())
        } else {
            Error::Unauthorized.err()
        }
    })
}

#[update]
fn set_symbol(sym: String) -> Result<()> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if state.custodians.contains(&api::caller()) {
            state.symbol = sym;
            Ok(())
        } else {
            Error::Unauthorized.err()
        }
    })
}

#[update]
fn set_custodian(data: CustodianRequest) -> Result<()> {
    let user = Principal::from_text(data.user).unwrap();
    let custodian = data.custodian;
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if state.custodians.contains(&api::caller()) {
            if custodian {
                state.custodians.insert(user);
            } else {
                state.custodians.remove(&user);
            }
            Ok(())
        } else {
            Error::Unauthorized.err()
        }
    })
}

#[query]
fn is_custodian(principal: Principal) -> bool {
    STATE.with(|state| state.borrow().custodians.contains(&principal))
}

#[query]
fn list_custodians() -> Vec<Principal> {
    STATE.with(|state| state.borrow().custodians.clone().into_iter().collect())
}

#[query]
fn list_histories(principal: Principal) -> Vec<Event> {
    STATE.with(|state| {
        let state = state.borrow();
        state
            .histories
            .clone()
            .into_iter()
            .filter(|history| (history.from == principal) || (history.to == principal))
            .collect()
    })
}

#[query]
fn list_nfts(page: usize, size: usize) -> Vec<Nft> {
    STATE.with(|state| {
        let state = state.borrow();
        state
            .nfts
            .iter()
            .skip(page * size)
            .take(size)
            .cloned()
            .collect()
    })
}

//TODO: implement list_histories_from(startAt: usize) function

#[query]
fn get_nft(token_id: u64) -> Nft {
    STATE.with(|state| {
        state
            .borrow()
            .nfts
            .get(usize::try_from(token_id).unwrap())
            .unwrap()
            .clone()
    })
}

#[update]
async fn bridge(id: u64, addr: String, signature: String) -> Nft {
    let address = match get_address(ByteBuf::from(ic_cdk::caller().as_slice().to_vec())) {
        Ok(addr) => addr,
        Err(_) => {
            trap("you must register EVM address in advance");
            panic!("you must register EVM address in advance")
        }
    };
    let api = BackendApi::new();
    if !api.check_owner_of(id, address).await {
        trap("you are not the owner of the NFT");
    }

    api.bridge_nft(id, addr, signature);

    let metadata = api.metadata(id).await;

    let nft = STATE.with(|state| {
        let mut state = state.borrow_mut();

        let nft_lists = state.nfts.clone();

        match nft_lists.iter().position(|data| data.id == id) {
            Some(_index) => panic!("already swapped"),
            None => {}
        };

        let nft = Nft {
            owner: caller(),
            approved: None,
            id,
            metadata,
        };
        state.nfts.push(nft.clone());

        let event = Event {
            token_id: id,
            event_name: "Mint".to_string(),
            from: MGMT,
            to: caller(),
        };
        state.histories.push(event.clone());

        add_balance!(state, caller());
        add_user_nfts!(state, caller(), nft.clone());
        add_token_histories!(state, id, event);
        nft
    });

    nft
}

#[update]
async fn get_metadata(id: u64) -> Metadata {
    BackendApi::new().metadata(id).await
}
