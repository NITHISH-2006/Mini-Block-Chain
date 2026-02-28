// ============================================================
// BLOCKCHAIN MODULE  (v2.1 — fixed)
// ------------------------------------------------------------
// FIXES from v2:
//   1. Genesis block uses real GENESIS_PREV_HASH (64 hex zeros)
//   2. add_transaction() returns Result — caller knows if it was rejected and why
//   3. is_valid() returns Result<(), String> — descriptive errors
//   4. get_balance() uses checked arithmetic — no u64 overflow crash
//   5. Removed Python-style format strings (were compile errors)
// ============================================================

use crate::block::{Block, GENESIS_PREV_HASH};
use crate::transaction::{Transaction, NITS_PER_TOKEN};

pub struct Blockchain {
    pub chain:      Vec<Block>,
    pub difficulty: String,
    pub mempool:    Vec<Transaction>,
    pub reward:     u64,   // mining reward in nits (not tokens)
}

impl Blockchain {
    pub fn new(difficulty: &str) -> Self {
        println!("🔗 Initializing blockchain [difficulty={}]", difficulty);

        // Genesis block: no transactions, points to the all-zero hash
        let genesis = {
            let mut b = Block::new(
                0,
                vec![],
                GENESIS_PREV_HASH.to_string(),
                "NETWORK".to_string(),
            );
            b.mine(difficulty);
            b
        };

        Blockchain {
            chain: vec![genesis],
            difficulty: difficulty.to_string(),
            mempool: vec![],
            reward: 50 * NITS_PER_TOKEN, // 50 tokens in nits
        }
    }

    /// Submit a signed transaction to the mempool.
    /// Returns Ok(()) if accepted, Err(reason) if rejected.
    /// Only valid (properly signed, non-zero) transactions enter the mempool.
    pub fn add_transaction(&mut self, txn: Transaction) -> Result<(), String> {
        txn.validate()?;  // propagates Err automatically with ?
        println!("  📥 Mempool: {}", txn.display());
        self.mempool.push(txn);
        Ok(())
    }

    /// Mine all pending mempool transactions into a new block.
    /// Miner receives `self.reward` nits as a coinbase transaction.
    /// Empties the mempool — those transactions are now confirmed on-chain.
    pub fn mine_pending_transactions(&mut self, miner_address: String) -> Result<(), String> {
        if self.mempool.is_empty() {
            return Err("Mempool is empty — nothing to mine".to_string());
        }

        println!("\n⛏️  Mining block #{}...", self.chain.len());

        // Coinbase: network rewards the miner — no signature required
        let reward_txn = Transaction::new_nits(
            "NETWORK".to_string(),
            miner_address.clone(),
            self.reward,
        );

        // Drain mempool into the new block, append reward at end
        let mut transactions: Vec<Transaction> = self.mempool.drain(..).collect();
        transactions.push(reward_txn);

        let previous_hash = self.chain.last()
            .ok_or("Chain is empty — this should never happen")?
            .hash
            .clone();

        let index = self.chain.len() as u32;
        let mut new_block = Block::new(index, transactions, previous_hash, miner_address);
        new_block.mine(&self.difficulty);
        self.chain.push(new_block);

        println!("  ✅ Block #{} confirmed\n", self.chain.len() - 1);
        Ok(())
    }

    /// Calculate balance of an address by replaying every transaction on the chain.
    /// Uses checked arithmetic — returns Err on u64 overflow instead of crashing.
    /// This is the "replay" model. Bitcoin uses UTXOs (more efficient, same idea).
    pub fn get_balance(&self, address: &str) -> Result<f64, String> {
        let mut balance: u64 = 0;

        for block in &self.chain {
            for txn in &block.transactions {
                if txn.to == address {
                    balance = balance.checked_add(txn.amount)
                        .ok_or("Balance overflow — u64 limit exceeded")?;
                }
                if txn.from == address {
                    balance = balance.checked_sub(txn.amount)
                        .ok_or(format!(
                            "Balance underflow for {} — spending more than available",
                            &address[..12]
                        ))?;
                }
            }
        }

        Ok(balance as f64 / NITS_PER_TOKEN as f64)
    }

    /// Full chain validation — checks hash integrity, chain links, and signatures.
    /// Returns Ok(()) if chain is valid, Err(description) of first problem found.
    pub fn validate(&self) -> Result<(), String> {
        for i in 1..self.chain.len() {
            let current  = &self.chain[i];
            let previous = &self.chain[i - 1];

            // Check 1: block's own hash must still be correct (tamper detection)
            let expected_hash = current.calculate_hash();
            if current.hash != expected_hash {
                return Err(format!(
                    "Block #{} hash mismatch — expected {}... got {}...",
                    i, &expected_hash[..16], &current.hash[..16]
                ));
            }

            // Check 2: chain link must be intact (no block inserted or removed)
            if current.previous_hash != previous.hash {
                return Err(format!(
                    "Block #{} is disconnected — previous_hash doesn't match block #{} hash",
                    i, i - 1
                ));
            }

            // Check 3: every transaction in this block must have a valid signature
            current.validate_transactions()?;
        }
        Ok(())
    }

    /// Convenience bool wrapper — backwards compatible
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
        for block in &self.chain {
            block.display();
            println!();
        }
    }
}