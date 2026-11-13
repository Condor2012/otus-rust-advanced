use std::cell::RefCell;
use std::collections::HashMap;

use candid::{Nat, Principal};

use crate::wallet::BalanceStorage;

thread_local! {
    static BALANCES: RefCell<HashMap<Principal, Nat>> = RefCell::new(HashMap::new());
    static OWNER: RefCell<Principal> = RefCell::new(Principal::anonymous());
}

#[derive(Default)]
pub struct IcStorage {}

impl IcStorage {
    pub fn set_owner(&mut self, user: Principal) {
        OWNER.with(|s| *s.borrow_mut() = user)
    }

    pub fn get_owner(&self) -> Principal {
        OWNER.with(|s| *s.borrow())
    }

    pub fn check_owner(&self, user: Principal) -> bool {
        user == self.get_owner()
    }
}

impl BalanceStorage for IcStorage {
    type Id = Principal;

    type Value = Nat;

    fn credit(&mut self, to: Self::Id, amount: Self::Value) {
        BALANCES.with(|map| {
            let mut map = map.borrow_mut();
            *map.entry(to).or_default() += amount;
        })
    }

    fn debit(&mut self, from: Self::Id, amount: Self::Value) -> Option<()> {
        BALANCES.with(|map| {
            let mut map = map.borrow_mut();
            let balance = map.entry(from).or_default();

            if *balance < amount {
                return None;
            }

            *balance -= amount;
            Some(())
        })
    }

    fn balance_of(&self, user: Self::Id) -> Self::Value {
        BALANCES.with(|map| map.borrow().get(&user).cloned().unwrap_or_default())
    }
}
