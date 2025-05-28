# Proof of Hash (POH) Blockchain

This is the full implementation of POH — a fast, deterministic, forkless L1 blockchain powered by logic-as-hash and schema-driven smart contracts.

## Features

- Stateless validator with 100ms block time
- RocksDB-based persistent ledger
- Mempool with deduplication
- Genesis chain loader
- CLI wallet for Dilithium-style keypairs
- REST API for transaction relay
- Built-in schema registry (Send, Mint, VaultLock, Custom, etc.)

## Structure

- `validator/` - Core validator binary
- `wallet_cli/` - Standalone CLI wallet
- `validator_api/` - REST API for submitting transactions
- `validator/schemas/` - Schema definitions and loader

## Running the Chain

### 1. Start the validator node
```bash
cd validator
cargo run
```

### 2. Generate a wallet
```bash
cd ../wallet_cli
cargo run -- new
```

### 3. Submit a transaction via REST
```bash
curl -X POST http://localhost:8080/txn -d "payload_here"
```

## Docker Compose (Optional)

This repository includes a `docker-compose.yml` file to run:
- Validator node
- Wallet CLI container
- REST API interface

```bash
docker-compose up --build
```

## License

MIT