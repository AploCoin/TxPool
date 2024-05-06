use std::collections::{BTreeMap, HashMap, HashSet};

use blockchaintree::transaction::Transactionable;

#[derive(Default)]
struct TxPool {
    transactions: HashMap<[u8; 32], Box<dyn Transactionable>>,
    time_reference: BTreeMap<u64, HashSet<[u8; 32]>>,
}

// получение хешей транзакций 
fn get_tx_hash(transactions: &HashMap<[u8; 32], Box<dyn Transactionable>>) -> Option<[u8; 32]> {
    for (tx_hash, _) in transactions.iter() {
        return Some(*tx_hash);
    }
    None
}

impl TxPool {

    pub fn add_transaction(&mut self, transaction: Box<dyn Transactionable>) -> bool {
        let mut transactions = HashMap::new();
        let tx_hash = get_tx_hash(&transactions).unwrap();
        // Проверяем, если хеш(транзакция) есть
        if transactions.contains_key(&tx_hash) {
            return false;
        }    
            // Если хэша нет, добавляем транзакцию.
            transactions.insert(tx_hash, transaction);

            true
    }
        
    }

