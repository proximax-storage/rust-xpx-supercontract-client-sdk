use std::{cmp::min, io::Read};

use sdk::file::FileReader;

static mut BUF: [u8; 16384 * 1024] = [99; 16384 * 1024];
static mut I: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn read_file_stream(_identifier: i64, ptr_to_write: u64) -> u64 {
    let ptr = ptr_to_write as *mut u8;
    let mut ret = 0;
    let offset = I;
    for x in I..min(16384 + I, BUF.len()) {
        ret += 1;
        *ptr.add(x - offset) = BUF[x];
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

#[test]
fn test_read_16mb_buffer_1gb() {
    let mut file = unsafe { FileReader::new("./".to_string()).unwrap() };
    let mut big_buffer = Vec::new();
    let mut buffer = vec![0; 1073741824];
    let mut len = file.read(&mut buffer).unwrap();
    let mut tmp_buffer = buffer[..len].to_vec();
    big_buffer.append(&mut tmp_buffer);
    assert_eq!(big_buffer, vec![99; 16384]); // Max buffer size in the file class is 16kb only
    while len > 0 {
        len = file.read(&mut buffer).unwrap();
        tmp_buffer = buffer[..len].to_vec();
        big_buffer.append(&mut tmp_buffer);
    }
    assert_eq!(big_buffer, vec![99; 16384 * 1024]);
}

#[test]
fn test_read_16mb_buffer_1mb() {
    let mut file = unsafe { FileReader::new("./".to_string()).unwrap() };
    let mut big_buffer = Vec::new();
    let mut buffer = vec![0; 1024 * 1024];
    let mut len = file.read(&mut buffer).unwrap();
    let mut tmp_buffer = buffer[..len].to_vec();
    big_buffer.append(&mut tmp_buffer);
    assert_eq!(big_buffer, vec![99; 16384]); // Max buffer size in the file class is 16kb only
    while len > 0 {
        len = file.read(&mut buffer).unwrap();
        tmp_buffer = buffer[..len].to_vec();
        big_buffer.append(&mut tmp_buffer);
    }
    assert_eq!(big_buffer, vec![99; 16384 * 1024]);
}

#[test]
fn test_read_16mb_buffer_1kb() {
    let mut file = unsafe { FileReader::new("./".to_string()).unwrap() };
    let mut big_buffer = Vec::new();
    let mut buffer = vec![0; 1024];
    let mut len = file.read(&mut buffer).unwrap();
    let mut tmp_buffer = buffer[..len].to_vec();
    big_buffer.append(&mut tmp_buffer);
    assert_eq!(big_buffer, vec![99; 1024]); // My buffer size is only 1kb, so the expected buffer should be of size 1kb
    while len > 0 {
        len = file.read(&mut buffer).unwrap();
        tmp_buffer = buffer[..len].to_vec();
        big_buffer.append(&mut tmp_buffer);
    }
    assert_eq!(big_buffer, vec![99; 16384 * 1024]);
}
