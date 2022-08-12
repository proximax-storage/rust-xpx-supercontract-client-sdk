use std::{
    collections::HashMap,
    io::{Error, Result},
};

#[allow(dead_code)]
// I changed the memory addresses (ptr_to_something) to u64 becuase my machine is a 64-bit system
extern "C" {
    fn get_block_height() -> u64;
    fn get_block_hash(ptr_to_write: u64) -> u64;
    fn get_block_time() -> u64;
    fn get_block_generation_time() -> u64;
    fn get_transaction_hash(ptr_to_write: u64) -> u64;
    fn get_caller_public_key(ptr_to_write: u64) -> u64;
    fn get_sc_prepayment() -> u64;
    fn get_sm_prepayment() -> u64;
    fn get_call_params(return_ptr: u64) -> u64;
    fn get_service_payment() -> u64;
    fn add_transaction(
        ptr_to_write: u64,
        ptr_to_name: u64,
        length_name: u64,
        ptr_to_parameters: u64,
        length_parameters: u64,
    ) -> u64;
    fn get_transaction_block_height(ptr: u64) -> u64;
    fn get_response_transaction_hash(ptr_to_read: u64, ptr_to_write: u64) -> u64;
    fn get_transaction_content(ptr_to_read: u64, ptr_to_write: u64) -> u64;
}

pub struct BlockchainInterface {
    hash_buffer: [u8; 32],
}

impl BlockchainInterface {
    pub unsafe fn new() -> Self {
        let buffer = [0; 32];
        Self {
            hash_buffer: buffer,
        }
    }

    pub unsafe fn get_block_height(&self) -> u64 {
        get_block_height()
    }

    pub unsafe fn get_block_hash(&mut self) -> Result<[u8; 32]> {
        let ret = get_block_hash(self.hash_buffer.as_mut_ptr() as u64);
        if ret != 32 {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Failed to retrieve a valid hash",
            ));
        }
        return Ok(self.hash_buffer);
    }

    pub unsafe fn get_block_time(&self) -> u64 {
        get_block_time()
    }

    pub unsafe fn get_block_generation_time(&self) -> u64 {
        get_block_generation_time()
    }

    pub unsafe fn get_transaction_hash(&mut self) -> Result<[u8; 32]> {
        let ret = get_transaction_hash(self.hash_buffer.as_mut_ptr() as u64);
        if ret != 32 {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Failed to retrieve a valid hash",
            ));
        }
        return Ok(self.hash_buffer);
    }

    pub unsafe fn get_caller_public_key(&mut self) -> Result<[u8; 32]> {
        let ret = get_caller_public_key(self.hash_buffer.as_mut_ptr() as u64);
        if ret != 32 {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Failed to retrieve a valid hash",
            ));
        }
        return Ok(self.hash_buffer);
    }

    pub unsafe fn get_sc_prepayment(&self) -> u64 {
        get_sc_prepayment()
    }

    pub unsafe fn get_sm_prepayment(&self) -> u64 {
        get_sm_prepayment()
    }

    #[allow(unused_variables)]
    pub unsafe fn add_transaction<T: serde::ser::Serialize>(
        &mut self,
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
    pub unsafe fn get_call_params(&mut self) -> Result<HashMap<String, String>> {
        let len = get_call_params(self.hash_buffer.as_mut_ptr() as u64);
        let serialized_json =
            String::from_utf8_unchecked(self.hash_buffer[..len as usize].to_vec());
        return Ok(serde_json::from_str(&serialized_json)?);
    }

    pub unsafe fn get_service_payment(&self) -> u64 {
        get_service_payment()
    }

    pub unsafe fn get_transaction_block_height(&mut self, hash: [u8; 32]) -> u64 {
        return get_transaction_block_height(hash.as_ptr() as u64);
    }

    pub unsafe fn get_response_transaction_hash(&mut self, hash: [u8; 32]) -> Option<[u8; 32]> {
        let ret = get_response_transaction_hash(
            hash.as_ptr() as u64,
            self.hash_buffer.as_mut_ptr() as u64,
        );
        if ret == 0 {
            return None;
        }
        return Some(self.hash_buffer);
    }

    // Is this JSON in String?
    pub unsafe fn get_transaction_content(&mut self, hash: String) -> String {
        let hash = hash.as_bytes();
        let ret =
            get_transaction_content(hash.as_ptr() as u64, self.hash_buffer.as_mut_ptr() as u64);
        return String::from_utf8_unchecked(self.hash_buffer[..ret as usize].to_vec());
    }
}
