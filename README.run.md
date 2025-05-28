# POH Deployment Guide

## Run in Dev Mode

Single validator + REST API:
```bash
bash start-dev.sh
```

## Run Full Testnet (3 Validators)

```bash
bash start-net.sh
```

## Reset Ledger

```bash
bash reset-ledger.sh
```

## Generate a Wallet

```bash
bash generate-wallet.sh
```

## Configuration

Edit `.env` or `config/peers.json` to change validator ports, peers, and ledger paths.