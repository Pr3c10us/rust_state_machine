use crate::support::DispatchResult;
use num::{zero, CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
    type Balance: Zero + CheckedSub + CheckedAdd + Copy;
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

    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&mut self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&zero())
    }
}


#[macros::call]
impl<T: Config> Pallet<T> {
    pub fn transfer(
        &mut self,
        caller: T::AccountId,
        receiver: T::AccountId,
        amount: T::Balance,
    ) -> DispatchResult {
        let mut caller_balance = self.balance(&caller);
        let mut receiver_balance = self.balance(&receiver);

        caller_balance = caller_balance
            .checked_sub(&amount)
            .ok_or("Not Enough Funds.")?;
        receiver_balance = receiver_balance
            .checked_add(&amount)
            .ok_or("Overflow Balance.")?;

        self.set_balance(&caller, caller_balance);
        self.set_balance(&receiver, receiver_balance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::balances::{Config, Pallet};

    struct TestConfig {}
    impl crate::system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }
    impl Config for TestConfig {
        type Balance = u128;
    }
    #[test]
    fn init_balance() {
        let mut balances: Pallet<TestConfig> = Pallet::new();

        assert_eq!(balances.balance(&"alice".to_string()), 0);

        balances.set_balance(&"alice".to_string(), 100);

        assert_eq!(balances.balance(&"alice".to_string()), 100);

        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        let mut balances: Pallet<TestConfig> = Pallet::new();

        let transfer_result = balances.transfer("alice".to_string(), "bob".to_string(), 100);
        assert_eq!(transfer_result, Err("Not Enough Funds."));

        balances.set_balance(&"alice".to_string(), 100);
        let transfer_result = balances.transfer("alice".to_string(), "bob".to_string(), 55);
        assert_eq!(transfer_result, Ok(()));

        assert_eq!(balances.balance(&"alice".to_string()), 45);
        assert_eq!(balances.balance(&"bob".to_string()), 55);
    }
}
