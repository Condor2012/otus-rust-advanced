pub trait BalanceStorage {
    type Id;
    type Value: Clone;

    fn credit(&mut self, to: Self::Id, amount: Self::Value);
    fn debit(&mut self, from: Self::Id, amount: Self::Value) -> Option<()>;
    fn balance_of(&self, user: Self::Id) -> Self::Value;
}

pub struct Wallets<S> {
    storage: S
}

impl<S: BalanceStorage> Wallets<S> {
    pub fn new(storage: S) -> Self {
        Self { storage }
    }

    pub fn mint(&mut self, to: S::Id, amount: S::Value) {
        self.storage.credit(to, amount);
    }

    pub fn transfer(&mut self, from: S::Id, to: S::Id, amount: S::Value) -> Option<S::Value> {
        self.storage.debit(from, amount.clone())?;
        self.storage.credit(to, amount.clone());
        Some(amount)
    }

    pub fn balance_of(&self, user: S::Id) -> S::Value {
        self.storage.balance_of(user)
    }

}
