use crate::support::{Dispatch, Extrinsic, Header};
use crate::types::{AccountId, Balance, Block, BlockNumber, Content, Nonce};

mod balances;
mod proof_of_existence;
mod support;
mod system;
mod types;

impl system::Config for Runtime {
    type AccountId = AccountId;
    type BlockNumber = BlockNumber;
    type Nonce = Nonce;
}
impl balances::Config for Runtime {
    type Balance = Balance;
}
impl proof_of_existence::Config for Runtime {
    type Content = Content;
}

#[derive(Debug)]
#[macros::runtime]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
    proof_of_existence: proof_of_existence::Pallet<Self>,
}

fn main() {
    let mut runtime = Runtime::new();
    let alice = String::from("alice");
    let bob = String::from("bob");
    let charlie = String::from("charlie");

    runtime.balances.set_balance(&alice, 100);

    let block_1 = Block {
        header: Header { block_number: 1 },
        extrinsics: vec![
            Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::balances(balances::Call::transfer {
                    receiver: bob.clone(),
                    amount: 30,
                }),
            },
            Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::balances(balances::Call::transfer {
                    receiver: charlie.clone(),
                    amount: 20,
                }),
            },
        ],
    };
    let block_2 = Block {
        header: Header { block_number: 2 },
        extrinsics: vec![
            Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: "ggs",
                }),
            },
            Extrinsic {
                caller: bob.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::revoke_claim {
                    claim: "ggs",
                }),
            },
            Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::revoke_claim {
                    claim: "ggs",
                }),
            },
            Extrinsic {
                caller: bob.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: "lakaka",
                }),
            },
        ],
    };
    let block_3 = types::Block {
        header: support::Header { block_number: 3 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice,
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::revoke_claim {
                    claim: "Hello, world!",
                }),
            },
            support::Extrinsic {
                caller: bob,
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: "Hello, world!",
                }),
            },
        ],
    };
    runtime.execute_block(block_1).expect("Invalid block.");
    runtime.execute_block(block_2).expect("Invalid block.");
    runtime.execute_block(block_3).expect("Invalid block.");

    println!("{runtime:#?}");
}
