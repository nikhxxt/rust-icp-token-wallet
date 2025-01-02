
# ICP Token Wallet

## Overview
The **ICP Token Wallet** is a secure wallet built on the **Internet Computer Protocol (ICP)** blockchain using **Rust**. 
This wallet allows users to send and receive **IRCRC2 tokens** and check their token balances. 
The project demonstrates proficiency in **Rust programming**, **blockchain development**, and the principles of managing token transactions.

This project includes a smart contract written in Rust that manages the basic operations of a token wallet. 
It supports sending and receiving IRCRC2 tokens and displays the current balance of tokens.

### Features:
- **Send Tokens**: Users can send tokens to other ICP addresses.
- **Receive Tokens**: The wallet can receive IRCRC2 tokens and update the balance accordingly.
- **Check Balance**: Users can query the current balance of their wallet.

## Setup Instructions

### Prerequisites:
Before running this project, ensure the following are installed on your machine:

- [Rust](https://www.rust-lang.org/tools/install) (Install Rust via `rustup`)
- [Dfinity SDK](https://sdk.dfinity.org/docs/quickstart/) (For deploying and interacting with ICP)

### Steps to Run the Project

1. **Clone the repository:**
   ```bash
   git clone https://github.com/your-username/icp-token-wallet.git
   cd icp-token-wallet
   ```

2. **Install the Dfinity SDK** by following the [Dfinity SDK Quickstart guide](https://sdk.dfinity.org/docs/quickstart/).

3. **Start the local test network:**
   In your project directory, start Dfinity's local test network:
   ```bash
   dfx start --background
   ```

4. **Deploy the contract:**
   Deploy the token wallet smart contract to the local ICP testnet:
   ```bash
   dfx deploy
   ```

5. **Interact with the contract**: You can now use the contract functions like sending tokens and checking balances by interacting with it via Dfinity's tools or through custom client code.

### Smart Contract Functions
The token wallet implements the following key functionalities:
- **send_tokens(to: Principal, amount: u64)**: Sends a specified amount of IRCRC2 tokens to the given address (`Principal`).
- **get_balance()**: Returns the current balance of tokens in the wallet.

These functions are implemented in the `src/main.rs` file, which is the core logic of the smart contract.

## Testing

Unit tests are written to validate the contract's functionality. To run the tests, use:

```bash
cargo test
```

The tests include:
- Verifying token transfers (e.g., sending and receiving tokens).
- Checking if the contract correctly handles insufficient balance scenarios.

### Example Tests:
- **test_send_tokens()**: Verifies that tokens are correctly transferred from the wallet.
- **test_insufficient_balance()**: Tests for scenarios where the sender doesnâ€™t have enough balance to make a transfer.

## Security Considerations

- The wallet ensures that a user cannot send more tokens than their balance.
- Proper validation is implemented to avoid invalid transactions, such as insufficient funds.
- The wallet stores balance in a static variable, which is simple for testing. In a production environment, you would need to use more secure storage methods.

## Deployment

1. **Local Testnet**: The project is configured to run on Dfinity's local test network, which allows developers to test transactions without interacting with the live network.
2. **Mainnet Deployment**: To deploy to the ICP mainnet, additional configuration and security considerations would be required, such as using a secure wallet for private key management and implementing secure token transfer logic.

## File Structure

```plaintext
icp-token-wallet/
â”œâ”€â”€ src/
â”‚   â””â”€â”€ main.rs                  # Smart contract logic (Rust code)
â”œâ”€â”€ tests/
â”‚   â””â”€â”€ main.rs                  # Unit tests for the smart contract
â”œâ”€â”€ dfx.json                      # Dfinity project configuration file
â”œâ”€â”€ Cargo.toml                    # Rust project dependencies
â”œâ”€â”€ README.md                     # Documentation file
â””â”€â”€ .gitignore                    # Git ignore file (to exclude unnecessary files from Git tracking)
```

### Explanation:
- **`src/main.rs`**: Contains the smart contract logic for sending tokens, receiving tokens, and checking balances.
- **`tests/main.rs`**: Contains the unit tests to validate the functionality of the smart contract.
- **`dfx.json`**: Dfinity project configuration file, specifies the canisters and build settings.
- **`Cargo.toml`**: Manages the dependencies for the Rust project, including the ICP SDK libraries.
- **`README.md`**: This file provides an overview and setup instructions for the project.
- **`.gitignore`**: Ignores unnecessary files and directories such as build outputs.

## License

This project is licensed under the [MIT License](LICENSE).

## Contact

For any queries, contact **hr@quadbtech.com** with the subject "Rust Blockchain Developer Task" for prioritized assistance.
