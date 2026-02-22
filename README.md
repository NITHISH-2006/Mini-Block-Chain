# Mini-Block-Chain ⛏️

Single-node Proof of Work miner written in Rust.  
First Rust program I've ever written. Gave myself 6 hours.

---

## Why I built this

I've been doing React and JavaScript for a while and wanted to understand what's
actually happening inside a blockchain node — not conceptually, but in code.
What does a block look like in memory? How does SHA-256 hashing actually work?
Why is mining slow but verification instant?

So I gave myself a 6-hour timebox and wrote it in Rust, because if I'm going to
poke at protocol-level stuff, I should probably stop using a garbage-collected language.

---

## What it does

1. Creates a `Block` struct — index, timestamp, data, previous hash, nonce, current hash
2. Hashes the block with SHA-256, packing every field into a single input string
3. Mines it — keeps incrementing the nonce until the hash starts with `0000`
4. Prints the result

That's it. No UI. No server. Just a block getting mined in a terminal.

---

## Run it

Need Rust installed. If you don't have it:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then:
```bash
git clone https://github.com/NITHISH-2006/Mini-Block-Chain.git
cd Mini-Block-Chain
cargo run
```

Output looks like this:
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

The nonce changes every run because the timestamp is part of the hash input.

---

## Things I actually learned writing this

**Ownership stopped me from doing dumb things**  
`calculate_hash(&self)` borrows the block to read it. `mine(&mut self)` explicitly
says it's going to mutate. I couldn't mix those up even if I wanted to —
the compiler just refuses to build. Coming from JS where you can mutate anything
from anywhere, this was initially annoying, then obviously correct.

**No null, no undefined, no surprises**  
Reading the system clock returns a `Result`. `.unwrap()` says "crash here if
this fails" instead of letting it silently break somewhere downstream. I kind of
love this.

**SHA-256 has no pattern**  
Change one character anywhere in the input — different block index, one extra
space in the data — and the output is completely unrecognizable. There's no
gradient, no shortcut. The only way to find a hash starting with `0000` is to
try ~65,000 times on average and get lucky. Verification is one hash call.
That gap between producing and verifying a valid block is the whole point of PoW.

---

## Adjusting difficulty
```rust
genesis_block.mine("0000");   // ~65k attempts on average
genesis_block.mine("00000");  // ~1M attempts — takes noticeably longer
genesis_block.mine("000");    // ~4k attempts — almost instant
```

Each zero multiplies the expected work by 16.

---

## What's next

- [ ] Add a `Blockchain` struct — a `Vec<Block>` with chain validation so tampering any block breaks everything after it
- [ ] Parallel mining with `rayon` — split the nonce range across threads
- [ ] `axum` HTTP layer — `GET /chain` and `POST /mine` so it runs as an actual node

---

## Project structure
```
Mini-Block-Chain/
├── src/
│   └── main.rs
├── Cargo.toml
└── Cargo.lock
```

One dependency: `sha2` for SHA-256.

---

*First Rust project. 6-hour timebox. CS student, graduating 2027.*  
*Trying to understand systems from the ground up before I have to work on them.*
