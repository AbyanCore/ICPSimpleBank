## ✨ Simple Bank Canister on the Internet Computer ✨

This project showcases a basic bank canister built on the revolutionary Internet Computer (IC) blockchain. It leverages the power of smart contracts and decentralized architecture to provide essential banking functionalities such as account creation, information retrieval, existence verification, and fund transfers.

### 🌟 Features

* **Account Creation:** Users can create new accounts with an initial balance of 100 tokens, securely stored on the blockchain. 🎁
* **Account Information Retrieval:** Users can query their account ID and balance using their password, ensuring privacy and data integrity. 🕵️‍♀️
* **Account Existence Verification:** The system can efficiently check if an account exists based on its ID, thanks to the transparent nature of the blockchain. ✅
* **Fund Transfer:** Users can seamlessly transfer funds between accounts, leveraging the speed and security of the IC blockchain. 💸

### 🚀 How to Run the Project

### 1. Requirements

* rustc 1.64 or higher
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```
* rust wasm32-unknown-unknown target
```bash
$ rustup target add wasm32-unknown-unknown
```
* candid-extractor
```bash
$ cargo install candid-extractor
```
* install `dfx`
```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```

### 2. Update dependencies

Update the `dependencies` block in `/src/{canister_name}/Cargo.toml`:

```
[dependencies]
candid = "0.9.9"
ic-cdk = "0.11.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
ic-stable-structures = { git = "https://github.com/lwshang/stable-structures.git", branch = "lwshang/update_cdk"}
```

### 3. Generate Candid Interface

1. Add the `did.sh` script to the root directory of your project:

```bash
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

2. Update line 16 of the script with the name of your canister.

3. Run the script to generate the Candid interface. 

**Important:** Run this script each time you modify the exported functions of your canister.

**Optional:** Add a `package.json` file with the following content for convenience:

```json
{
  "scripts": {
    "generate": "./did.sh && dfx generate",
    "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
  }
}
```

Now you can use the commands `npm run generate` to generate the Candid interface or `npm run gen-deploy` to generate the Candid interface and deploy the canister.

### 4. Running Locally

```bash
# Starts the local replica
$ dfx start --background

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```

### 🤌 Interact with the Canister:

Utilize tools like `dfx` or Candid UI to interact with the canister's functions:

### 1. Create an Account

This function allows a user to create a new account by providing a password. The password must be at least 8 characters long.

```bash
dfx canister call simple_bank make_account "(\"your_password\")"
```

**Response:**

- **Success**: `Ok("Account created successfully. ID: <account_id>")`
- **Failure**: `Err("Password must be at least 8 characters")`

### 2. Retrieve Account Information

Retrieve account information using the account password.

```bash
dfx canister call simple_bank account_info "(\"your_password\")"
```

**Response:**

- **Success**: `Ok("Account ID: <id>, Balance: <balance>")`
- **Failure**: `Err("Account not found")`

### 3. Check Account Existence

Verify if an account exists by providing the destination account ID.

```bash
dfx canister call simple_bank check_account "(\"destination_account_id\")"
```

**Response:**

- **Success**: `Ok("Account exists")`
- **Failure**: `Err("Account does not exist")`

### 4. Transfer Funds

Transfer funds from your account to a destination account by providing your password, the amount to transfer, and the destination account ID.

```bash
dfx canister call simple_bank transfer_money "(\"your_password\", <amount>, \"destination_account_id\")"
```

**Response:**

- **Success**: `Ok("Transferred <amount> successfully. Source new balance: <balance>")`
- **Failure**: `Err("Transfer failed. Either account not found, incorrect password, or insufficient balance")`

### 5. Delete Account

Delete your account using your password.

```bash
dfx canister call simple_bank delete_account "(\"your_password\")"
```

**Response:**

- **Success**: `Ok("Account deleted successfully")`
- **Failure**: `Err("Account not found")`

### 6. Update Password

Update the password for your account by providing the old password and the new password (minimum of 8 characters).

```bash
dfx canister call simple_bank update_password "(\"old_password\", \"new_password\")"
```

**Response:**

- **Success**: `Ok("Password updated successfully")`
- **Failure**: `Err("Account not found, Password must be at least 8 characters or incorrect password")`

### 📝 Important Notes

* **Account IDs:** Account IDs are generated randomly upon account creation, ensuring uniqueness and anonymity on the blockchain. 🎲
* **Passwords:** Passwords are used for authentication and account management. It's crucial to keep them secure to protect your assets on the blockchain. 🔐
* **Initial Balance:** New accounts are initialized with a balance of 100 tokens, ready for transactions on the IC network. 💰
* **Error Handling:** The canister provides informative error messages for invalid operations (e.g., insufficient balance, account not found), ensuring a smooth user experience. 🚫

### ✨ Future Enhancements

* **Enhanced Security:** Implement advanced cryptographic techniques and multi-factor authentication for even stronger security on the blockchain. 🛡️
* **Transaction History:** Store and provide access to an immutable transaction history for each account, leveraging the transparency of the blockchain. 📜
* **Interest and Fees:** Incorporate functionalities for calculating and applying interest or fees, enabling more complex financial operations on the IC. 📈
* **Frontend Integration:** Develop a user-friendly frontend for seamless interaction with the canister, abstracting the complexities of the blockchain. 🖥️

**Disclaimer:** This is a simplified example for illustrative purposes. It is crucial to implement appropriate security measures and error handling for production-ready applications on the blockchain. 🚧

Feel free to explore, modify, and extend this project to build your own innovative decentralized applications on the Internet Computer! Embrace the power of blockchain technology and happy coding! 🎉

Please let me know if you have any further questions or requests. 😊 
