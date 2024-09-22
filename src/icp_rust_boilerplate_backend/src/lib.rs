use ic_cdk::*;
use ic_cdk::storage;
use candid::{ CandidType, Deserialize };
use std::collections::HashMap;

// Account structure
#[derive(Clone, CandidType, Deserialize)]
struct Account {
    id: String,
    password: String,
    balance: f64,
}

// State to store all accounts
#[derive(CandidType, Deserialize)]
struct State {
    accounts: HashMap<String, Account>,
}

impl State {
    fn new() -> Self {
        Self {
            accounts: HashMap::new(),
        }
    }

    fn generate_random_string() -> String {
        let id = ic_cdk::api::id();
        format!("{}_{}", id, ic_cdk::api::time())
    }
}

// Initialize the state storage using IC's storage API
#[init]
fn init() {
    let state = State::new();
    storage::stable_save((state,)).expect("Failed to initialize state");
}

// Safely retrieve the state, or create a new state if it doesn't exist
fn get_state() -> State {
    match storage::stable_restore::<(State,)>() {
        Ok((state,)) => state,
        Err(_) => {
            ic_cdk::println!("State not found or corrupted. Initializing new state.");
            State::new() // Return a new state if none exists
        }
    }
}

// Save the updated state back to storage
fn update_state(new_state: State) {
    storage::stable_save((new_state,)).expect("Failed to save state");
}

// Create a new account
#[update]
fn make_account(password: String) -> String {
    let mut state = get_state();
    let random_id = State::generate_random_string();

    let account = Account {
        id: random_id.clone(),
        password: password.clone(),
        balance: 100.0, // Initial balance
    };

    state.accounts.insert(random_id.clone(), account);
    update_state(state);

    format!("Account created successfully. ID: {}", random_id)
}

// Query account info
#[query]
fn account_info(password: String) -> String {
    let state = get_state();

    let account = state.accounts.values().find(|acc| acc.password == password);
    match account {
        Some(acc) => format!("Account ID: {}, Balance: {}", acc.id, acc.balance),
        None => "Account not found".to_string(),
    }
}

// Check if account exists by destination ID
#[query]
fn check_account(dest_id: String) -> String {
    let state = get_state();

    if state.accounts.contains_key(&dest_id) {
        "Account exists".to_string()
    } else {
        "Account does not exist".to_string()
    }
}

// Transfer money between accounts
#[update]
fn transfer_money(password: String, amount: f64, dest_id: String) -> String {
    let mut state = get_state();

    // Find the source account's ID by matching the password
    let source_account_id = state.accounts
        .iter()
        .find(|(_, acc)| acc.password == password)
        .map(|(id, _)| id.clone());

    // Check if the destination account exists by its ID
    let dest_account_id = state.accounts.get(&dest_id).map(|acc| acc.id.clone());

    match (source_account_id, dest_account_id) {
        (Some(src_id), Some(dest_id)) => {
            let src_balance;
            {
                let src = state.accounts.get_mut(&src_id).unwrap();
                // Check if the source has sufficient balance
                if src.balance >= amount {
                    // Deduct the amount from the source account
                    src.balance -= amount;
                } else {
                    return "Insufficient balance".to_string();
                }
                src_balance = src.balance;
            }

            {
                let dest = state.accounts.get_mut(&dest_id).unwrap();
                // Add the amount to the destination account
                dest.balance += amount;
            }

            // Save updated state
            update_state(state);

            format!("Transferred {} successfully. Source new balance: {}", amount, src_balance)
        }
        _ => "Transfer failed. Either source or destination account not found.".to_string(),
    }
}
