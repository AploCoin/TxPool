use std::collections::{BTreeMap, HashMap, HashSet};

use blockchaintree::transaction::{self, Transactionable};

#[derive(Default)]
struct TxPool {
    transactions: HashMap<[u8; 32], Box<dyn Transactionable>>,
    time_reference: BTreeMap<u64, HashSet<[u8; 32]>>,
}

impl TxPool {

    pub fn add_transaction(&mut self, transaction: Box<dyn Transactionable>) -> bool {
        let tx_hash = transaction.hash();
        let time = transaction.get_timestamp(); 
        
        self.transactions.entry(tx_hash)
            
            .or_insert(transaction);

        true
    }
}

