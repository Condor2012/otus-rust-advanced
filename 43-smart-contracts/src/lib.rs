use candid::{CandidType, Nat, Principal};
use ic_cdk::{init, query, update};
use serde::Deserialize;

use crate::storage::IcStorage;
use crate::wallet::Wallets;

mod storage;
mod wallet;

#[derive(Debug, CandidType, Deserialize)]
pub enum TokenError {
    InsufficientBalance,
    NotAuthorized,
}

fn balances() -> Wallets<IcStorage> {
    Wallets::new(IcStorage::default())
}

#[init]
pub fn init(init_balance: Nat) {
    let owner = ic_cdk::api::msg_caller();
    IcStorage::default().set_owner(owner);
    balances().mint(owner, init_balance);
}

#[update]
pub fn mint(to: Principal, value: Nat) -> Result<Nat, TokenError> {
    let caller = ic_cdk::api::msg_caller();
    let caller_is_owner = IcStorage::default().check_owner(caller);

    if !caller_is_owner {
        return Err(TokenError::NotAuthorized);
    }

    balances().mint(to, value.clone());

    Ok(value)
}

#[update]
pub fn transfer(to: Principal, value: Nat) -> Result<Nat, TokenError> {
    let caller = ic_cdk::api::msg_caller();

    let transfered = balances().transfer(caller, to, value);
    let amount = transfered.ok_or(TokenError::InsufficientBalance)?;

    Ok(amount)
}

#[query]
pub fn balance_of(user: Option<Principal>) -> Nat {
    let caller = ic_cdk::api::msg_caller();
    balances().balance_of(user.unwrap_or(caller))
}

ic_cdk::export_candid!();
