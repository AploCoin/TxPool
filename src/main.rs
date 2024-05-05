use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use std::fmt::{Display, Debug};
use BlockChainTree;

//fn main() {
//    println!("Hello, world!");
//}

pub trait Transactionable: {
    fn get_tx_time(&self) -> tx_time;
}

struct TxStore<tx_hash: Hash, tx_time: Ord> {
    transactions: HashMap<tx_hash, Box<dyn Transactionable<tx_time>>>,
    time_reference: BTreeMap<tx_time, HashSet<tx_hash>>,
}

impl<tx_hash: Hash, tx_time: Ord> TxStore<tx_hash, tx_time> {
    pub fn new() -> Self {
        TxStore {
            transactions: HashMap::new(),
            time_reference: BTreeMap::new(),
        }
    }
    pub fn add_transaction(transaction: Box<dyn Transactionable<tx_time>>) -> bool {
        let tx_hash = transaction.get_hash();
        if self.transactions.contains_key(&tx_hash) {
            return false;
        }
    } 
}
