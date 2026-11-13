use std::collections::BTreeMap;

use num::{CheckedAdd, One, Zero};

pub trait Config {
    type AccountId: Clone + Ord + ToString;
    type BlockNumber: Copy + Zero + One + CheckedAdd;
    type Nonce: Copy + Zero + One + CheckedAdd;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number = self
            .block_number
            .checked_add(&T::BlockNumber::one())
            .unwrap_or(self.block_number);
    }

    pub fn inc_nonce(&mut self, account: T::AccountId) {
        let new_nonce = self
            .get_nonce(&account)
            // .unwrap_or(&T::Nonce::zero())
            .checked_add(&T::Nonce::one())
            .unwrap();

        self.nonce.insert(account, new_nonce);
    }

    pub fn get_nonce(&self, account: &T::AccountId) -> T::Nonce {
        *self.nonce.get(account).unwrap_or(&T::Nonce::zero())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct SystemConfig;

    impl super::Config for SystemConfig {
        type AccountId = String;
        type BlockNumber = u64;
        type Nonce = u64;
    }

    #[test]
    fn init_system() {
        let system: Pallet<SystemConfig> = super::Pallet::new();

        assert_eq!(system.block_number(), 0);
    }

    #[test]
    fn inc_block_number() {
        let mut system: Pallet<SystemConfig> = super::Pallet::new();
        system.inc_block_number();
        assert_eq!(system.block_number(), 1);
        system.inc_block_number();
        assert_eq!(system.block_number(), 2);
        system.inc_block_number();
        assert_eq!(system.block_number(), 3);
    }

    #[test]
    fn inc_nonce() {
        let mut system: Pallet<SystemConfig> = super::Pallet::new();
        system.inc_nonce("Alice".to_string());
        assert_eq!(system.get_nonce(&"Alice".to_string()), 1);
        system.inc_nonce("Alice".to_string());
        assert_eq!(system.get_nonce(&"Alice".to_string()), 2);
    }
}
