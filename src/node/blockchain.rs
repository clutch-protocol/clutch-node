use crate::node::block::Block; 

pub struct Blockchain {
    pub name: &'static str,
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {            
        let genesis_block = Block::new_genesis_block();
        let mut all_blocks = vec![Block::new_genesis_block()];

        let blockchain = Blockchain {
            name: "clutch",      
            blocks: vec![genesis_block],
        };

        blockchain
    }

    pub fn add_block(&mut self, block:Block){
        self.blocks.push(block);
    }

}