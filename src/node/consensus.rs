use crate::node::block::Block;

pub trait Consensus {
    fn current_author(&self) -> &String;
    fn verify_block_author(&self, block: &Block) -> bool;
}