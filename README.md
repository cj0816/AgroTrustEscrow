# AgroTrust Escrow

Milestone-based yield escrow platform for agricultural traders and smallholder farmers on Stellar.

## Problem & Solution
- **Problem**: Smallholder farmers in Uganda wait over 30 days for payment settlement from intermediaries, risking cash flow collapses.
- **Solution**: AgroTrust holds funds securely in Soroban smart contracts and instantly settles payments upon verifier delivery confirmation.

## Timeline
Bootcamp Project (1-Week Prototype)

## Stellar Features Used
- Soroban Smart Contracts
- Stellar Asset Contracts (USDC)
- Account Trustlines

## Prerequisites
- Rust `1.75.0` or higher
- Target `wasm32-unknown-unknown`
- Soroban CLI `>=20.0.0`

## Build & Test

```bash
# Build the contract target
soroban contract build

# Execute cargo unit tests
cargo test

## Deployed Contract

| Field | Value |
|-------|-------|
| Contract ID | `CBI6ARNNS5UKPDLMKQDMGX5HXQHBIO6V7BEYOXOKXGJ7GTZJ42CTG76B` |
| Network | testnet |
| Explorer | [View on stellar.expert](https://stellar.expert/explorer/testnet/contract/CBI6ARNNS5UKPDLMKQDMGX5HXQHBIO6V7BEYOXOKXGJ7GTZJ42CTG76B) |
| Deploy Tx | [View transaction](https://stellar.expert/explorer/testnet/tx/5023e859cddc25e32652e6298360e485008c60b4cfb9f6d80a00b3a00b92f5ca) |
| Deployed | 2026-08-24 08:29:53 UTC |
| Wallet | freighter (`GADH…3E2B`) |
