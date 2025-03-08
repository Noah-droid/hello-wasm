# Hello CosmWasm Smart Contract

## Overview
This is a simple CosmWasm smart contract written in Rust that responds with a random word when queried. The contract supports the following:

- Instantiation
- Querying a random word from a predefined list

## Prerequisites
Ensure you have the following installed:

- Rust and Cargo
- CosmWasm tools
- Wasm compilation target

To install Rust and the necessary tools:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown
```
Generate Project using the template

```sh
cargo generate --git https://github.com/CosmWasm/cw-template.git --name my_contract
cd my_contract
```
Replace my_contract with your preferred name.



## How to Compile
To build the contract:
```sh
cargo build --release --target wasm32-unknown-unknown
```

For an optimized `.wasm` file:
```sh
cargo wasm
```

To further optimize the binary:
```sh
docker run --rm -v "$(pwd)":/code \
    cosmwasm/workspace-optimizer:0.14.0
```
The optimized `.wasm` file will be in the `artifacts/` directory.

## Running Locally Before Deployment
You can test the contract locally using `wasmd` (CosmWasm's blockchain emulator) or `cosmwasm-vm` in Rust:

### Running in `wasmd`
1. Start a local `wasmd` node:
   ```sh
   wasmd start
   ```
2. Store the contract on the blockchain:
   ```sh
   wasmd tx wasm store artifacts/your_contract.wasm --from your-key --gas auto --fees 1000stake
   ```
3. Instantiate the contract:
   ```sh
   wasmd tx wasm instantiate <code_id> '{}' --from your-key --label "random_word_contract" --gas auto --fees 1000stake --no-admin
   ```
4. Query the contract:
   ```sh
   wasmd query wasm contract-state smart <contract_address> '{"random_word": {}}'
   ```

### Running Tests in Rust
To test the contract without deploying:
```sh
cargo test
```
This will execute unit tests written in the contract.

## Deployment
Once tested, deploy your contract to a CosmWasm-supported blockchain like Juno or Osmosis using the same `wasmd` commands.

## License
MIT License.

