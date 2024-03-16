pub struct Transaction {
    pub from: String,
    pub to: String,
    pub value: f64, 
}

impl Transaction{
    pub fn new_genesis_transactions() -> Vec<Transaction> {
        vec![]
    }
}
