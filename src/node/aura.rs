use std::time::{SystemTime, UNIX_EPOCH};
use crate::node::consensus::Consensus;
use crate::node::block::Block;

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

    // Determine the current slot number based on the system time
    fn current_slot(&self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now / self.step_duration
    }  
}

impl Consensus for Aura {
    fn current_author(&self) -> &String {
        let slot = self.current_slot() as usize;
        &self.authorities[slot % self.authorities.len()]
    }

    fn verify_block_author(&self, _block: &Block) -> bool {
        //&block.author == self.current_author()
        true
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
