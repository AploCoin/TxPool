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

#[test]
fn pop_transactions_test() {
    let mut pool = TxPool::new();

    for i in 0..100 {
        pool.add_transaction(Box::new(Transaction::new(
            [0; 33],
            [0; 33],
            i,
            U256::zero(),
            [1; 32],
            None,
        )));
    }

    let trxs = pool.pop_transactions(50);
    for i in 0..50 {
        assert_eq!(trxs[i].get_timestamp(), i as u64);
    }
    let trxs = pool.pop_transactions(50);
    for i in 50..100 {
        assert_eq!(trxs[i - 50].get_timestamp(), i as u64);
    }

    assert!(pool.pop_transaction().is_none());
}
