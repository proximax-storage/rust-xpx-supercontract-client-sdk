use sdk::file::FileWriter;
use serial_test::serial;
use std::{
    cmp::min,
    io::{BufWriter, Write},
};

static mut FILE: [u8; 16384 * 1024] = [0; 16384 * 1024];
static mut POINTER: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn write_file_stream(
    _identifier: i64,
    ptr_to_buffer: u64,
    length_buffer: u64,
) -> i64 {
    let ptr = ptr_to_buffer as *mut u8;
    let mut ret = 0;
    let offset = POINTER;
    for x in POINTER..min(FILE.len(), POINTER + length_buffer as usize) {
        ret += 1;
        FILE[x] = *ptr.add(x - offset);
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
pub unsafe extern "C" fn buffer_size() -> u32 {
    return 16 * 1024;
}

#[no_mangle]
pub unsafe extern "C" fn flush(_identifier: i64) -> u32 {
    return 1;
}

#[test]
#[serial]
fn test_file_write() {
    let test_case = "Hi wasmer.\nGood to see you.\n".as_bytes();
    let file = unsafe { FileWriter::new("./").unwrap() };
    let mut writer = BufWriter::new(file);
    let ret = writer.write(test_case).unwrap();
    // flush to ensure the buffer wrapped by the BufWriter is all written in the file
    writer.flush().unwrap(); // https://stackoverflow.com/questions/69819990/whats-the-difference-between-flush-and-sync-all#:~:text=So%20what%20is,%27s%20documentation).
    let mut expected = "Hi wasmer.\nGood to see you.\n".as_bytes().to_vec();
    expected.resize(16 * 1048576, 0);
    assert_eq!(ret, test_case.len());
    assert_eq!(unsafe { FILE.to_vec() }, expected);

    let test_case = [99; 1 * 1048576];
    let file = unsafe { FileWriter::new("./").unwrap() };
    let mut writer = BufWriter::new(file);
    let ret = writer.write(&test_case).unwrap();
    writer.flush().unwrap();
    let mut expected = "Hi wasmer.\nGood to see you.\n".as_bytes().to_vec();
    expected.append(&mut vec![99; 1048576]);
    expected.resize(unsafe { FILE.len() }, 0);
    assert_eq!(ret, 1048576);
    assert_eq!(unsafe { FILE.to_vec() }, expected);
    unsafe {
        FILE = [0; 16384 * 1024];
        POINTER = 0;
    };
}

#[test]
#[serial]
fn test_file_write_all() {
    let test_case = "Hi wasmer.\nGood to see you.\n".as_bytes();
    let file = unsafe { FileWriter::new("./").unwrap() };
    let mut writer = BufWriter::new(file);
    writer.write_all(test_case).unwrap();
    // flush to ensure the buffer wrapped by the BufWriter is all written in the file
    writer.flush().unwrap(); // https://stackoverflow.com/questions/69819990/whats-the-difference-between-flush-and-sync-all#:~:text=So%20what%20is,%27s%20documentation).
    let mut expected = "Hi wasmer.\nGood to see you.\n".as_bytes().to_vec();
    expected.resize(16 * 1048576, 0);
    assert_eq!(unsafe { FILE.to_vec() }, expected);

    let test_case = [99; 1 * 1048576];
    let file = unsafe { FileWriter::new("./").unwrap() };
    let mut writer = BufWriter::new(file);
    writer.write_all(&test_case).unwrap();
    writer.flush().unwrap();
    let mut expected = "Hi wasmer.\nGood to see you.\n".as_bytes().to_vec();
    expected.append(&mut vec![99; 1048576]);
    expected.resize(unsafe { FILE.len() }, 0);
    assert_eq!(unsafe { FILE.to_vec() }, expected);
    unsafe {
        FILE = [0; 16384 * 1024];
        POINTER = 0;
    };
}
