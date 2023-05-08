mod test_utils;

use test_utils::{MAX_FEE, EMBEDDED_SIZE, ENTITY_TYPE, VERSION, PAYLOAD};
use sdk::blockchain::{AggregateTransaction, EmbeddedTransaction};


#[test]
fn test_set_transaction() {
    // Arrange:
    unsafe {
        MAX_FEE = 1;
        ENTITY_TYPE = 1;
        EMBEDDED_SIZE = 3;
        VERSION = 1;
        PAYLOAD = vec![0u8, 1u8, 2u8, 3u8];
    }

    let mut aggregate = AggregateTransaction::default();
    aggregate.set_max_fee(1);
    for _ in 0..3 {
        let mut embedded = EmbeddedTransaction::default();
        embedded.set_entity_type(1);
        embedded.set_version(1);
        embedded.set_payload(vec![0u8, 1u8, 2u8, 3u8]);
        aggregate.add_embedded_transaction(embedded);
    }
    
    // Act:
    sdk::blockchain::set_transaction(&aggregate);

}