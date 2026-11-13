use std::collections::BTreeMap;

use num::{CheckedAdd, CheckedSub, Zero};

pub trait Config: crate::system::Config {
    type Balance: Copy + Zero + CheckedAdd + CheckedSub + PartialOrd;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn get_balance(&self, account: &T::AccountId) -> T::Balance {
        *self.balances.get(account).unwrap_or(&T::Balance::zero())
    }

    pub fn set_balance(&mut self, account: &T::AccountId, balance: T::Balance) {
        self.balances.insert(account.clone(), balance);
    }

    pub fn transfer(
        &mut self,
        caller: &T::AccountId,
        to: &T::AccountId,
        amount: T::Balance,
    ) -> Result<(), &'static str> {
        let caller_balance: T::Balance = self.get_balance(caller);
        let recepient_balance: T::Balance = self.get_balance(to);

        if caller_balance < amount {
            return Err("Not enough balance");
        }

        let new_caller_balance = caller_balance
            .checked_sub(&amount)
            .ok_or("Not enough balance")?;
        let new_recepient_balance = recepient_balance.checked_add(&amount).ok_or("Overflow")?;

        self.set_balance(caller, new_caller_balance);
        self.set_balance(to, new_recepient_balance);
        Ok(())
    }
}

pub enum Call<T: Config> {
    Transfer {
        to: T::AccountId,
        amount: T::Balance,
    },
    // Topup {
    //     to: T::AccountId,
    //     amount: T::Balance,
    // },
}

impl<T: Config> crate::support::Dispatch for Pallet<T> {
    type Caller = T::AccountId;
    type Call = Call<T>;

    fn dispatch(
        &mut self,
        caller: Self::Caller,
        call: Self::Call,
    ) -> crate::support::DispatchResult {
        match call {
            Call::Transfer { to, amount } => {
                self.transfer(&caller, &to, amount)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct BalanceConfig;

    impl crate::system::Config for BalanceConfig {
        type AccountId = String;
        type BlockNumber = u64;
        type Nonce = u64;
    }

    impl super::Config for BalanceConfig {
        type Balance = u128;
    }

    #[test]
    fn test_get_balance() {
        let mut pallet: Pallet<BalanceConfig> = Pallet::new();
        pallet.set_balance(&"Alice".to_string(), 100);
        assert_eq!(pallet.get_balance(&"Alice".to_string()), 100);
    }

    #[test]
    fn test_transfer() {
        let mut pallet: Pallet<BalanceConfig> = Pallet::new();
        pallet.set_balance(&"Alice".to_string(), 100);
        pallet.set_balance(&"Bob".to_string(), 0);

        pallet
            .transfer(&"Alice".to_string(), &"Bob".to_string(), 50)
            .unwrap();

        assert_eq!(pallet.get_balance(&"Alice".to_string()), 50);
        assert_eq!(pallet.get_balance(&"Bob".to_string()), 50);
    }

    #[test]
    fn test_overflow() {
        let mut pallet: Pallet<BalanceConfig> = Pallet::new();
        pallet.set_balance(&"Alice".to_string(), 100);
        pallet.set_balance(&"Bob".to_string(), u128::MAX);

        let result = pallet.transfer(&"Alice".to_string(), &"Bob".to_string(), 50);

        assert_eq!(result, Err("Overflow"));
    }

    #[test]
    fn test_not_enough_balance() {
        let mut pallet: Pallet<BalanceConfig> = Pallet::new();
        pallet.set_balance(&"Alice".to_string(), 100);
        pallet.set_balance(&"Bob".to_string(), 0);

        let result = pallet.transfer(&"Alice".to_string(), &"Bob".to_string(), 200);

        assert_eq!(result, Err("Not enough balance"));
    }
}
