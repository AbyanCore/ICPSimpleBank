use ic_cdk::*;
use ic_cdk::storage;
use candid::{CandidType, Deserialize};
use std::collections::HashMap;

// Account structure with enhanced security for password storage
#[derive(Clone, CandidType, Deserialize)]
struct Account {
    id: String,
    password: String, // In real-world, password should be hashed
    balance: f64,
}

// State structure to store all accounts
#[derive(CandidType, Deserialize)]
struct State {
    accounts: HashMap<String, Account>,
}

impl State {
    // Creates a new state
    fn new() -> Self {
        Self {
            accounts: HashMap::new(),
        }
    }

    // Generates a random string as an account ID based on IC's unique ID and time
    fn generate_random_string() -> String {
        let id = ic_cdk::api::id();
        format!("{}_{}", id, ic_cdk::api::time())
    }
}

// Initializes the state storage
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

// Saves the updated state back to storage
fn update_state(new_state: State) {
    storage::stable_save((new_state,)).expect("Failed to save state");
}

// Helper function to validate password length
fn validate_password(password: &str) -> Result<(), String> {
    if password.len() < 8 {
        Err("Password must be at least 8 characters long".to_string())
    } else {
        Ok(())
    }
}

// Helper function to validate balance for transfer
fn validate_amount(amount: f64) -> Result<(), String> {
    if amount <= 0.0 {
        Err("Transfer amount must be greater than 0".to_string())
    } else {
        Ok(())
    }
}

// Create a new account
#[update]
fn make_account(password: String) -> Result<String, String> {
    validate_password(&password)?;

    let mut state = get_state();
    let random_id = State::generate_random_string();

    let account = Account {
        id: random_id.clone(),
        password: password.clone(),
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

    if password.is_empty() {
        return Err("Password cannot be empty".to_string());
    }

    let account = state.accounts.values().find(|acc| acc.password == password);
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
    validate_password(&password)?;
    validate_amount(amount)?;

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
                    return Err("Insufficient balance".to_string());
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

            Ok(format!("Transferred {} successfully. Source new balance: {}", amount, src_balance))
        }
        _ => Err("Transfer failed. Either source or destination account not found.".to_string()),
    }
}

// Delete an account
#[update]
fn delete_account(password: String) -> Result<String, String> {
    let mut state = get_state();

    let account_id = state.accounts
        .iter()
        .find(|(_, acc)| acc.password == password)
        .map(|(id, _)| id.clone());

    match account_id {
        Some(id) => {
            state.accounts.remove(&id);
            update_state(state);
            Ok("Account deleted successfully".to_string())
        }
        None => Err("Account not found or incorrect password".to_string()),
    }
}

// Update account password
#[update]
fn update_password(old_password: String, new_password: String) -> Result<String, String> {
    validate_password(&new_password)?;

    let mut state = get_state();

    let account_id = state.accounts
        .iter()
        .find(|(_, acc)| acc.password == old_password)
        .map(|(id, _)| id.clone());

    match account_id {
        Some(id) => {
            let account = state.accounts.get_mut(&id).unwrap();
            account.password = new_password;
            update_state(state);
            Ok("Password updated successfully".to_string())
        }
        None => Err("Account not found or incorrect old password".to_string()),
    }
}

// Get balance of an account by password
#[query]
fn get_balance(password: String) -> Result<String, String> {
    let state = get_state();

    if password.is_empty() {
        return Err("Password cannot be empty".to_string());
    }

    let account = state.accounts.values().find(|acc| acc.password == password);
    match account {
        Some(acc) => Ok(format!("Current balance: {}", acc.balance)),
        None => Err("Account not found".to_string()),
    }
}

ic_cdk::export_candid!();