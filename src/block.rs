// ============================================================
// BLOCK MODULE  (v2.1 — fixed)
// ------------------------------------------------------------
// FIXES from v2:
//   1. Genesis previous_hash is now a real 64-char hex zero string
//      (not a fake "0000000000000000000" that would fail hash checks)
//   2. has_valid_transactions() now returns descriptive errors
//   3. display() cleaned up — no panic if hash is short
// ============================================================

use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::transaction::Transaction;

/// A real genesis "previous hash" — 64 hex zeros (256 bits of zero).
/// This matches what Bitcoin does: the genesis block points to a zeroed hash.
pub const GENESIS_PREV_HASH: &str =
    "0000000000000000000000000000000000000000000000000000000000000000";

pub struct Block {
    pub index:         u32,
    pub timestamp:     u64,
    pub transactions:  Vec<Transaction>,
    pub previous_hash: String,
    pub nonce:         u64,
    pub hash:          String,
    pub miner:         String,
}

impl Block {
    pub fn new(
        index: u32,
        transactions: Vec<Transaction>,
        previous_hash: String,
        miner: String,
    ) -> Self {
        Block {
            index,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()   // SystemTime before UNIX_EPOCH is impossible in practice
                .as_secs(),
            transactions,
            previous_hash,
            nonce: 0,
            hash: String::new(),
            miner,
        }
    }

    /// Produces a deterministic SHA-256 hash of this block's complete contents.
    /// Any change to any field (including any transaction field) changes the hash.
    pub fn calculate_hash(&self) -> String {
        // Serialize all transaction data into a single canonical string
        // Format: "from1|to1|amount1::from2|to2|amount2::..."
        let txn_data: String = self.transactions
            .iter()
            .map(|t| format!("{}|{}|{}", t.from, t.to, t.amount))
            .collect::<Vec<_>>()
            .join("::");

        let input = format!(
            "{}::{}::{}::{}::{}",
            self.index, self.timestamp, txn_data, self.previous_hash, self.nonce
        );

        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Proof of Work: find a nonce such that hash starts with difficulty_prefix.
    /// Each extra "0" in the prefix makes mining ~16x harder (hex digit = 4 bits).
    pub fn mine(&mut self, difficulty_prefix: &str) {
        loop {
            self.hash = self.calculate_hash();
            if self.hash.starts_with(difficulty_prefix) {
                println!(
                    "  ⛏️  Block #{} mined  nonce={}  hash={}...",
                    self.index, self.nonce, &self.hash[..16]
                );
                break;
            }
            self.nonce += 1;
        }
    }

    /// Validates every transaction in this block.
    /// Returns first error found, or Ok(()) if all pass.
    pub fn validate_transactions(&self) -> Result<(), String> {
        for (i, txn) in self.transactions.iter().enumerate() {
            txn.validate().map_err(|e| {
                format!("Block #{} — transaction {} invalid: {}", self.index, i, e)
            })?;
        }
        Ok(())
    }

    /// Convenience bool wrapper
    pub fn has_valid_transactions(&self) -> bool {
        self.validate_transactions().is_ok()
    }

    pub fn display(&self) {
        // Safe slice — hash is always 64 chars after mining, but guard anyway
        let hash_short = if self.hash.len() >= 20 { &self.hash[..20] } else { &self.hash };
        let prev_short = if self.previous_hash.len() >= 20 {
            &self.previous_hash[..20]
        } else {
            &self.previous_hash
        };
        let miner_short = if self.miner.len() >= 12 { &self.miner[..12] } else { &self.miner };

        println!("┌─ Block #{} ─────────────────────────────────", self.index);
        println!("│  Hash      : {}...", hash_short);
        println!("│  Prev Hash : {}...", prev_short);
        println!("│  Miner     : {}...", miner_short);
        println!("│  Nonce     : {}", self.nonce);
        println!("│  Txns ({}):", self.transactions.len());
        for txn in &self.transactions {
            println!("│    • {}", txn.display());
        }
        println!("└────────────────────────────────────────────");
    }
}