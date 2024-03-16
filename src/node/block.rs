use sha2::{Sha256, Digest};

pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64, 
}

pub struct Block {
    pub index: usize,
    pub previous_hash: String,
    pub hash: String,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let transactions_string = self.transactions.iter()
            .map(|tx| format!("{}{}{}", tx.sender, tx.receiver, tx.amount))
            .collect::<Vec<String>>()
            .join(""); 
        hasher.update(format!("{}{}{}", self.index, self.previous_hash, transactions_string));
        let result = hasher.finalize();
        format!("{:x}", result)  
    }
}
