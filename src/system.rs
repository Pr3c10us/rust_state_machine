use num::{one, zero, One, Zero};
use std::collections::BTreeMap;
use std::ops::AddAssign;

pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: Zero + One + AddAssign + Copy;
    type Nonce: Zero + One + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            block_number: zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number += one();
    }

    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        let binding = zero();
        let current_nonce = self.nonce.get(who).unwrap_or(&binding);
        self.nonce.insert(who.clone(), *current_nonce + one());
    }
}
#[cfg(test)]
mod test {
    use crate::system::{Config, Pallet};

    struct TestConfig {}
    impl Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }
    #[test]
    fn init_system() {
        let mut system: Pallet<TestConfig> = Pallet::new();

        system.inc_block_number();
        system.inc_nonce(&"alice".to_string());

        assert_eq!(system.block_number, 1);
        assert_eq!(system.nonce.get(&"alice".to_string()), Some(&1));
        assert_eq!(system.nonce.get(&"bob".to_string()), None);
    }
}
