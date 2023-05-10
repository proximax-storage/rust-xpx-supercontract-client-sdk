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
    let ptr = return_ptr as *mut u8;
    unsafe {
        std::ptr::copy(buffer.as_ptr(), ptr, buffer.len());
    }
    return 1;
}
#[no_mangle]
pub extern "C" fn buffer_size() -> u64 {
    return 16;
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

struct Loan {
    due_date: u64,
    loan_amount: u64,
    interest: u64,
    paid: u64,
    mortgage: u64, // other asset deposit
}

#[derive(Serialize)]
struct Mosaic {
    mosaic_id: u64,
    amount: u64,
}

impl Loan {
    fn new(mortgage: u64) -> Loan {
        Loan {
            due_date: get_block_height() + 2000,
            loan_amount: mortgage*2, //1000
            interest: mortgage*2*10/100, //100
            paid: 0,
            mortgage,
        }
    }

    fn verify(&self) -> i32 {
        // calculate amount transfered to the contract
        let deposit = sdk::blockchain::get_service_payments();
        let mut total_amount = 0;
        for item in deposit {
            total_amount += item.amount;
        }

        if total_amount >= self.mortgage {
            let mut emb = EmbeddedTransaction::default();
            emb.set_entity_type(0x4154);
            emb.set_version(3);
            let receiver = sdk::blockchain::get_contract_public_key();
            let mosaic_size = 1u64.to_le_bytes();
            let msg_size = 0u64.to_le_bytes();
            let mosaic = Mosaic {
                mosaic_id: 1,
                amount: self.loan_amount,
            };
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
            return 1;
        } else {
            return 0;
        }
    }

    fn check_balance(&self) -> u64 {
        return self.loan_amount + self.interest - self.paid;
    }

    fn pay(&mut self, amount: u64) -> i32 {
        self.paid = self.paid + amount;

        // fine for late payment
        if get_block_height() > self.due_date && self.loan_amount + self.interest > 0 {
            self.interest = self.interest + self.mortgage * 10 / 100;
        }

        // finish repay loan return mortgage
        if self.loan_amount + self.interest - self.paid == 0 {
            let mut emb = EmbeddedTransaction::default();
            emb.set_entity_type(0x4154);
            emb.set_version(3);
            let receiver = sdk::blockchain::get_contract_public_key();
            let mosaic_size = 1u64.to_le_bytes();
            let msg_size = 0u64.to_le_bytes();
            let mosaic = Mosaic {
                mosaic_id: 2,
                amount: self.mortgage,
            };
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
            return 1;
        }
        return 0;
    }
}

fn quote(mortgage: u64) -> u64 {
    return mortgage * 2;
}

#[test]
fn test_loan() {
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
    let mut payload2: Vec<u8> = Vec::new();
    let mosaic2 = Mosaic{ mosaic_id: 2, amount: 500 };
    let mosaic_byte2 = bincode::serialize(&mosaic2).unwrap();
    payload2.extend_from_slice(&receiver);
    payload2.extend_from_slice(&mosaic_size);
    payload2.extend_from_slice(&msg_size);
    payload2.extend_from_slice(&mosaic_byte2);
    // expected result
    unsafe { 
        ENTITY_TYPE = 0x4154;
        VERSION = 3;
        PAYLOAD = payload;
        MAX_FEE = 10;
        EMBEDDED_SIZE = 1;
    }
    

    // Act:
    let expected_loan = quote(500);
    let mut loan = Loan::new(500);
    let success = loan.verify();
    
    let mut balance = loan.check_balance();
    assert_eq!(balance, 1100);
    
    unsafe {
        PAYLOAD = payload2
    };

    let pay_status1 = loan.pay(550);
    balance = loan.check_balance();
    
    let pay_status2 = loan.pay(550);
    
    // Assert:
    assert_eq!(expected_loan, 1000);
    assert_eq!(success, 1);
    assert_eq!(pay_status1, 0);
    assert_eq!(balance, 550);
    assert_eq!(pay_status2, 1);

}