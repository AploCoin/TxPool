use std::collections::{BTreeMap, HashMap, HashSet};

use blockchaintree::transaction::Transactionable;

#[derive(Default)]
pub struct TxPool {
    transactions: HashMap<[u8; 32], Box<dyn Transactionable>>,
    time_reference: BTreeMap<u64, HashSet<[u8; 32]>>,
}

impl TxPool {
    pub fn new() -> Self {
        Self {
            transactions: HashMap::new(),
            time_reference: BTreeMap::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Box<dyn Transactionable>) -> bool {
        let transaction_hash = transaction.hash();
        match self.transactions.entry(transaction_hash) {
            std::collections::hash_map::Entry::Occupied(_) => false,
            std::collections::hash_map::Entry::Vacant(entry) => {
                self.time_reference
                    .entry(transaction.get_timestamp())
                    .and_modify(|inner| {
                        inner.insert(transaction_hash);
                    })
                    .or_insert_with(|| {
                        let mut to_insert = HashSet::new();
                        to_insert.insert(transaction_hash);
                        to_insert
                    });
                entry.insert(transaction);

                true
            }
        }
    }

    pub fn pop_transaction(&mut self) -> Option<Box<dyn Transactionable>> {
        let mut time_reference = self.time_reference.first_entry()?;

        let references = time_reference.get_mut();
        let tx_hash = references.iter().next()?.clone();
        references.remove(&tx_hash);

        if references.is_empty() {
            time_reference.remove();
        }

        self.transactions.remove(&tx_hash)
    }

    pub fn pop_transactions(&mut self, n: usize) -> Vec<Box<dyn Transactionable>> {
        //  TODO: performance issues, fix popping transactions
        (0..n).map_while(|_| self.pop_transaction()).collect()
    }
}
