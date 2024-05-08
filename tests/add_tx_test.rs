use blockchaintree::transaction::Transaction;
use primitive_types::U256;
use txpool::TxPool;

#[test]
fn test_add_tx() {
    let mut pool = TxPool::new();

    assert_eq!(
        pool.add_transaction(Box::new(Transaction::new(
            [0; 33],
            [1; 33],
            0,
            U256::zero(),
            [1; 32],
            None,
        ))),
        true
    );

    assert_eq!(
        pool.add_transaction(Box::new(Transaction::new(
            [0; 33],
            [1; 33],
            0,
            U256::zero(),
            [1; 32],
            None,
        ))),
        false
    );
}
