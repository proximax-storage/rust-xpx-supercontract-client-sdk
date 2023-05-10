mod test_utils;

use sdk::blockchain::*;
use serde::Serialize;
use test_utils::*;

#[no_mangle]
extern "C" fn get_service_payments(return_ptr: u64) -> u64 {
    let mut buffer: Vec<u8> = Vec::new();
    let byte_id = 1u64.to_le_bytes();
    let byte_amount = 500u64.to_le_bytes();
    buffer.extend_from_slice(&byte_id);
    buffer.extend_from_slice(&byte_amount);
    buffer.extend_from_slice(&byte_id);
    buffer.extend_from_slice(&byte_amount);
    let ptr = return_ptr as *mut u8;
    unsafe {
        std::ptr::copy(buffer.as_ptr(), ptr, buffer.len());
    }
    return 2;
}
#[no_mangle]
pub extern "C" fn buffer_size() -> u64 {
    return 32;
}
#[no_mangle]
pub extern "C" fn get_block_time() -> u64 {
    return 2500;
}
#[no_mangle]
pub extern "C" fn get_block_height() -> u64 {
    return 2500;
}
#[no_mangle]
pub extern "C" fn get_contract_public_key(ptr: u64) {
    let ptr = ptr as *mut u8;
    let mut buffer: Vec<u8> = Vec::new();
    buffer = [0u8; 32].to_vec();
    unsafe {
        std::ptr::copy(buffer.as_ptr(), ptr, buffer.len());
    } 
}

// fundraising contract
struct Fund {
    due_date: u64,
    goal: u64,
    amount: u64,
}

#[derive(Serialize)]
struct Mosaic {
    mosaic_id: u64,
    amount: u64
}

impl Fund {
    pub fn get_due_date(&self) -> u64 {
        return self.due_date;
    }

    pub fn set_due_date(&mut self, due_date: u64) {
        self.due_date = due_date;
    }

    pub fn get_goal(&self) -> u64 {
        return self.goal;
    }

    pub fn set_goal(&mut self, goal: u64) {
        self.goal = goal;
    }

    pub fn get_amount(&self) -> u64 {
        return self.amount;
    }

    pub fn pay(&mut self, amount: u64) {
        self.amount = self.get_amount() + amount;

        let mut emb = EmbeddedTransaction::default();
        emb.set_entity_type(0x4154);
        emb.set_version(3);
        let receiver = sdk::blockchain::get_contract_public_key();
        let mosaic_size = 1u64.to_le_bytes();
        let msg_size = 0u64.to_le_bytes();
        let mosaic = Mosaic{ mosaic_id: 1, amount: amount };
        let mosaic_byte = bincode::serialize(&mosaic).unwrap();
        let mut payload: Vec<u8> = Vec::new();
        payload.extend_from_slice(&receiver);
        payload.extend_from_slice(&mosaic_size);
        payload.extend_from_slice(&msg_size);
        payload.extend_from_slice(&mosaic_byte);
        emb.set_payload(payload);
        let mut agg = AggregateTransaction::default();
        agg.set_max_fee(10);
        agg.add_embedded_transaction(emb);
        sdk::blockchain::set_transaction(&agg);
    }

    pub fn check_fund(&self) -> i32 {
        // calculate amount transfered to the contract
        let assets = sdk::blockchain::get_service_payments();
        let mut total_amount = 0;
        for item in assets {
            total_amount += item.amount;
        }
        assert_eq!(total_amount, 1000);

        // check fund goal and due date,  if yes set transaction
        if total_amount >= self.get_goal() && get_block_time() <= self.get_due_date() {
            let mut emb = EmbeddedTransaction::default();
            emb.set_entity_type(0x4154);
            emb.set_version(3);
            let receiver = [0u8; 32];
            let mosaic_size = 1u64.to_le_bytes();
            let msg_size = 0u64.to_le_bytes();
            let mosaic = Mosaic{ mosaic_id: 1, amount: self.amount };
            let mosaic_byte = bincode::serialize(&mosaic).unwrap();
            let mut payload: Vec<u8> = Vec::new();
            payload.extend_from_slice(&receiver);
            payload.extend_from_slice(&mosaic_size);
            payload.extend_from_slice(&msg_size);
            payload.extend_from_slice(&mosaic_byte);
            assert_eq!(64, payload.len());
            emb.set_payload(payload);
            let mut agg = AggregateTransaction::default();
            agg.set_max_fee(10);
            agg.add_embedded_transaction(emb);
            sdk::blockchain::set_transaction(&agg);
            return 1;
        }else{
            return 0;
        }
    }
}

#[test]
fn test_fund() {
    // Arrange:
    let receiver = [0u8; 32];
    let mosaic_size = 1u64.to_le_bytes();
    let msg_size = 0u64.to_le_bytes();
    let mosaic = Mosaic{ mosaic_id: 1, amount: 1000 };
    let mosaic_byte = bincode::serialize(&mosaic).unwrap();
    let mut payload: Vec<u8> = Vec::new();
    payload.extend_from_slice(&receiver);
    payload.extend_from_slice(&mosaic_size);
    payload.extend_from_slice(&msg_size);
    payload.extend_from_slice(&mosaic_byte);
    unsafe { 
        // expected result
        ENTITY_TYPE = 0x4154;
        VERSION = 3;
        PAYLOAD = payload;
        MAX_FEE = 10;
        EMBEDDED_SIZE = 1;
    }
    let mut fund = Fund {
        due_date: 0,
        goal: 0,
        amount: 0,
    };

    // Act:
    fund.set_due_date(2500);
    fund.set_goal(1000);
    fund.pay(1000);

    let result = fund.check_fund();

    // Assert:
    assert_eq!(1, result);

}