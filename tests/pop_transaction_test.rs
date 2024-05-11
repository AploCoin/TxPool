use blockchaintree::transaction::Transaction;
use primitive_types::U256;
use txpool::TxPool;

#[test]
fn pop_transaction_test() {
    let mut pool = TxPool::new();

    pool.add_transaction(Box::new(Transaction::new(
        [0; 33],
        [0; 33],
        0,
        U256::zero(),
        [1; 32],
        None,
    )));

    let popped = pool.pop_transaction();

    assert!(popped.is_some());
    assert_eq!(popped.unwrap().get_amount().unwrap(), U256::zero());

    let popped = pool.pop_transaction();

    assert!(popped.is_none());
}
