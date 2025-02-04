mod balances;
mod system;


impl system::Config for Runtime {
    type AccountID = String;
    type BlockNumber = u32;
    type Nonce = u32;
}

impl balances::Config for Runtime {
    type Balance = u32;
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

    let _first_transfer_result = runtime
        .balances
        .transfer(alice.clone(), bob.clone(), 30)
        .map_err(|e| eprintln!("error: {}", e));

    runtime.system.inc_block_number();

    let _second_transfer_result = runtime
        .balances
        .transfer(alice.clone(), charlie.clone(), 20)
        .map_err(|e| eprintln!("error: {}", e));

    println!("{runtime:#?}");
}
