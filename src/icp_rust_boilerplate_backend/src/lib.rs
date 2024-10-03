use ic_cdk::*;
use ic_cdk::storage;
use candid::{CandidType, Deserialize};
use std::collections::HashMap;
use std::sync::Mutex;
use bcrypt::{hash, verify};

// Account structure
#[derive(Clone, CandidType, Deserialize)]
struct Account {
    id: String,
    password_hash: String,
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
    if storage::stable_restore::<(State,)>().is_err() {
        let state = State::new();
        storage::stable_save((state,)).expect("Failed to initialize state");
    }
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

// Helper functions for validation
fn validate_password(password: &String) -> Result<(), String> {
    if password.len() < 8 {
        Err("Password must be at least 8 characters long".to_string())
    } else {
        Ok(())
    }
}

fn validate_amount(amount: f64) -> Result<(), String> {
    if amount <= 0.0 { Err("Amount must be greater than 0".to_string()) } else { Ok(()) }
}

// Create a new account
#[update]
fn make_account(password: String) -> Result<String, String> {
    validate_password(&password)?;

    let mut state = get_state();
    let random_id = State::generate_random_string();
    let password_hash = hash(&password, 4).map_err(|_| "Failed to hash password".to_string())?;

    let account = Account {
        id: random_id.clone(),
        password_hash,
        balance: 100.0, // Initial balance
    };

    state.accounts.insert(random_id.clone(), account);
    update_state(state);

    Ok(format!("Account created successfully. ID: {}", random_id))
}

// Query account info
#[query]
fn account_info(password: String) -> Result<String, String> {
    let state = get_state();

    let account = state.accounts.values().find(|acc| verify(&password, &acc.password_hash).unwrap_or(false));
    match account {
        Some(acc) => Ok(format!("Account ID: {}, Balance: {}", acc.id, acc.balance)),
        None => Err("Account not found".to_string()),
    }
}

// Check if account exists by destination ID
#[query]
fn check_account(dest_id: String) -> Result<String, String> {
    let state = get_state();

    if state.accounts.contains_key(&dest_id) {
        Ok("Account exists".to_string())
    } else {
        Err("Account does not exist".to_string())
    }
}

// Transfer money between accounts
#[update]
fn transfer_money(password: String, amount: f64, dest_id: String) -> Result<String, String> {
    // Validate the transfer amount
    validate_amount(amount)?;

    let mut state = get_state();

    // Find the source account's ID by matching the password
    let source_account_id = state.accounts.iter().find_map(|(id, acc)| {
        if verify(&password, &acc.password_hash).unwrap_or(false) { Some(id.clone()) } else { None }
    });

    // Check if the destination account exists by its ID
    let dest_account_exists = state.accounts.contains_key(&dest_id);

    // Check for detailed errors
    if source_account_id.is_none() {
        return Err("Source account not found".to_string());
    }

    if !dest_account_exists {
        return Err("Destination account not found.".to_string());
    }

    let src_id = source_account_id.unwrap();

    {
        let src = state.accounts.get_mut(&src_id).unwrap();
        // Check if the source has sufficient balance
        if src.balance < amount {
            return Err("Insufficient balance in source account.".to_string());
        }
        // Deduct the amount from the source account
        src.balance -= amount;
    }

    {
        let dest = state.accounts.get_mut(&dest_id).unwrap();
        // Add the amount to the destination account
        dest.balance += amount;
    }

    // Save the source account's new balance before updating the state
    let new_balance = state.accounts.get(&src_id).unwrap().balance;

    // Save updated state
    update_state(state);

    Ok(format!("Successfully transferred {}. Source new balance: {}", amount, new_balance))
}

// Delete account
#[update]
fn delete_account(password: String) -> Result<String, String> {
    let mut state = get_state();

    // Find the account's ID by matching the password
    let account_id = state.accounts
        .iter()
        .find(|(_, acc)| verify(&password, &acc.password_hash).unwrap_or(false))
        .map(|(id, _)| id.clone());

    match account_id {
        Some(acc_id) => {
            state.accounts.remove(&acc_id);
            update_state(state);
            Ok("Account deleted successfully".to_string())
        }
        None => Err("Account not found".to_string()),
    }
}

// Update password
#[update]
fn update_password(old_password: String, new_password: String) -> Result<String, String> {
    validate_password(&new_password)?;

    let mut state = get_state();

    let account = state.accounts.values_mut().find(|acc| verify(&old_password, &acc.password_hash).unwrap_or(false));

    match account {
        Some(acc) => {
            acc.password_hash = hash(&new_password, 4).map_err(|_| "Failed to hash password".to_string())?;
            update_state(state);
            Ok("Password updated successfully".to_string())
        }
        None => Err("Account not found".to_string()),
    }
}

ic_cdk::export_candid!();