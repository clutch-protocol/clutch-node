pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64, 
}

impl Transaction{
    pub fn new_genesis_transactions() -> Vec<Transaction> {
        vec![]
    }
}
