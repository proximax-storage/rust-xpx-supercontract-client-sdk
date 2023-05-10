use std::io::{Read, Write};

#[no_mangle]
extern "C" fn read_from_internet(_identifier: i64, ptr_to_write: u64) -> u64 {
    let buf = [99u8; 1024];
    let ptr = ptr_to_write as *mut u8;
    unsafe {
        std::ptr::copy(buf.as_ptr(), ptr, buf.len());
    }
    return 1024;
}

#[no_mangle]
extern "C" fn open_connection(
    _ptr_to_url: u64,
    _length_url: u64,
    _soft_revocation_mode: u8,
) -> i64 {
    return 10000000;
}

#[no_mangle]
extern "C" fn close_connection(_identifier: i64) -> u32 {
    return 1;
}

#[no_mangle]
extern "C" fn buffer_size() -> u64 {
    return 1024;
}

#[no_mangle]
extern "C" fn write_file_stream(
    _identifier: i64,
    _ptr_to_buffer: u64,
    _length_buffer: u64,
) -> i64 {
    return 1024;
}
#[no_mangle]
extern "C" fn open_file(
    _ptr_to_path: u64,
    _length_path: u64,
    _ptr_to_mode: u64,
    _length_mode: u64,
) -> i64 {
    return 1;
}
#[no_mangle]
extern "C" fn close_file(_identifier: i64) -> u32 {
    return 1;
}
#[no_mangle]
extern "C" fn flush(_identifier: i64) -> u32 {
    return 1;
}

#[test]
fn test_internet() {
    // Arrange:
    let mut file = sdk::internet::Internet::new("./", true).unwrap();
    let mut buf = vec![99u8; 1024];
    let len = file.read(buf.as_mut_slice()).unwrap();
    assert_eq!(1024, len);

    let mut file = sdk::file::FileWriter::new("./").unwrap();
    let len = file.write(buf.as_mut_slice()).unwrap();
    file.flush().expect("flush successful");
    assert_eq!(1024, len);
}