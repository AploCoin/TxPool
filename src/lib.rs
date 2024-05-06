use std::collections::{BTreeMap, HashMap, HashSet};

use blockchaintree::transaction::Transactionable;

#[derive(Default)]
struct TxPool {
    transactions: HashMap<[u8; 32], Box<dyn Transactionable>>,
    time_reference: BTreeMap<u64, HashSet<[u8; 32]>>,
}

impl TxPool {}
