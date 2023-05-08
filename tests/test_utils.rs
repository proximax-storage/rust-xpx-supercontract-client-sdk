use sdk::blockchain::{AggregateTransaction, EmbeddedTransaction};

pub fn read16 (offset: usize, ptr: &Vec<u8>) -> u16 {
    let x = u16::from_le_bytes(ptr[offset .. offset+2].try_into().unwrap());
    return x
}

pub fn read32 (offset: usize, ptr: &Vec<u8>) -> u32 {
    let x = u32::from_le_bytes(ptr[offset .. offset+4].try_into().unwrap());
    return x
}

pub fn read64 (offset: usize, ptr: &Vec<u8>) -> u64 {
    let x = u64::from_le_bytes(ptr[offset .. offset+8].try_into().unwrap());
    return x
}

pub static mut ENTITY_TYPE: u16 = 0;
pub static mut VERSION: u32 = 0;
pub static mut PAYLOAD: Vec<u8> = Vec::new();
pub static mut MAX_FEE: u64 = 0;
pub static mut EMBEDDED_SIZE: u16 = 0;

#[no_mangle]
pub extern "C" fn set_transaction(ptr_to_transaction: u64, length_transaction: u64) {
    let mut aggregate: AggregateTransaction = AggregateTransaction::default();
    let ptr = ptr_to_transaction as *mut u8;
    let mut buffer = Vec::new();
    buffer.resize(length_transaction as usize, 0);
    for i in 0..length_transaction as usize {
        buffer[i] = unsafe { *ptr.add(i)};
    }

    let mut offset = 0;
    aggregate.set_max_fee(read64(offset, &buffer));
    unsafe { assert_eq!(MAX_FEE, aggregate.get_max_fee()) };
    offset += 8;

    let n = read16(offset, &buffer);
    unsafe { assert_eq!(EMBEDDED_SIZE, n) };
    offset += 2;
    for _ in 0..n {
        let mut embedded = EmbeddedTransaction::default();
        embedded.set_entity_type(read16(offset, &buffer));
        unsafe { assert_eq!(ENTITY_TYPE, embedded.get_entity_type()) };
        offset += 2;
        embedded.set_version(read32(offset, &buffer));
        unsafe { assert_eq!(VERSION, embedded.get_version()) };
        offset += 4;
        let payload_size = read16(offset, &buffer) as usize;
        unsafe { assert_eq!(PAYLOAD.len(), payload_size) };
        offset += 2;
        embedded.set_payload((&buffer[offset..offset+payload_size]).to_vec());
        unsafe { assert_eq!(&PAYLOAD, embedded.get_payload()) };
        offset += payload_size;
        aggregate.add_embedded_transaction(embedded);
    }
}