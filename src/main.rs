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

#[derive(Debug)]
struct Blockchain{
    chain : Vec<Block>,
    difficulty : String,
}


impl Blockchain {
    fn new(difficulty: &str) -> Blockchain {
        let mut genesis = Block::new(0, String::from("Genesis Block"), String::from("0"));
        genesis.mine(difficulty);

        Blockchain {
            chain: vec![genesis],
            difficulty: difficulty.to_string(),
        }
    }

    fn add_block(&mut self, data: String) {
        let previous_hash = self.chain.last().unwrap().hash.clone();
        let index = self.chain.len() as u32;
        let mut new_block = Block::new(index, data, previous_hash);
        new_block.mine(&self.difficulty);
        self.chain.push(new_block);
    }

    fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            // Check current block's hash is still correct
            if current.hash != current.calculate_hash() {
                println!("Block {} hash is corrupted!", i);
                return false;
            }

            // Check the chain link is intact
            if current.previous_hash != previous.hash {
                println!("Block {} is disconnected from chain!", i);
                return false;
            }
        }
        true
    }
}







fn main() {
    println!("\n==============================");
    println!("     Building Blockchain");
    println!("==============================\n");

    let mut blockchain = Blockchain::new("0000");

    blockchain.add_block("Alice sends Bob 10 tokens".to_string());
    blockchain.add_block("Bob sends Carol 5 tokens".to_string());
    blockchain.add_block("Carol sends Dave 2 tokens".to_string());

    println!("\n--- Full Chain ---");
    for block in &blockchain.chain {
        println!("{:#?}", block);
    }

    println!("\n--- Validation (clean chain) ---");
    println!("Chain valid: {}", blockchain.is_valid());

    // Tamper demo
    println!("\n--- Simulating Tamper Attack ---");
    blockchain.chain[1].data = "Alice sends Bob 1000 tokens".to_string();

    println!("\n--- Validation (after tamper) ---");
    println!("Chain valid: {}", blockchain.is_valid());
}
