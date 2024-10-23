use std::io::{BufReader, Read};

use blockchain::print_log;
use serde::{Deserialize, Serialize};  // this is the external crate serde

pub mod blockchain;
pub mod dir_iterator;
pub mod file;
pub mod filesystem;
pub mod internet;

// here i use external crate for the struct to deserialize json into struct
#[derive(Debug, Serialize, Deserialize)]
struct Mosaic {
    id: Vec<u32>,
    amount: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Account {
    address: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Root {
    account: Account,
}

// read rest endpoint, deserialize into struct, and print account address
#[no_mangle]
pub unsafe extern "C" fn check() -> u32 {

    let baseurl = "http://127.0.0.1:3000/account/";
    
    // Read participants
    {
        let internet = internet::Internet::new(&format!("{}{}", baseurl, "SD2L2LRSBZUMYV2T34C4UXOIAAWX4TWQSQGBPMQO"), true).unwrap();
        let mut reader = BufReader::with_capacity(1024, internet);
        let mut internet_buffer = Vec::new();
        reader.read_to_end(&mut internet_buffer).unwrap();
        let result: Root = serde_json::from_slice(&internet_buffer).unwrap(); // here is also the serdejson 
        
        print_log(&result.account.address);
    }
    return 0;

}