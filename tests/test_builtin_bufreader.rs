use sdk::file::FileReader;
use serial_test::serial;
use std::{
    cmp::min,
    io::{BufRead, BufReader, Read},
};

static mut FILE: [u8; 16384 * 1024] = [99; 16384 * 1024];
static mut POINTER: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn read_file_stream(_identifier: i64, ptr_to_write: u64) -> u64 {
    let ptr = ptr_to_write as *mut u8;
    let mut ret = 0;
    let offset = POINTER;
    for x in POINTER..min(16384 + POINTER, FILE.len()) {
        ret += 1;
        *ptr.add(x - offset) = FILE[x];
        POINTER = x + 1;
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
pub unsafe extern "C" fn read_from_internet(_identifier: i64, ptr_to_write: u64) -> u64 {
    let ptr = ptr_to_write as *mut u8;
    let mut ret = 0;
    let offset = POINTER;
    for x in POINTER..min(16384 + POINTER, FILE.len()) {
        ret += 1;
        *ptr.add(x - offset) = FILE[x];
        POINTER = x + 1;
    }
    return ret;
}

#[no_mangle]
pub unsafe extern "C" fn open_connection(_ptr_to_url: u64, _length_url: u64) -> i64 {
    return 10000000;
}

#[no_mangle]
pub unsafe extern "C" fn close_connection(_identifier: i64) -> u32 {
    return 1;
}

#[no_mangle]
pub unsafe extern "C" fn buffer_size() -> u32 {
    return 16 * 1024;
}

#[test]
#[serial]
fn test_file_read_default() {
    let test_case = "Hi wasmer.\nGood to see you.\n".as_bytes();
    unsafe {
        for i in 0..test_case.len() {
            FILE[i] = *test_case.get(i).unwrap();
        }
    }
    let file = unsafe { FileReader::new("./".to_string()).unwrap() };
    let mut reader = BufReader::new(file);
    let mut big_buffer = Vec::new();
    reader.read(&mut big_buffer).unwrap();
    let expected: Vec<u8> = vec![];
    assert_eq!(big_buffer, expected);
    big_buffer.resize(20 * 1048576, 0);
    let ret = reader.read(&mut big_buffer).unwrap();
    let mut remain = unsafe { FILE[..8192].to_vec() };
    remain.resize(20 * 1048576, 0);
    assert_eq!(big_buffer.len(), 20 * 1048576);
    assert_eq!(ret, 8192); // default size is 8kb for BufReader, see https://doc.rust-lang.org/std/io/struct.BufReader.html#method.new
    assert_eq!(big_buffer.to_vec(), remain);
    unsafe {
        FILE = [99; 16384 * 1024];
        POINTER = 0
    };
}

#[test]
#[serial]
fn test_file_read_with_cap() {
    let test_case = "Hi wasmer.\nGood to see you.\n".as_bytes();
    unsafe {
        for i in 0..test_case.len() {
            FILE[i] = *test_case.get(i).unwrap();
        }
    }
    let file = unsafe { FileReader::new("./".to_string()).unwrap() };
    let mut reader = BufReader::with_capacity(20 * 1048576, file);
    let mut big_buffer = Vec::new();
    big_buffer.resize(20 * 1048576, 0);
    let ret = reader.read(&mut big_buffer).unwrap();
    let mut expected = unsafe { FILE.to_vec() };
    expected.resize(20 * 1048576, 0);
    assert_eq!(ret, 16 * 1048576);
    assert_eq!(big_buffer, expected);
    unsafe {
        FILE = [99; 16384 * 1024];
        POINTER = 0
    };
}

#[test]
#[serial]
fn test_file_read_line() {
    let test_case = "Hi wasmer.\nGood to see you.\n".as_bytes();
    unsafe {
        for i in 0..test_case.len() {
            FILE[i] = *test_case.get(i).unwrap();
        }
    }
    let file = unsafe { FileReader::new("./".to_string()).unwrap() };
    let mut reader = BufReader::new(file);
    let mut big_buffer = String::new();
    reader.read_line(&mut big_buffer).unwrap();
    assert_eq!(big_buffer, "Hi wasmer.\n");
    reader.read_line(&mut big_buffer).unwrap();
    assert_eq!(big_buffer, "Hi wasmer.\nGood to see you.\n");
    unsafe {
        FILE = [99; 16384 * 1024];
        POINTER = 0
    };
}

#[test]
#[serial]
fn test_file_read_all() {
    let test_case = "Hi wasmer.\nGood to see you.\n".as_bytes();
    unsafe {
        for i in 0..test_case.len() {
            FILE[i] = *test_case.get(i).unwrap();
        }
    }
    let file = unsafe { FileReader::new("./".to_string()).unwrap() };
    let mut reader = BufReader::new(file);
    let mut big_buffer = Vec::new();
    reader.read_to_end(&mut big_buffer).unwrap();
    let mut remain = vec![];
    for _ in 0..unsafe { FILE.len() } - test_case.len() {
        remain.push(99);
    }
    let mut expected = "Hi wasmer.\nGood to see you.\n".as_bytes().to_vec();
    expected.append(&mut remain);
    assert_eq!(big_buffer, expected);
    unsafe {
        FILE = [99; 16384 * 1024];
        POINTER = 0
    };
}

#[test]
#[serial]
fn test_file_read_exact() {
    let test_case = "Hi wasmer.\nGood to see you.\n".as_bytes();
    unsafe {
        for i in 0..test_case.len() {
            FILE[i] = *test_case.get(i).unwrap();
        }
    }
    let file = unsafe { FileReader::new("./".to_string()).unwrap() };
    let mut reader = BufReader::new(file);
    let mut big_buffer = Vec::new();
    reader.read_exact(&mut big_buffer).unwrap();
    let expected: Vec<u8> = vec![];
    assert_eq!(big_buffer, expected);
    let mut big_buffer = [0u8; 1 * 1048576];
    reader.read_exact(&mut big_buffer).unwrap();
    let mut remain = vec![];
    for _ in 0..1048576 - test_case.len() {
        remain.push(99);
    }
    let mut expected = "Hi wasmer.\nGood to see you.\n".as_bytes().to_vec();
    expected.append(&mut remain);
    assert_eq!(big_buffer.len(), 1048576);
    assert_eq!(expected.len(), 1048576);
    assert_eq!(unsafe { POINTER }, 1048576);
    assert_eq!(big_buffer.to_vec(), expected);
    unsafe {
        FILE = [99; 16384 * 1024];
        POINTER = 0
    };
}
