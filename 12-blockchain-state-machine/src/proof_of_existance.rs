use std::collections::BTreeMap;
use std::fmt::Debug;

use crate::support::DispatchResult;

pub trait Config: crate::system::Config {
    type Content: Debug + Ord + Clone;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            claims: BTreeMap::new(),
        }
    }

    pub fn get_claim(&self, content: &T::Content) -> Option<T::AccountId> {
        self.claims.get(content).cloned()
    }

    pub fn create_claim(&mut self, caller: T::AccountId, content: T::Content) -> DispatchResult {
        match self.get_claim(&content) {
            Some(_) => Err("Claim already exists"),
            _ => {
                self.claims.insert(content, caller);
                Ok(())
            }
        }
    }

    pub fn revoke_claim(&mut self, caller: T::AccountId, content: T::Content) -> DispatchResult {
        if let Some(claim_owner) = self.get_claim(&content) {
            if claim_owner != caller {
                return Err("Caller is not the owner of the claim");
            }
            self.claims.remove(&content);
        } else {
            return Err("Claim does not exist");
        }
        Ok(())
    }
}

pub enum Call<T: Config> {
    Create { claim: T::Content },
    Revoke { claim: T::Content },
}

impl<T: Config> crate::support::Dispatch for Pallet<T> {
    type Caller = T::AccountId;
    type Call = Call<T>;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
        match call {
            Call::Create { claim } => {
                self.create_claim(caller, claim)?;
            }
            Call::Revoke { claim } => {
                self.revoke_claim(caller, claim)?;
            }
        }
        Ok(())
    }
}

mod tests {
    #[allow(dead_code)]
    struct PoEConfig;

    impl super::Config for PoEConfig {
        type Content = String;
    }

    impl crate::system::Config for PoEConfig {
        type BlockNumber = u64;
        type AccountId = &'static str;
        type Nonce = u64;
    }

    #[test]
    fn test_create_claim() {
        let mut pallet = super::Pallet::<PoEConfig>::new();
        let _ = pallet.create_claim("Alice", "my_document".to_string());
        assert_eq!(pallet.get_claim(&"my_document".to_string()), Some("Alice"));

        let res = pallet.create_claim("Alice", "my_document".to_string());
        assert_eq!(res, Err("Claim already exists"));
    }

    #[test]
    fn test_revoke_claim() {
        let mut pallet = super::Pallet::<PoEConfig>::new();

        let _ = pallet.create_claim("Alice", "document_for_revoke".to_string());
        let res = pallet.revoke_claim("Alice", "document_for_revoke".to_string());
        assert_eq!(res, Ok(()));
        assert_eq!(pallet.get_claim(&"document_for_revoke".to_string()), None);

        let _ = pallet.create_claim("Alice", "document_for_revoke".to_string());
        let res = pallet.revoke_claim("Bob", "document_for_revoke".to_string());
        assert_eq!(res, Err("Caller is not the owner of the claim"));

        let res = pallet.revoke_claim("Alice", "document not exist".to_string());
        assert_eq!(res, Err("Claim does not exist"));
    }
}
