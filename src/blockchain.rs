use std::{
    collections::HashMap,
    io::{Error, Result},
};

mod import_function {
    #[allow(dead_code)]
    // I changed the memory addresses (ptr_to_something) to u64 becuase my machine is a 64-bit system
    extern "C" {
        pub fn get_block_height() -> u64;
        pub fn get_block_hash(ptr_to_write: u64) -> u64;
        pub fn get_block_time() -> u64;
        pub fn get_block_generation_time() -> u64;
        pub fn get_transaction_hash(ptr_to_write: u64) -> u64;
        pub fn get_caller_public_key(ptr_to_write: u64) -> u64;
        pub fn get_sc_prepayment() -> u64;
        pub fn get_sm_prepayment() -> u64;
        pub fn get_call_params(return_ptr: u64) -> u64;
        pub fn get_service_payment() -> u64;
        pub fn add_transaction(
            ptr_to_write: u64,
            ptr_to_name: u64,
            length_name: u64,
            ptr_to_parameters: u64,
            length_parameters: u64,
        ) -> u64;
        pub fn set_transaction(
            ptr_to_transaction: u64,
            length_transaction: u64,
        ) -> u64;
        pub fn get_transaction_block_height(ptr: u64) -> u64;
        pub fn get_response_transaction_hash(ptr_to_read: u64, ptr_to_write: u64) -> u64;
        pub fn get_transaction_content(ptr_to_read: u64, ptr_to_write: u64) -> u64;
    }
}

pub unsafe fn get_block_height() -> u64 {
    import_function::get_block_height()
}

pub unsafe fn get_block_hash() -> Result<[u8; 32]> {
    let mut hash_buffer = [0; 32];
    let ret = import_function::get_block_hash(hash_buffer.as_mut_ptr() as u64);
    if ret != 32 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Failed to retrieve a valid hash",
        ));
    }
    return Ok(hash_buffer);
}

pub unsafe fn get_block_time() -> u64 {
    import_function::get_block_time()
}

pub unsafe fn get_block_generation_time() -> u64 {
    import_function::get_block_generation_time()
}

pub unsafe fn get_transaction_hash() -> Result<[u8; 32]> {
    let mut hash_buffer = [0; 32];
    let ret = import_function::get_transaction_hash(hash_buffer.as_mut_ptr() as u64);
    if ret != 32 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Failed to retrieve a valid hash",
        ));
    }
    return Ok(hash_buffer);
}

pub unsafe fn get_caller_public_key() -> Result<[u8; 32]> {
    let mut hash_buffer = [0; 32];
    let ret = import_function::get_caller_public_key(hash_buffer.as_mut_ptr() as u64);
    if ret != 32 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Failed to retrieve a valid hash",
        ));
    }
    return Ok(hash_buffer);
}

pub unsafe fn get_sc_prepayment() -> u64 {
    import_function::get_sc_prepayment()
}

pub unsafe fn get_sm_prepayment() -> u64 {
    import_function::get_sm_prepayment()
}

#[allow(unused_variables)]
pub unsafe fn add_transaction<T: serde::ser::Serialize>(
    tx_name: String,
    param: HashMap<String, T>,
) -> Result<[u8; 32]> {
    // let tx_name = tx_name.as_bytes();
    // let param = serde_json::to_string(&param)?; // I assume the Blockchain will recieve it in JSON format like the common POST method
    // let param = Vec::from(param);
    // let ret = add_transaction(
    //     self.buffer.as_mut_ptr() as u64,
    //     tx_name.as_ptr() as u64,
    //     tx_name.len() as u64,
    //     param.as_ptr() as u64,
    //     param.len() as u64,
    // );
    // return Ok(String::from_utf8_unchecked(
    //     self.buffer[..ret as usize].to_vec(),
    // ));
    todo!()
}

// I assume it will be the same as POST method body (recieving it in JSON format)
pub unsafe fn get_call_params() -> Result<HashMap<String, String>> {
    let mut hash_buffer = [0; 32];
    let len = import_function::get_call_params(hash_buffer.as_mut_ptr() as u64);
    let serialized_json = String::from_utf8_unchecked(hash_buffer[..len as usize].to_vec());
    return Ok(serde_json::from_str(&serialized_json)?);
}

pub unsafe fn get_service_payment() -> u64 {
    import_function::get_service_payment()
}

pub unsafe fn get_transaction_block_height(hash: [u8; 32]) -> u64 {
    return import_function::get_transaction_block_height(hash.as_ptr() as u64);
}

pub unsafe fn get_response_transaction_hash(hash: [u8; 32]) -> Option<[u8; 32]> {
    let mut hash_buffer = [0; 32];
    let ret = import_function::get_response_transaction_hash(
        hash.as_ptr() as u64,
        hash_buffer.as_mut_ptr() as u64,
    );
    if ret == 0 {
        return None;
    }
    return Some(hash_buffer);
}

// Is this JSON in String?
pub unsafe fn get_transaction_content(hash: String) -> String {
    let mut hash_buffer = [0; 32];
    let hash = hash.as_bytes();
    let ret = import_function::get_transaction_content(
        hash.as_ptr() as u64,
        hash_buffer.as_mut_ptr() as u64,
    );
    return String::from_utf8_unchecked(hash_buffer[..ret as usize].to_vec());
}

struct EmbeddedTransaction {
    entity_version: u8,
    version: u8,
    payload: Vec<u8>,
}

impl EmbeddedTransaction {
    fn get_entity_version(&self) -> u8 {
        return self.entity_version;
    }

    fn set_entity_version(&mut self, entity_version: u8) {
        self.entity_version = entity_version;
    }

    fn get_version(&self) -> u8 {
        return self.version;
    }

    fn set_version(&mut self, version: u8) {
        self.version = version;
    }

    fn get_payload(&self) -> &Vec<u8> {
        return &self.payload
    }

    fn set_payload(&mut self, payload: Vec<u8>) {
        self.payload = payload;
    }

}

pub struct AggregateTranction {
    max_fee: u64,
    embedded_transactions: Vec<EmbeddedTransaction>,
}

impl AggregateTranction {
    fn get_max_fee(&self) -> u64 {
        return self.max_fee;
    }

    fn set_max_fee(&mut self, max_fee: u64) {
        self.max_fee = max_fee;
    }

    fn get_embedded_transactions(&self) -> &Vec<EmbeddedTransaction> {
        &self.embedded_transactions
    }

}
pub unsafe fn set_transaction(transaction: &AggregateTranction) -> Vec<u8> {
    let mut bytes = transaction.get_max_fee().to_le_bytes().to_vec();
    bytes.extend_from_slice(&transaction.get_embedded_transactions().len().to_le_bytes());
    for value in transaction.get_embedded_transactions().iter() {
        bytes.extend_from_slice(&value.entity_version.to_le_bytes());
        bytes.extend_from_slice(&value.version.to_le_bytes());
        bytes.extend_from_slice(&value.payload.len().to_le_bytes());
        bytes.extend_from_slice(&value.payload);
    }
    return bytes
}