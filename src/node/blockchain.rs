use crate::node::block::Block; 

#[derive(Debug)]
pub struct Blockchain {
    pub name: String,
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new(name:String) -> Blockchain {            
        let genesis_block = Block::new_genesis_block();        
        let blockchain = Blockchain {
            name: name,      
            blocks: vec![genesis_block],
        };

        blockchain
    }

    pub fn add_block(&mut self, block:Block){
        self.blocks.push(block);
    }
}