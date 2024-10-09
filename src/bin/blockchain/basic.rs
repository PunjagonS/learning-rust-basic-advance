// --------------------------------------------
//                 Blockchain
// --------------------------------------------

use chrono::Utc;
use sha256::digest;

#[derive(Debug, Clone)]
struct BlockChain {
    blocks: Vec<Block>,
}

impl BlockChain {
    fn new() -> Self {
        BlockChain { blocks: vec![] }
    }

    fn starting_block(&mut self) {
        let genesis_block = Block {
            id: 1,
            nonce: 7211,
            data: "I am first or genesis block".to_string(),
            previous_hash: "0000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
            hash: "00008e246a88dee2500b55d0ac37dda73176db4fb36b9e94782da9eb3be8fd52".to_string(),
            timestamp: Utc::now().timestamp(),
        };
        self.blocks.push(genesis_block);
    }

    fn try_add_block(&mut self, new_block: Block) {
        match self.blocks.last() {
            None => {
                println!("The blockcahin does not have atleast one block");
            }

            Some(latest_block) => {
                if self.is_valid_block(&new_block, latest_block) {
                    self.blocks.push(new_block);
                    println!("Block has been successfully added");
                } else {
                    println!("Could not add block, invalid!");
                }
            }
        }
    }

    fn is_valid_block(&self, new_block: &Block, latest_block: &Block) -> bool {
        if new_block.previous_hash != latest_block.hash {
            println!("Block with id {} has wrong previous hash", new_block.id);
            return false;
        } else if !new_block.hash.starts_with("0000") {
            println!("Block with id {} has invalid hash", new_block.id);
            return false;
        } else if new_block.id != latest_block.id + 1 {
            println!(
                "Block with id {} is not next to latest block with id {}",
                new_block.id, latest_block.id
            );
            return false;
        } else if digest(format!(
            "{}{}{}{}{}",
            new_block.id,
            new_block.nonce,
            new_block.data,
            new_block.previous_hash,
            new_block.timestamp
        )) != new_block.hash
        {
            println!("Block with id {} has invalid hash", new_block.id);
            return false;
        }
        true
    }
}

#[derive(Debug, Clone)]
struct Block {
    id: u64,
    nonce: u64,
    data: String,
    previous_hash: String,
    hash: String,
    timestamp: i64,
}

impl Block {
    fn new(id: u64, data: String, previous_hash: String) -> Self {
        let time_now = Utc::now().timestamp();

        let (nonce, hash) = Self::mine_block(id, &data, &previous_hash, time_now);

        Self {
            id,
            nonce,
            data,
            previous_hash,
            hash,
            timestamp: time_now,
        }
    }

    fn mine_block(id: u64, data: &str, previous_hash: &str, timestamp: i64) -> (u64, String) {
        println!("Mining block...");
        let mut nonce = 1;

        loop {
            let block_string = format!("{}{}{}{}{}", id, nonce, data, previous_hash, timestamp);
            let hash = digest(block_string);
            if hash.starts_with("0000") {
                println!("Mined! nonce: {}, hash: {}", nonce, hash);
                return (nonce, hash);
            }
            nonce += 1;
        }
    }
}

fn main() {
    let mut blockchain = BlockChain::new();
    blockchain.starting_block();
    println!("{:?}", blockchain);

    let new_block = Block::new(2, "Azam".to_string(), blockchain.blocks[0].hash.clone());
    blockchain.try_add_block(new_block);
}
