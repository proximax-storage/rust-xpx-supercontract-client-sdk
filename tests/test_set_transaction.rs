use sdk::blockchain::{AggregateTranction, EmbeddedTransaction, set_transaction as other_set_transaction};

fn read16 (offset: usize, ptr: &Vec<u8>) -> u16 {
    let x = u16::from_le_bytes(ptr[offset .. offset+2].try_into().unwrap());
    return x
}

fn read32 (offset: usize, ptr: &Vec<u8>) -> u32 {
    let x = u32::from_le_bytes(ptr[offset .. offset+4].try_into().unwrap());
    return x
}

fn read64 (offset: usize, ptr: &Vec<u8>) -> u64 {
    let x = u64::from_le_bytes(ptr[offset .. offset+8].try_into().unwrap());
    return x
}

static ENTITY_TYPE: u16 = 1;
static VERSION: u32 = 1;
static PAYLOAD: [u8; 4] = [0u8, 1u8, 2u8, 3u8];
static MAX_FEE: u64 = 1;
static EMBEDDED_SIZE: u16 = 3;

// function to read vec of bytes
#[no_mangle]
pub extern "C" fn set_transaction(ptr_to_transaction: u64, length_transaction: u64) {
    let mut aggregate: AggregateTranction = AggregateTranction::default();
    let ptr = ptr_to_transaction as *mut u8;
    let mut buffer = Vec::new();
    buffer.resize(length_transaction as usize, 0);
    for i in 0..length_transaction as usize {
        buffer[i] = unsafe { *ptr.add(i)};
    }

    let mut offset = 0;
    aggregate.set_max_fee(read64(offset, &buffer));
    assert_eq!(MAX_FEE, aggregate.get_max_fee());
    offset += 8;

    let n = read16(offset, &buffer);
    assert_eq!(EMBEDDED_SIZE, n);
    offset += 2;
    for _ in 0..n {
        let mut embedded = EmbeddedTransaction::default();
        embedded.set_entity_type(read16(offset, &buffer));
        assert_eq!(ENTITY_TYPE, embedded.get_entity_type());
        offset += 2;
        embedded.set_version(read32(offset, &buffer));
        assert_eq!(VERSION, embedded.get_version());
        offset += 4;
        let payload_size = read16(offset, &buffer) as usize;
        assert_eq!(PAYLOAD.len(), payload_size);
        offset += 2;
        embedded.set_payload((&buffer[offset..offset+payload_size]).to_vec());
        assert_eq!(&PAYLOAD.to_vec(), embedded.get_payload());
        offset += payload_size;
        aggregate.add_embedded_transaction(embedded);
    }
}

#[test]
fn test_set_transaction() {
    // Arrange:
    let mut aggregate = AggregateTranction::default();
    aggregate.set_max_fee(MAX_FEE);
    for _ in 0..EMBEDDED_SIZE {
        let mut embedded = EmbeddedTransaction::default();
        embedded.set_entity_type(ENTITY_TYPE);
        embedded.set_version(VERSION);
        embedded.set_payload(vec![0u8, 1u8, 2u8, 3u8]);
        aggregate.add_embedded_transaction(embedded);
    }
    
    // Act:
    other_set_transaction(&aggregate);

}