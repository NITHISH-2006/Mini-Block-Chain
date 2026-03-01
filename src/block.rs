// Block — holds a list of transactions, mines using Proof of Work.
// Hash covers all transaction data so any tampering is detected.

use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use crate::transaction::Transaction;

pub const GENESIS_PREV_HASH: &str =
    "0000000000000000000000000000000000000000000000000000000000000000";

#[derive(Serialize, Deserialize, Clone)]
#[allow(dead_code)]
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
    pub fn new(index: u32, transactions: Vec<Transaction>, previous_hash: String, miner: String) -> Self {
        Block {
            index,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            transactions,
            previous_hash,
            nonce: 0,
            hash: String::new(),
            miner,
        }
    }

    /// Hash covers every field including all transaction data.
    /// Change anything in any transaction → completely different hash.
    pub fn calculate_hash(&self) -> String {
        let txn_data: String = self.transactions
            .iter()
            .map(|t| format!("{}|{}|{}", t.from, t.to, t.amount))
            .collect::<Vec<_>>()
            .join("::");

        let input = format!("{}::{}::{}::{}::{}",
            self.index, self.timestamp, txn_data, self.previous_hash, self.nonce);

        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Proof of Work — increment nonce until hash starts with difficulty_prefix.
    pub fn mine(&mut self, difficulty_prefix: &str) {
        loop {
            self.hash = self.calculate_hash();
            if self.hash.starts_with(difficulty_prefix) {
                println!("  ⛏️  Block #{} mined  nonce={}  hash={}...",
                    self.index, self.nonce, &self.hash[..16]);
                break;
            }
            self.nonce += 1;
        }
    }

    pub fn validate_transactions(&self) -> Result<(), String> {
        for (i, txn) in self.transactions.iter().enumerate() {
            txn.validate().map_err(|e| {
                format!("Block #{} transaction {}: {}", self.index, i, e)
            })?;
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn has_valid_transactions(&self) -> bool {
        self.validate_transactions().is_ok()
    }

    pub fn display(&self) {
        let hash_short = if self.hash.len() >= 20 { &self.hash[..20] } else { &self.hash };
        let prev_short = if self.previous_hash.len() >= 20 { &self.previous_hash[..20] } else { &self.previous_hash };
        let miner_short = if self.miner.len() >= 12 { &self.miner[..12] } else { &self.miner };
        println!("┌─ Block #{} ────────────────────────────────", self.index);
        println!("│  Hash      : {}...", hash_short);
        println!("│  Prev Hash : {}...", prev_short);
        println!("│  Miner     : {}...", miner_short);
        println!("│  Nonce     : {}", self.nonce);
        println!("│  Txns ({}):", self.transactions.len());
        for txn in &self.transactions { println!("│    • {}", txn.display()); }
        println!("└───────────────────────────────────────────");
    }
}