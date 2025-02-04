use crate::support::{Dispatch, DispatchResult, Extrinsic, Header};
use crate::types::{AccountId, Balance, Block, BlockNumber, Content, Nonce};
use crate::RuntimeCall::{ProofOfExistence,BalanceTransfer};

mod balances;
mod proof_of_existence;
mod support;
mod system;
mod types;

pub enum RuntimeCall {
    BalanceTransfer(balances::Call<Runtime>),
    ProofOfExistence(proof_of_existence::Call<Runtime>),
}

impl system::Config for Runtime {
    type AccountID = AccountId;
    type BlockNumber = BlockNumber;
    type Nonce = Nonce;
}
impl balances::Config for Runtime {
    type Balance = Balance;
}
impl proof_of_existence::Config for Runtime {
    type Content = Content;
}
impl Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountID;
    type Call = RuntimeCall;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
        match call {
            RuntimeCall::BalanceTransfer(call) => {
                self.balances.dispatch(caller, call)?;
            }
            RuntimeCall::ProofOfExistence(call) => {
                self.proof_of_existence.dispatch(caller, call)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
    proof_of_existence: proof_of_existence::Pallet<Self>,
}

impl Runtime {
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
            proof_of_existence: proof_of_existence::Pallet::new(),
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
                call: BalanceTransfer(balances::Call::Transfer {
                    receiver: bob.clone(),
                    amount: 30,
                }),
            },
            Extrinsic {
                caller: alice.clone(),
                call: BalanceTransfer(balances::Call::Transfer {
                    receiver: charlie.clone(),
                    amount: 20,
                }),
            },
        ],
    };
    runtime.execute_block(block_1).expect("Invalid block.");

    runtime.system.inc_block_number();
    let block_2 = Block {
        header: Header { block_number: 2 },
        extrinsic: vec![
            Extrinsic {
                caller: alice.clone(),
                call: ProofOfExistence(proof_of_existence::Call::CreateClaim { claim: "ggs" }),
            },
            Extrinsic {
                caller: bob.clone(),
                call: ProofOfExistence(proof_of_existence::Call::RevokeClaim { claim: "ggs" }),
            },
            Extrinsic {
                caller: alice.clone(),
                call: ProofOfExistence(proof_of_existence::Call::RevokeClaim { claim: "ggs" }),
            },
            Extrinsic {
                caller: bob.clone(),
                call: ProofOfExistence(proof_of_existence::Call::CreateClaim { claim: "lakaka" }),
            },
        ],
    };
    runtime.execute_block(block_2).expect("Invalid block.");

    println!("{runtime:#?}");
}
