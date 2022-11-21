use sdk::file::FileWriter;
use serial_test::serial;
use std::{cmp::min, io::Write};

static mut BUF: [u8; 0] = [];
static mut I: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn write_file_stream(
    _identifier: i64,
    ptr_to_buffer: u64,
    length_buffer: u64,
) -> i64 {
    let ptr = ptr_to_buffer as *mut u8;
    let mut ret = 0;
    let offset = I;
    for x in I..min(BUF.len(), I + length_buffer as usize) {
        ret += 1;
        BUF[x] = *ptr.add(x - offset);
        I = x + 1;
    }
    return ret;
}

#[no_mangle]
pub unsafe extern "C" fn open_file(
    _ptr_to_path: u64,
    _length_path: u64,
    _ptr_to_mode: u64,
    _length_mode: u64,
) -> i64 {
    return 10000000;
}

#[no_mangle]
pub unsafe extern "C" fn close_file(_identifier: i64) -> u32 {
    return 1;
}

#[no_mangle]
pub unsafe extern "C" fn buffer_size() -> u32 {
    return 16 * 1024;
}

#[no_mangle]
pub unsafe extern "C" fn flush(_identifier: i64) -> u32 {
    return 1;
}

#[test]
#[serial]
fn test_write_buffer_1gb() {
    let mut file = unsafe { FileWriter::new("./".to_string()).unwrap() };
    let mut buffer = vec![128; 1073741824];
    match file.write(&mut buffer) {
        Ok(_) => panic!("Should panic"),
        Err(_) => (),
    };
    unsafe {
        BUF = [];
        I = 0;
    };
}

#[test]
#[serial]
fn test_write_buffer_100mb() {
    let mut file = unsafe { FileWriter::new("./".to_string()).unwrap() };
    let mut buffer = vec![128; 1048576 * 100];
    match file.write(&mut buffer) {
        Ok(_) => panic!("Should panic"),
        Err(_) => (),
    };
    unsafe {
        BUF = [];
        I = 0;
    };
}

#[test]
#[serial]
fn test_write_buffer_100kb() {
    let mut file = unsafe { FileWriter::new("./".to_string()).unwrap() };
    let mut buffer = vec![128; 1024 * 100];
    match file.write(&mut buffer) {
        Ok(_) => panic!("Should panic"),
        Err(_) => (),
    };
    unsafe {
        BUF = [];
        I = 0;
    };
}

#[test]
#[serial]
fn test_write_buffer_empty() {
    let mut file = unsafe { FileWriter::new("./".to_string()).unwrap() };
    let mut buffer = vec![];
    let ret = file.write(&mut buffer).unwrap();
    assert_eq!(ret, 0);
    let expected: [u8; 0] = [];
    assert_eq!(unsafe { BUF }, expected);
    unsafe {
        I = 0;
    };
}
