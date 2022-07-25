use std::{collections::HashMap, io::Result};

use super::file::buffer_size;

static HASH_SIZE: usize = 32;

extern "C" {
    // Other import functions than these are trivial, so I assume we don't need to wrap them
    fn get_block_hash(ptr_to_write: u32) -> u32;
    fn get_transaction_hash(ptr_to_write: u32) -> u32;
    fn get_caller_public_key(ptr_to_write: u32) -> u32;
    fn add_transaction(
        ptr_to_write: u32,
        ptr_to_name: u32,
        length_name: u32,
        ptr_to_parameters: u32,
        length_parameters: u32,
    ) -> u32;
    fn get_transaction_block_height(ptr: u32) -> u64;
    fn get_response_transaction_hash(ptr_to_read: u32, ptr_to_write: u32) -> u32;
    fn get_transaction_content(ptr_to_read: u32, ptr_to_write: u32) -> u32;
    // fn buffer_size() -> u32;
    fn get_call_params(return_ptr: u32) -> u32;
}

pub struct BlockchainInterface {
    buffer: Vec<u8>,
}

impl BlockchainInterface {
    pub unsafe fn new() -> Self {
        let mut buffer = Vec::new();
        for _ in 0..buffer_size() {
            buffer.push(0);
        }
        Self { buffer }
    }

    pub unsafe fn get_block_hash(&mut self) -> String {
        let ret = get_block_hash(self.buffer.as_mut_ptr() as u32);
        return String::from_utf8_unchecked(self.buffer[..ret as usize].to_vec());
    }

    pub unsafe fn get_transaction_hash(&mut self) -> String {
        let ret = get_transaction_hash(self.buffer.as_mut_ptr() as u32);
        return String::from_utf8_unchecked(self.buffer[..ret as usize].to_vec());
    }

    pub unsafe fn get_caller_public_key(&mut self) -> String {
        let ret = get_caller_public_key(self.buffer.as_mut_ptr() as u32);
        return String::from_utf8_unchecked(self.buffer[..ret as usize].to_vec());
    }

    pub unsafe fn add_transaction<T: serde::ser::Serialize>(
        &mut self,
        tx_name: String,
        param: HashMap<String, T>,
    ) -> Result<String> {
        let tx_name = tx_name.as_bytes();
        let param = serde_json::to_string(&param)?; // I assume the Blockchain will recieve it in JSON format like the common POST method
        let param = Vec::from(param);
        let ret = add_transaction(
            self.buffer.as_mut_ptr() as u32,
            tx_name.as_ptr() as u32,
            tx_name.len() as u32,
            param.as_ptr() as u32,
            param.len() as u32,
        );
        return Ok(String::from_utf8_unchecked(
            self.buffer[..ret as usize].to_vec(),
        ));
    }

    pub unsafe fn get_transaction_block_height(&mut self, hash: String) -> u64 {
        let hash = hash.as_bytes();
        return get_transaction_block_height(hash.as_ptr() as u32);
    }

    // I assume the returned buffer contains multiple hashes?
    pub unsafe fn get_response_transaction_hash(&mut self, hash: String) -> Vec<String> {
        let hash = hash.as_bytes();
        let mut buf = Vec::new();
        get_response_transaction_hash(hash.as_ptr() as u32, buf.as_mut_ptr() as u32); // The return value is the total length of the buffer written, so I'm not using it
        let buf = buf.chunks(HASH_SIZE);
        let ret = buf.map(|x| String::from_utf8_unchecked(x.into())).collect();
        return ret;
    }

    // Is this JSON in String?
    pub unsafe fn get_transaction_content(&mut self, hash: String) -> String {
        let hash = hash.as_bytes();
        let ret = get_transaction_content(hash.as_ptr() as u32, self.buffer.as_mut_ptr() as u32);
        return String::from_utf8_unchecked(self.buffer[..ret as usize].to_vec());
    }

    // I assume it will be the same as POST method body (recieving it in JSON format)
    pub unsafe fn get_call_params(&mut self) -> Result<HashMap<String, String>> {
        let len = get_call_params(self.buffer.as_mut_ptr() as u32);
        let serialized_json = String::from_utf8_unchecked(self.buffer[..len as usize].to_vec());
        return Ok(serde_json::from_str(&serialized_json)?);
    }
}
