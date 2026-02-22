use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct Block {
    index: u32,
    timestamp: u64,
    data: String,
    previous_hash: String,
    nonce: u64,
    hash: String,
}

impl Block {
    // Constructor for a new block
    fn new(index: u32, data: String, previous_hash: String) -> Block {
        Block {
            index,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            data,
            previous_hash,
            nonce: 0,
            hash: String::new(),
        }
    }

    // Hashes the block's current state
    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        // Pack all block data into a single string
        let input = format!("{}{}{}{}{}", self.index, self.timestamp, self.data, self.previous_hash, self.nonce);
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    // The Proof of Work algorithm
    fn mine(&mut self, difficulty_prefix: &str) {
        loop {
            self.hash = self.calculate_hash();
            if self.hash.starts_with(difficulty_prefix) {
                println!("Block Mined! Nonce required: {}", self.nonce);
                break;
            }
            // If the hash doesn't meet the target, increment the nonce and try again
            self.nonce += 1;
        }
    }
}

fn main() {
    println!("Mining Genesis Block...");
    let mut genesis_block = Block::new(0, String::from("Genesis Block - Built for Hashira Context"), String::from("0"));
    
    // Require 4 leading zeros (Our arbitrary PoW difficulty)
    genesis_block.mine("0000"); 
    
    println!("{:#?}", genesis_block);
}