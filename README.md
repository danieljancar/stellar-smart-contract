# Stellar Smart Contract Example

Find the YouTube video tutorial [here](https://www.youtube.com/).

This repository contains a basic example of a Stellar smart contract written in Rust. It includes all the necessary
instructions to build, deploy, and interact with the contract on the Stellar testnet.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Building the Contract](#building-the-contract)
- [Set up local Source Account](#set-up-local-source-account)
- [Deploying the Contract](#deploying-the-contract)
- [Interacting with the Contract](#interacting-with-the-contract)
    - [Saving a Value](#saving-a-value)
    - [Getting a Value](#getting-a-value)
- [Running Tests](#running-tests)
- [Project Structure](#project-structure)
- [License](#license)

## Prerequisites

Before you begin, ensure you have the following tools installed:

- **Rust**: The Rust programming language. You can install it using [rustup](https://rustup.rs/).
- **Soroban CLI**: The Stellar CLI tool for working with smart
  contracts. [Installation instructions](https://developers.stellar.org/docs/tools/stellar-cli).
- **Cargo**: Rust's package manager, installed automatically with Rust.

## Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/danieljancar/stellar-smart-contract.git
    cd stellar-smart-contract
    ```

2. Install Rust (if you haven't already):

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

3. Install the Soroban CLI:

    ```bash
    cargo install soroban-cli
    ```

## Building the Contract

To build the smart contract, run the following command:

```bash
stellar contract build
```

This command will compile the contract and generate a `contract.wasm` file in the `target/deploy` directory.

## Set up local Source Account

To deploy the contract to the Stellar testnet, you need to set up a local source account. You can create a new account
using the following command:

```bash
stellar keys generate alice \                                                                                                                              
  --rpc-url https://horizon-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015" \
  --network testnet
```

## Deploying the Contract

To deploy the smart contract to the Stellar testnet, run the following command:

```bash
stellar contract deploy \
  --wasm ./target/wasm32-unknown-unknown/release/stellar_smart_contract.wasm \
  --source-account alice \
  --network testnet
```

This command will deploy the contract to the testnet and return the contract's address.

## Interacting with the Contract

### Saving a Value

To save a value to the contract, run the following command:

```bash
stellar contract invoke \                                                                                                                   
  --id <YOUR_CONTRACT_ID> \
  --source-account alice \
  --network testnet \
  -- \
  save_value --key "brogramming_lang" --value "rust"
```

### Getting a Value

To get a value from the contract, run the following command:

```bash
stellar contract invoke \                                                                                                                            
  --id <YOUR_CONTRACT_ID> \
  --source-account alice \
  --network testnet \
  -- \
  get_value --key "brogramming_lang"
```

## Running Tests

To run the tests, execute the following command:

```bash
cargo test
```

## Project Structure

The project is structured as follows:

- `src/`: Contains the Rust source code for the smart contract and tests.
- `Cargo.toml`: The project manifest file that contains dependencies and metadata.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

