// Blockchain — the chain itself, plus mempool and balance logic.

use crate::block::{Block, GENESIS_PREV_HASH};
use crate::transaction::{Transaction, NITS_PER_TOKEN};
#[allow(dead_code)]

pub struct Blockchain {
    pub chain:      Vec<Block>,
    pub difficulty: String,
    pub mempool:    Vec<Transaction>,
    pub reward:     u64,
}

impl Blockchain {
    pub fn new(difficulty: &str) -> Self {
        println!("🔗 Initializing blockchain [difficulty={}]", difficulty);
        let genesis = {
            let mut b = Block::new(0, vec![], GENESIS_PREV_HASH.to_string(), "NETWORK".to_string());
            b.mine(difficulty);
            b
        };
        Blockchain {
            chain: vec![genesis],
            difficulty: difficulty.to_string(),
            mempool: vec![],
            reward: 50 * NITS_PER_TOKEN,
        }
    }

    /// Add a signed transaction to the mempool.
    /// Rejects invalid or unsigned transactions immediately.
    pub fn add_transaction(&mut self, txn: Transaction) -> Result<(), String> {
        txn.validate()?;
        println!("  📥 Mempool: {}", txn.display());
        self.mempool.push(txn);
        Ok(())
    }

    /// Mine all mempool transactions into a new block.
    /// Miner receives reward as a coinbase transaction.
    pub fn mine_pending_transactions(&mut self, miner_address: String) -> Result<(), String> {
        if self.mempool.is_empty() {
            return Err("Mempool is empty — nothing to mine".to_string());
        }
        println!("\n⛏️  Mining block #{}...", self.chain.len());
        let reward_txn = Transaction::new_nits("NETWORK".to_string(), miner_address.clone(), self.reward);
        let mut transactions: Vec<Transaction> = self.mempool.drain(..).collect();
        transactions.push(reward_txn);
        let previous_hash = self.chain.last()
            .ok_or("Chain is empty")?.hash.clone();
        let index = self.chain.len() as u32;
        let mut new_block = Block::new(index, transactions, previous_hash, miner_address);
        new_block.mine(&self.difficulty);
        self.chain.push(new_block);
        println!("  ✅ Block #{} confirmed\n", self.chain.len() - 1);
        Ok(())
    }

    /// Replay every transaction from genesis to get current balance.
    pub fn get_balance(&self, address: &str) -> Result<f64, String> {
        let mut balance: u64 = 0;
        for block in &self.chain {
            for txn in &block.transactions {
                if txn.to == address {
                    balance = balance.checked_add(txn.amount)
                        .ok_or("Balance overflow")?;
                }
                if txn.from == address {
                    balance = balance.checked_sub(txn.amount)
                        .ok_or("Balance underflow — spending more than available")?;
                }
            }
        }
        Ok(balance as f64 / NITS_PER_TOKEN as f64)
    }

    pub fn validate(&self) -> Result<(), String> {
        for i in 1..self.chain.len() {
            let current  = &self.chain[i];
            let previous = &self.chain[i - 1];
            let expected = current.calculate_hash();
            if current.hash != expected {
                return Err(format!("Block #{} hash mismatch", i));
            }
            if current.previous_hash != previous.hash {
                return Err(format!("Block #{} disconnected from chain", i));
            }
            current.validate_transactions()?;
        }
        Ok(())
    }

    pub fn is_valid(&self) -> bool {
        match self.validate() {
            Ok(_)    => true,
            Err(msg) => { println!("  ❌ {}", msg); false }
        }
    }

    pub fn print_chain(&self) {
        println!("\n{}", "═".repeat(50));
        println!("📦 BLOCKCHAIN — {} blocks", self.chain.len());
        println!("{}", "═".repeat(50));
        for block in &self.chain { block.display(); println!(); }
    }
}