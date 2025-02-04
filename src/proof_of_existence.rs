use crate::support::{DispatchResult};
use core::fmt::Debug;
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
    type Content: Debug + Ord;
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

    pub fn _get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
        self.claims.get(&claim)
    }
}

#[macros::call]
impl<T: Config> Pallet<T> {
    pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        if self.claims.contains_key(&claim) {
            Err("this content is already claimed")?;
        }
        self.claims.insert(claim, caller);
        Ok(())
    }

    pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        let owner = self.claims.get(&claim);
        match owner {
            None => {
                Err("Claim does not exist")?;
            }
            Some(owner) => {
                if *owner != caller {
                    Err("Claim does belong to caller")?;
                }
                self.claims.remove(&claim);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::Pallet;

    struct TestConfig;

    impl super::Config for TestConfig {
        type Content = &'static str;
    }

    impl crate::system::Config for TestConfig {
        type AccountId = &'static str;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn basic_proof_of_existence() {
        let mut poe = Pallet::<TestConfig>::new();
        assert_eq!(poe._get_claim(&"Hello, world!"), None);
        assert_eq!(poe.create_claim("alice", "Hello, world!"), Ok(()));
        assert_eq!(poe._get_claim(&"Hello, world!"), Some(&"alice"));
        assert_eq!(
            poe.create_claim("bob", "Hello, world!"),
            Err("this content is already claimed")
        );
        assert_eq!(poe.revoke_claim("alice", "Hello, world!"), Ok(()));
        assert_eq!(poe.create_claim("bob", "Hello, world!"), Ok(()));
    }
}
