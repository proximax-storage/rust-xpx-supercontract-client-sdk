mod import_function {
    extern "C" {
        pub fn get_block_height() -> u64;
        pub fn get_block_hash(ptr_to_write: u64);
        pub fn get_block_time() -> u64;
        pub fn get_block_generation_time() -> u64;
        pub fn get_transaction_hash(ptr_to_write: u64);
        pub fn get_caller_public_key(ptr_to_write: u64);
        pub fn get_contract_public_key(ptr_to_write: u64);
        pub fn get_execution_payment() -> u64;
        pub fn get_download_payment() -> u64;
        pub fn get_call_params(return_ptr: u64) -> u64;
        pub fn get_call_params_length() -> u64;
        pub fn get_service_payments(return_ptr: u64) -> u64;
        pub fn set_transaction(ptr_to_transaction: u64, length_transaction: u64);
        pub fn buffer_size() -> u64;
    }
}

pub fn get_block_height() -> u64 {
    unsafe {
        import_function::get_block_height()
    }
}

pub fn get_block_hash() -> [u8; 32] {
    let mut hash_buffer = [0; 32];

    unsafe {
        import_function::get_block_hash(hash_buffer.as_mut_ptr() as u64);
    }
    return hash_buffer;
}

pub fn get_block_time() -> u64 {
    unsafe {
        import_function::get_block_time()
    }
}

pub unsafe fn get_block_generation_time() -> u64 {
    import_function::get_block_generation_time()
}

pub fn get_transaction_hash() -> [u8; 32] {
    let mut hash_buffer = [0; 32];

    unsafe {
        import_function::get_transaction_hash(hash_buffer.as_mut_ptr() as u64);
    };
    return hash_buffer;
}

pub fn get_caller_public_key() -> [u8; 32] {
    let mut hash_buffer = [0; 32];

    unsafe {
        import_function::get_caller_public_key(hash_buffer.as_mut_ptr() as u64);
    };

    return hash_buffer;
}

pub fn get_contract_public_key() -> [u8; 32] {
    let mut hash_buffer = [0; 32];

    unsafe {
        import_function::get_contract_public_key(hash_buffer.as_mut_ptr() as u64);
    };

    return hash_buffer;
}

pub fn get_execution_payment() -> u64 {
    unsafe {
        import_function::get_execution_payment()
    }
}

pub fn get_download_payment() -> u64 {
    unsafe {
        import_function::get_download_payment()
    }
}

pub fn get_call_params() -> Vec<u8> {

    let params_length = unsafe {
        import_function::get_call_params_length()
    };

    let mut buffer: Vec<u8> = vec![0; params_length as usize];

    unsafe {
        import_function::get_call_params(buffer.as_mut_ptr() as u64);
    }

    return buffer;
}

#[derive(Default, Clone)]
pub struct ServicePayment {
    pub mosaic_id: u64,
    pub amount: u64
}

pub fn get_service_payments() -> Vec<ServicePayment> {
    let buffer_size = unsafe {
        import_function::buffer_size()
    };
    let mut buffer: Vec<u8> = vec![0; buffer_size as usize];

    let payments_num = unsafe {
        import_function::get_service_payments(buffer.as_mut_ptr() as u64)
    };

    let mut payments: Vec<ServicePayment> = Vec::with_capacity(payments_num as usize);

    let mut buffer_tail = buffer.as_slice();
    for _ in 0..payments_num {
        let mut payment: ServicePayment = Default::default();
        {
            let (number_bytes, tail) = buffer_tail.split_at(std::mem::size_of::<u64>());
            buffer_tail = tail;
            payment.mosaic_id = u64::from_le_bytes(number_bytes.try_into().unwrap());
        }
        {
            let (number_bytes, tail) = buffer_tail.split_at(std::mem::size_of::<u64>());
            buffer_tail = tail;
            payment.amount = u64::from_le_bytes(number_bytes.try_into().unwrap());
        }
        payments.push(payment);
    }

    return payments;
}

#[derive(Default, Clone)]
pub struct EmbeddedTransaction {
    entity_type: u16,
    version: u32,
    payload: Vec<u8>,
}

impl EmbeddedTransaction {
    pub fn get_entity_type(&self) -> u16 {
        return self.entity_type;
    }

    pub fn set_entity_type(&mut self, entity_type: u16) {
        self.entity_type = entity_type;
    }

    pub fn get_version(&self) -> u32 {
        return self.version;
    }

    pub fn set_version(&mut self, version: u32) {
        self.version = version;
    }

    pub fn get_payload(&self) -> &Vec<u8> {
        return &self.payload
    }

    pub fn set_payload(&mut self, payload: Vec<u8>) {
        self.payload = payload;
    }

}

#[derive(Default)]
pub struct AggregateTransaction {
    max_fee: u64,
    embedded_transactions: Vec<EmbeddedTransaction>,
}

impl AggregateTransaction {
    pub fn get_max_fee(&self) -> u64 {
        return self.max_fee;
    }

    pub fn set_max_fee(&mut self, max_fee: u64) {
        self.max_fee = max_fee;
    }

    pub fn get_embedded_transactions(&self) -> &Vec<EmbeddedTransaction> {
        &self.embedded_transactions
    }

    pub fn add_embedded_transaction(&mut self, new_embedded_transaction: EmbeddedTransaction) {
        self.embedded_transactions.push(new_embedded_transaction);
    }

}

pub fn set_transaction(transaction: &AggregateTransaction) {
    let mut bytes = transaction.get_max_fee().to_le_bytes().to_vec();
    let embedded_transaction_size = transaction.get_embedded_transactions().len() as u16;
    bytes.extend_from_slice(&embedded_transaction_size.to_le_bytes());
    for value in transaction.get_embedded_transactions().iter() {
        bytes.extend_from_slice(&value.entity_type.to_le_bytes());
        bytes.extend_from_slice(&value.version.to_le_bytes());
        let payload_size = value.payload.len() as u16;
        bytes.extend_from_slice(&payload_size.to_le_bytes());
        bytes.extend_from_slice(&value.payload);
    }

    unsafe {
        import_function::set_transaction(bytes.as_ptr() as u64, bytes.len() as u64);
    }
}