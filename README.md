# Transaction Anatomy & Versioned Transactions (v0)

A complete implementation **built in native, pure Rust** demonstrating Solana's Version-0 transactions with Address Lookup Tables (ALTs) for optimized on-chain operations!

## Overview

This project explores the difference between legacy and versioned transactions, focusing on how Address Lookup Tables enable high-throughput DeFi and MEV operations by compressing transaction size.

## Key Concepts

### 1. Legacy vs. Version-0 (v0) Transactions

**Legacy Transactions**
- Flat list of account keys
- No ALT support
- Transaction size grows quickly with additional accounts/instructions

**Version-0 (v0) Transactions**
- Use Address Lookup Tables (ALTs) to compress account metadata
- Support far more instructions and accounts in a single transaction
- Required for large DeFi/MEV/protocol operations

### 2. Address Lookup Tables (ALTs)

An ALT is an on-chain list of addresses that helps keep transactions small by replacing repeated account key references with compact index values.

**Benefits:**
- Reduces transaction size significantly
- Essential for protocols using dozens of account inputs
- Used exclusively by v0 Message transactions

**Operations:**
- Create new lookup tables
- Extend tables with additional addresses

### 3. Transaction Serialization

A Versioned Transaction contains:

- **Signatures**: All signer signatures placed at the top of the serialized form
- **Message**:
  - Version indicator (0)
  - Header (required signature count, read-only account count, etc.)
  - Static + ALT-loaded account keys
  - Recent blockhash
  - List of instructions
- **Keys**: Mix of directly included keys and ALT-indexed references
- **Serialization**: Bincode encoding transported as base64

## Implementation Details

### What This Program Does

1. **Constructs Transfer Instructions**
   - Transfer from payer to recipient 1
   - Transfer from payer to recipient 2
   - Transfer from payer to recipient 3

2. **Creates ALT on Devnet**
   - Generates recent slot
   - Creates lookup table via instruction
   - Signs and submits transaction

3. **Extends ALT**
   - Adds all three recipient public keys to the lookup table

4. **Compiles v0 Message**
   - Uses `V0Message::try_compile()` with static and ALT accounts

5. **Creates Versioned Transaction**
   - Builds `VersionedTransaction` with signatures and compiled message

6. **Serializes to Base64**
   - Encodes complete transaction for RPC transport

7. **Simulates Against Devnet**
   - Validates transaction without execution
   - Enables signature verification
   - Confirms recent blockhash replacement behavior

## Building & Running

```bash
cargo build --release
cargo run
```

## Technical Stack

- **Language**: Pure Rust (no FFI or bindings)
- **Framework**: Solana SDK (`solana-sdk`, `solana-client`, `solana-address-lookup-table-program`)
- **Network**: Devnet
- **Serialization**: Bincode → Base64

## Sample Output

```
Payer: Fskji1sm9H8QwZBGmuRTTie6B111RhCfLtbALMaNRkt
Recipients:
  1: 1111111QLbz7JHiBTspS962RLKV8GndWFwiEaqKM
  2: 1111111ogCyDbaRMvkdsHB3qfdyFYaG1WtRUAfdh
  3: 11111112D1oxKts8YPdTJRG5FzxTNpMtWmq8hkVx3
Recent slot for ALT creation: 426146356
Derived lookup table pubkey: 6Y7aDhD2dkPheFJVmiogEzP4GuZbFrtnet1nEjSZvqxj
Create ALT tx sig: 4oNS7YVZxGNq9iWYG64Uv6Tr3nBHJmC9Zp3jy9tZbPFZ82JQ4AWEsnLMkCM9JRLiqxyxGfWbQqitgapgUby75K7h
Extend ALT tx sig: 43rDdXA3WfouqJkKCkCLrpNorqT3e5cSHotrmjs2NZJgcUkChM4JGWcZW5xX7F6oqxvDWprzVmPMDDuJwhiJtePk

Base64 serialized v0 transaction:
AWeqt0sSH7fhL3nNFtIwsWeIg0kgZClD3AumkCFeaGlF1gFoHGoLebOGE8TPZh6D4gH5eRovHoNNwV8t3Dm/NwaAAQABAgPPhOaBOqwheggmphlGTP6tEFYo4+Ew2wGLCodcQJtpAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAFlUUPN3ulkzOOxM9id+NBz0EYGoET+IeUSwClFXmWHwMBAgACDAIAAAAQJwAAAAAAAAECAAMMAgAAABAnAAAAAAAAAQIABAwCAAAAECcAAAAAAAABUkLfHMExPpMug/nSYrlDZTJiX4kDA9yjmGXOQvaZzEgDAAECAA==

Simulation result:
✅ No errors
✅ 3 successful system program invocations
✅ 450 compute units consumed
```

## Results

✅ Successfully implemented complete v0 transaction pipeline:
- v0 Transaction construction
- Multiple instructions in single transaction
- ALT creation and usage
- Base64 serialization
- RPC simulation on Devnet

This demonstrates the exact workflow used when preparing high-throughput transactions optimized for Solana's v0 transaction model—all written in native, pure Rust using the Solana SDK!
