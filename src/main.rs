use crate::support::{Dispatch, DispatchResult, Extrinsic, Header};
use crate::types::{AccountId, Balance, Block, BlockNumber, Nonce};

mod balances;
mod support;
mod system;
mod types;

pub enum RuntimeCall {
    BalanceTransfer(balances::Call<Runtime>),
}

impl system::Config for Runtime {
    type AccountID = AccountId;
    type BlockNumber = BlockNumber;
    type Nonce = Nonce;
}
impl balances::Config for Runtime {
    type Balance = Balance;
}
impl Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountID;
    type Call = RuntimeCall;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
        match call {
            RuntimeCall::BalanceTransfer(call) => {
                self.balances.dispatch(caller, call)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
}

impl Runtime {
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }

    fn execute_block(&mut self, block: Block) -> DispatchResult {
        self.system.inc_block_number();

        if block.header.block_number == self.system.block_number() {
            Err("Invalid block number")?;
        }

        for (i, Extrinsic { call, caller }) in block.extrinsic.into_iter().enumerate() {
            self.system.inc_nonce(&caller);
            let _res = self.dispatch(caller, call).map_err(|e| {
                eprintln!(
                    "Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
                    block.header.block_number, i, e
                )
            });
        }

        Ok(())
    }
}

fn main() {
    let mut runtime = Runtime::new();

    let alice = String::from("alice");
    let bob = String::from("bob");
    let charlie = String::from("charlie");

    runtime.balances.set_balance(&alice, 100);

    runtime.system.inc_block_number();

    assert_eq!(runtime.system.block_number(), 1);

    runtime.system.inc_nonce(&alice);

    let block_1 = Block {
        header: Header { block_number: 1 },
        extrinsic: vec![
            Extrinsic {
                caller: alice.clone(),

                call: RuntimeCall::BalanceTransfer(balances::Call::Transfer {
                    receiver: bob.clone(),
                    amount: 30,
                }),
            },
            Extrinsic {
                caller: alice.clone(),

                call: RuntimeCall::BalanceTransfer(balances::Call::Transfer {
                    receiver: charlie.clone(),
                    amount: 20,
                }),
            },
        ],
    };

    runtime.execute_block(block_1).expect("Invalid block.");

    println!("{runtime:#?}");
}
