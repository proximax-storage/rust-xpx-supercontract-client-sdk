use sdk::blockchain::{AggregateTranction, EmbeddedTransaction};

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

// function to read vec of bytes
pub fn set_transaction(ptr_to_transaction: u64, length_transaction: u64) -> AggregateTranction {
    let mut aggregate: AggregateTranction = AggregateTranction::default();
    let ptr = ptr_to_transaction as *mut u8;
    let mut buffer = Vec::new();
    buffer.resize(length_transaction as usize, 0);
    for i in 0..length_transaction as usize {
        buffer[i] = unsafe { *ptr.add(i)};
    }

    let mut offset = 0;
    aggregate.set_max_fee(read64(offset, &buffer));
    offset += 8;

    let n = read16(offset, &buffer);
    offset += 2;
    for _ in 0..n {
        let mut embedded = EmbeddedTransaction::default();
        embedded.set_entity_type(read16(offset, &buffer));
        offset += 2;
        embedded.set_version(read32(offset, &buffer));
        offset += 4;
        let payload_size = read16(offset, &buffer) as usize;
        offset += 2;
        // println!("{}", payload_size);
        embedded.set_payload((&buffer[offset..offset+payload_size]).to_vec());
        offset += payload_size;
        aggregate.add_embedded_transaction(embedded);
    }
    return aggregate
}

#[test]
fn test_set_transaction() {
    // Arrange:
    let entity_type = 1;
    let version = 1;
    let payload = vec![0u8, 1u8, 2u8, 3u8];
    let max_fee = 1;
    let embedded_size = 3;
    let mut expected_aggregate = AggregateTranction::default();
    expected_aggregate.set_max_fee(max_fee);
    for _ in 0..embedded_size {
        let mut expected_embedded = EmbeddedTransaction::default();
        expected_embedded.set_entity_type(entity_type);
        expected_embedded.set_version(version);
        expected_embedded.set_payload(vec![0u8, 1u8, 2u8, 3u8]);
        expected_aggregate.add_embedded_transaction(expected_embedded);
    }
    
    // Act:
    // create bytes vec (same as in blockchain.rs set_transaction fn)
    let mut bytes = max_fee.to_le_bytes().to_vec();
    let embedded_transaction_size = embedded_size as u16;
    bytes.extend_from_slice(&embedded_transaction_size.to_le_bytes());
    for _ in 0..embedded_size {
        bytes.extend_from_slice(&entity_type.to_le_bytes());
        bytes.extend_from_slice(&version.to_le_bytes());
        let payload_size = payload.len() as u16;
        bytes.extend_from_slice(&payload_size.to_le_bytes());
        bytes.extend_from_slice(&payload);
    }
    
    let bytes_ptr_casted_u64 = bytes.as_ptr() as u64;
    let length_of_txn = bytes.len() as u64;

    // retrive data from ptr
    let actual_aggregate = set_transaction(bytes_ptr_casted_u64, length_of_txn);
    
    // Assert:
    assert_eq!(expected_aggregate.get_max_fee() , actual_aggregate.get_max_fee());
    assert_eq!(expected_aggregate.get_embedded_transactions().len() , actual_aggregate.get_embedded_transactions().len());
    for i in 0..expected_aggregate.get_embedded_transactions().len() {
        assert_eq!(expected_aggregate.get_embedded_transactions()[i].get_entity_type(), actual_aggregate.get_embedded_transactions()[i].get_entity_type());
        assert_eq!(expected_aggregate.get_embedded_transactions()[i].get_version(), actual_aggregate.get_embedded_transactions()[i].get_version());
        assert_eq!(expected_aggregate.get_embedded_transactions()[i].get_payload().len(), actual_aggregate.get_embedded_transactions()[i].get_payload().len());
        assert_eq!(expected_aggregate.get_embedded_transactions()[i].get_payload(), actual_aggregate.get_embedded_transactions()[i].get_payload());
    }
}