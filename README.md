# FarmCredit Contracts

[Soroban](https://soroban.stellar.org) smart contracts powering the FarmCredit carbon credit marketplace on the Stellar blockchain.

## About FarmCredit

FarmCredit makes climate action seamless: choose a project, pay with crypto, and receive verifiable proof of your offset—all secured and tracked on the blockchain.

**How it works:**
- **Select A Project** - Choose from verified carbon credit programs or tree planting initiatives worldwide
- **Pay With Crypto** - Complete purchases instantly using stablecoins or other cryptocurrencies
- **Get Blockchain Proof** - Receive an NFT certificate with permanent, verifiable proof of your contribution

## Getting Started

First, build the contracts:

```bash
stellar contract build
# or
cargo build --target wasm32-unknown-unknown --release
```

Run the tests:

```bash
cargo test
```

Deploy to testnet:

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/hello_world.wasm \
  --network testnet \
  --source <YOUR_SECRET_KEY>
```

## Project Structure

```text
.
├── contracts
│   └── hello_world      # Example contract
│       ├── src
│       │   ├── lib.rs   # Contract implementation
│       │   └── test.rs  # Contract tests
│       └── Cargo.toml
├── Cargo.toml           # Workspace configuration
└── README.md
```

- Smart contracts are located in the `contracts` directory, each in their own subdirectory.
- Contracts share dependencies from the workspace `Cargo.toml`.

## Learn More

To learn more about Soroban development, take a look at the following resources:

- [Soroban Documentation](https://soroban.stellar.org/docs) - learn about Soroban features and API.
- [Stellar SDK](https://developers.stellar.org/docs) - comprehensive Stellar development guide.
- [Soroban Examples](https://github.com/stellar/soroban-examples) - explore example contracts.

You can check out the [Soroban GitHub repository](https://github.com/stellar/soroban) - your feedback and contributions are welcome!

## Deploy on Testnet

The easiest way to deploy your Soroban contracts is to use the [Stellar CLI](https://github.com/stellar/stellar-cli):

```bash
# Install the CLI
cargo install --locked stellar-cli

# Fund your account on testnet
stellar keys generate --network testnet --fund

# Deploy your contract
stellar contract deploy --network testnet --source <YOUR_KEY> --wasm <WASM_PATH>
```

Check out the [Soroban deployment documentation](https://soroban.stellar.org/docs/how-to-guides/deploy-to-testnet) for more details.