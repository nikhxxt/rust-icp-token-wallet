use ic_cdk::export::candid::{CandidType, Principal};
use ic_cdk_macros::{query, update};
use std::collections::HashMap;

#[derive(Default, CandidType)]
struct Wallet {
    balances: HashMap<Principal, u64>,
}

static mut WALLET: Wallet = Wallet {
    balances: HashMap::new(),
};

#[update]
fn send_tokens(to: Principal, amount: u64) -> String {
    let caller = ic_cdk::caller();

    unsafe {
        let sender_balance = WALLET.balances.entry(caller).or_default();
        if *sender_balance < amount {
            return "Insufficient balance.".to_string();
        }

        *sender_balance -= amount;
        let recipient_balance = WALLET.balances.entry(to).or_default();
        *recipient_balance += amount;

        format!("Successfully sent {} tokens to {}", amount, to)
    }
}

#[query]
fn get_balance() -> u64 {
    let caller = ic_cdk::caller();
    unsafe {
        *WALLET.balances.get(&caller).unwrap_or(&0)
    }
}

#[query]
fn initialize_balance(amount: u64) {
    let caller = ic_cdk::caller();
    unsafe {
        WALLET.balances.insert(caller, amount);
    }
}
