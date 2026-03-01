# Mini Blockchain ⛏️

Started as a 6-hour timebox to understand how a single block gets mined. Kept going.

This is the same project — I just didn't stop after the first block.

---

## What it is now

A working blockchain node with a REST API. You can create wallets, sign transactions, mine blocks, and query balances over HTTP. Written in Rust. Started from zero Rust knowledge.

**v1** — one block, one file, SHA-256 hashing, proof of work. Built for a hiring round.  
**v2** — added wallets, signed transactions, mempool, chain validation, balance replay.  
**v3** — wrapped everything in a REST API using actix-web.

---

## Run it

```
git clone https://github.com/NITHISH-2006/Mini-Block-Chain.git
cd Mini-Block-Chain
cargo run
```

Server starts at `http://localhost:3000`.

---

## What you can do

```
GET  /wallet/new           — generate a wallet (address + private key)
POST /transaction          — send tokens from one wallet to another
POST /mine                 — mine pending transactions into a block
GET  /chain                — see the full blockchain as JSON
GET  /balance/:address     — check any wallet's balance
GET  /validate             — verify the chain hasn't been tampered with
```

### Try it in order

**1. Make two wallets**
```
GET /wallet/new   ← do this twice, save both responses
```

**2. Send a transaction**
```
POST /transaction
{
  "from": "ALICE_ADDRESS",
  "to": "BOB_ADDRESS",
  "amount": 25,
  "private_key_hex": "ALICE_PRIVATE_KEY"
}
```

**3. Mine it**
```
POST /mine
{
  "miner_address": "ALICE_ADDRESS"
}
```

**4. Check balances**
```
GET /balance/ALICE_ADDRESS
GET /balance/BOB_ADDRESS
```

Alice gets 50 tokens for mining. Bob gets 25 from the transaction.

---

## How it actually works

### Wallets

A wallet is just an ed25519 keypair. The public key is your address — a 64 character hex string. The private key never leaves your hands.

When you send tokens, your private key signs a hash of `(from + to + amount)`. That signature proves you authorized the transaction. Anyone can verify it using your public key, which is just your address. No lookup table. No central authority. The math is self-contained.

### Transactions

Each transaction has a sender, receiver, amount, and a signature. Before a transaction touches the mempool, it gets validated — signature checked, amount nonzero, sender address parses as a real public key.

Amounts are stored as `u64` integers called nits (1 token = 1000 nits). `f64` would give you `0.1 + 0.2 = 0.30000000000000004`. For money that's a bug. Same reason Bitcoin uses satoshis.

### Mempool

Transactions don't go directly into a block. They sit in the mempool — a waiting room. When someone mines, they drain the mempool, bundle everything into a block, add a coinbase transaction rewarding themselves 50 tokens, and do proof of work.

### Blocks

Each block holds a list of transactions. The block's hash covers every field — index, timestamp, all transaction data, previous hash, nonce. Change anything in any transaction and the hash changes. The chain breaks. You can't quietly edit history.

### Chain validation

Three checks on every block:
1. Recalculate the block's hash — does it match what's stored?
2. Does `previous_hash` match the actual previous block's hash?
3. Is every transaction's signature valid?

All three have to pass. The tamper demo at `/validate` breaks the first two simultaneously by modifying a transaction amount.

### Mining

Same as v1. Keep incrementing the nonce until the hash starts with the difficulty prefix. Each extra zero multiplies expected work by 16. Verification is one hash call. That asymmetry is the whole point.

---

## What I learned building this

**Ed25519 over RSA** — faster, smaller keys, used by Solana. Bitcoin uses secp256k1 which is a different elliptic curve but the same idea.

**`Result<>` everywhere** — v1 had `unwrap()` calls that would crash on bad input. v2 returns descriptive errors up the call stack. The `?` operator makes this clean — if something fails, return the error to the caller and let them decide what to do.

**Mutex for shared state** — actix-web handles requests concurrently. Two simultaneous POST /mine requests could corrupt the chain. Wrapping the blockchain in a `Mutex` means only one request touches it at a time.

**Serialization** — ed25519 signatures are raw bytes. JSON doesn't know what to do with raw bytes. Converting signatures to hex strings before storing them means they serialize cleanly and can be decoded back later.

**Balances by replay** — there's no stored balance anywhere. Every call to `/balance/:address` walks the entire chain from block 0, adding incoming amounts and subtracting outgoing ones. Slower than caching, but the result is always consistent with what's actually on chain. Bitcoin's UTXO model is a more efficient version of this same idea.

---

## Project structure

```
src/
├── main.rs         — starts the actix-web server
├── api.rs          — route handlers
├── blockchain.rs   — chain, mempool, balance replay, validation
├── block.rs        — block struct, hashing, proof of work
├── transaction.rs  — signed transfer, validation
└── wallet.rs       — ed25519 keypair, signing, serialization
```

---

## What's next

HTLC atomic swap — a Hash Time-Locked Contract. You lock tokens with a secret hash. The other party claims them by revealing the secret, which simultaneously releases their payment to you. No middleman, no trust required. This is the core mechanic behind [Garden by Hashira](https://hashira.io), which is what pulled me into this in the first place.

---

CS student, graduating 2027. Building from the ground up.  
[Nithish Chandrasekaran](https://linkedin.com/in/nithish-chandrasekaran/)
