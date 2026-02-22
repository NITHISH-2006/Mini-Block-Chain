# ⛏️ Mini-Block-Chain

> A single-node Proof of Work minter written in Rust — built to understand the bare-metal primitives that power blockchain infrastructure.

---

## Context

I'm a CS student graduating in 2027, transitioning from web development (React, JavaScript) into protocol-level infrastructure engineering. I gave myself a strict **4-hour timebox** to write my first Rust program — not a "Hello World," but something that touches real cryptographic primitives.

This project is my attempt to understand exactly what happens inside a blockchain node at the lowest level: how blocks are structured in memory, how SHA-256 hashing works, and why Proof of Work is computationally expensive to produce but trivially cheap to verify.

---

## What This Does

1. **Constructs a `Block` struct** with strict Rust typing — index, timestamp, data payload, previous block hash, nonce, and current hash.
2. **Hashes the block** using the SHA-256 algorithm (via the `sha2` crate), packing all block fields into a single deterministic input.
3. **Mines the block** via Proof of Work — incrementing a nonce in a brute-force loop until the resulting SHA-256 hash starts with `0000` (4 leading zeros = our difficulty target).
4. **Prints the fully mined block** to the terminal with all fields visible.

---

## Core Rust Concepts Demonstrated

**Structs and `impl` blocks** — Data (`struct Block`) is separated from behavior (`impl Block`). Methods like `new`, `calculate_hash`, and `mine` are attached to the type cleanly, similar to classes but without inheritance.

**Ownership and Borrowing** — `calculate_hash(&self)` borrows the block read-only; the borrow checker guarantees it cannot mutate any field. `mine(&mut self)` takes a mutable borrow, explicitly declaring intent to change `nonce` and `hash`. This distinction is enforced at compile time — not runtime.

**No null, no garbage collector** — Rust has no `null` or `undefined`. Operations that can fail (like reading the system clock) return a `Result` type. `.unwrap()` is used here to say "crash immediately if this fails" rather than silently propagating a bad state.

**Immutable by default** — `let mut genesis_block` requires explicit `mut` because Rust variables are immutable unless you opt in. This forces intentional design around what changes and what doesn't.

---

## How Proof of Work Actually Functions

SHA-256 has two properties that make it ideal for Proof of Work:

- **Avalanche effect:** Changing even one character in the input produces a completely unrecognizable output. There is no gradual change.
- **One-way:** Given a hash output, you cannot reverse-engineer the input. Finding a hash that meets a target condition requires pure brute force.

Because the block's index, timestamp, data, and previous hash are all fixed, calculating the hash repeatedly yields the same result. The `nonce` is the only variable — a dummy counter that changes the input on every iteration, producing a new unpredictable hash each time. On average, finding a hash starting with `0000` requires ~65,536 attempts (16⁴ in the 2²⁵⁶ SHA-256 output space).

Anyone can verify a valid block by running the hash once and checking the prefix — verification is instant. Production is expensive. That asymmetry is the entire point.

---

## Running It

**Prerequisites:** Rust installed via `rustup`

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and run
git clone https://github.com/NITHISH-2006/Mini-Block-Chain.git
cd Mini-Block-Chain
cargo run
```

**Expected output:**
```
Mining Genesis Block...
Block Mined! Nonce required: 95190
Block {
    index: 0,
    timestamp: 1771765906,
    data: "Genesis Block - Built for Hashira Context",
    previous_hash: "0",
    nonce: 95190,
    hash: "0000ce58893a98c70e62f53a63b058e95b3dad32de4f8a65a319e1669aeb185c",
}
```

*(Exact nonce will vary — timestamp changes the hash space on every run.)*

---

## Project Structure

```
Mini-Block-Chain/
├── src/
│   └── main.rs       # Block struct, impl, PoW mining logic, main()
├── Cargo.toml        # Project manifest + sha2 dependency
└── Cargo.lock        # Locked dependency versions
```

---

## Dependencies

| Crate | Purpose |
|-------|---------|
| `sha2` | SHA-256 cryptographic hashing |

---

## Adjusting Difficulty

The difficulty target is set in `main()`:

```rust
genesis_block.mine("0000");   // 4 leading zeros — current setting
genesis_block.mine("00000");  // 5 leading zeros — ~16x harder
genesis_block.mine("000");    // 3 leading zeros — ~16x easier
```

Each additional zero multiplies the expected work by 16.

---

## Roadmap — Phase 2

- [ ] **Blockchain struct** — `Vec<Block>` with `add_block()` and `is_chain_valid()` to prove tamper-evidence across the full chain
- [ ] **Multithreaded mining** — Use `rayon` to spawn parallel threads checking different nonce ranges simultaneously, demonstrating CPU-level performance optimization
- [ ] **HTTP API** — Wrap in `axum` with `GET /chain` and `POST /mine` endpoints, deployable to AWS EC2 as a live infrastructure node

---

## Why This Matters for Protocol Engineering

Projects like Garden, and Solana are built in Rust specifically because the borrow checker eliminates memory bugs that would be catastrophic in financial infrastructure. Writing even a minimal system in Rust — where the compiler refuses to build unsafe code — builds the instinct for thinking about ownership, lifetimes, and concurrency that production protocol engineers use daily.

This is my first step off the JavaScript comfort zone and into bare-metal infrastructure. More to come.

---

*Built in a 4-hour timebox as a deliberate learning exercise. Not production code — a foundation to build from.*
