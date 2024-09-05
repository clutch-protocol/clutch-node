use crate::node::block::Block;
use crate::node::consensus::Consensus;

use super::time_utils::get_current_timespan;

#[derive(Debug)]
pub struct Aura {
    pub authorities: Vec<String>, // List of validators
    pub step_duration: u64,       // Duration of each step in seconds
}

impl Aura {
    pub fn new(authorities: Vec<String>, step_duration: u64) -> Self {
        Self {
            authorities,
            step_duration,
        }
    }

    // Determine the slot number based on a given timestamp
    fn slot_at_time(&self, timestamp: u64) -> u64 {
        timestamp / self.step_duration
    }

    // Determine the current slot number based on the system time
    fn current_slot(&self) -> u64 {
        let current_timespan = get_current_timespan();
        self.slot_at_time(current_timespan)
    }
}

impl Consensus for Aura {
    fn current_author(&self) -> &String {
        let slot = self.current_slot() as usize;
        &self.authorities[slot % self.authorities.len()]
    }

    fn verify_block_author(&self, block: &Block) -> Result<(), String> {
        let expected_author = self.current_author();
        if &block.author == expected_author {
            Ok(())
        } else {
            Err(format!(
                "Block author verification failed: expected author {}, but found {}",
                expected_author, block.author
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_author() {
        let aura = Aura::new(vec!["node_1".to_string(), "node_2".to_string()], 60);
        let slot = aura.current_slot() as usize;
        let expected_author = &aura.authorities[slot % aura.authorities.len()];
        println!(
            "current slot: {:?}, expected_author: {:?}",
            slot, expected_author
        );
        assert_eq!(aura.current_author(), expected_author);
    }
}
