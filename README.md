# Mini-Block-Chain ⛏️

First Rust program.

---

## Why

I do React and JS. I wanted to know what a blockchain actually looks like in code, not in a medium article. So I picked Rust because I kept hearing "if you want to understand memory, write Rust" and I wanted to see if that was true.

It was.

---

## What it does

Mines a small chain of 4 blocks, validates the whole thing, then I manually corrupt one block's data and run validation again to watch it break.

That's it. Terminal only. No server, no frontend, nothing clever.

---

## Run it

```bash
git clone https://github.com/NITHISH-2006/Mini-Block-Chain.git
cd Mini-Block-Chain
cargo run
```

You'll need Rust. If you don't have it: https://rustup.rs

---

## Output

```
==============================
     Building Blockchain
==============================

Block Mined! Nonce required: 236327
Block Mined! Nonce required: 2825
Block Mined! Nonce required: 72647
Block Mined! Nonce required: 140315

--- Validation (clean chain) ---
Chain valid: true

--- Simulating Tamper Attack ---

--- Validation (after tamper) ---
Block 1 hash is corrupted!
Chain valid: false
```

Nonce is different every run because timestamp is baked into the hash.

---

## Stuff I actually got from doing this

**The compiler taught me ownership faster than any tutorial**
I kept trying to borrow the block while also mutating it. Rust just wouldn't build. After fighting it for 20 minutes I understood why `&self` and `&mut self` are different things. JS never would have told me I was doing something wrong there.

**SHA-256 genuinely has no shortcut**
I knew this conceptually. Writing it made it real. The only way to find a hash starting with `0000` is to just... try. Over and over. Block 0 took 236,327 attempts. Block 1 took 2,825. Pure luck. Verification is one hash call. That asymmetry is the entire reason PoW works.

**Immutability isn't a feature I added**
I changed one field in Block 1 after the chain was built. The validator caught it immediately. I didn't write any special tamper-detection code — it just falls out of the structure. Each block stores the previous block's hash. If the data changes, the hash changes, and the link breaks. Obvious in hindsight, but seeing it actually happen in output was the moment it clicked.

---

## Difficulty

```rust
Blockchain::new("000");    // fast
Blockchain::new("0000");   // default, few seconds
Blockchain::new("00000");  // go make a coffee
```

Each extra zero is 16x harder.

---

## What's done / what's next

- [x] Block struct + SHA-256 hashing
- [x] Proof of Work mining
- [x] Blockchain with chain validation
- [x] Tamper demo
- [ ] Parallel mining with rayon
- [ ] HTTP endpoints with axum so it actually runs like a node

---

*CS student, graduating 2027. Trying to understand how things actually work before I have to.*
