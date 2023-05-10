use std::io::Write;

use sdk::file::FileWriter;

#[no_mangle]
extern "C" fn open_file(
    _ptr_to_path: u64,
    _length_path: u64,
    _ptr_to_mode: u64,
    _length_mode: u64,
) -> i64 {
    return 10000000;
}

#[no_mangle]
extern "C" fn read_file_stream(_identifier: i64, ptr_to_write: u64) -> i64 {
    let buf: [u8; 1024] = [99; 1024];
    let ptr = ptr_to_write as *mut u8;
    unsafe {
        std::ptr::copy(buf.as_ptr(), ptr, buf.len());
    }
    return 1;
}

#[no_mangle]
extern "C" fn close_file(_identifier: i64) -> u32 {
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
extern "C" fn path_exists(_ptr_to_path: u64, _length_path: u64) -> u8 {
    return 1;
}
#[no_mangle]
extern "C" fn is_file(_ptr_to_path: u64, _length_path: u64) -> i8 {
    return 1;
}
#[no_mangle]
extern "C" fn file_size(_ptr_to_path: u64, _length_path: u64) -> i64 {
    return 1;
}
#[no_mangle]
extern "C" fn create_dir(_ptr_to_path: u64, _length_path: u64) -> u8 {
    return 1;
}
#[no_mangle]
extern "C" fn move_filesystem_entry(
    _ptr_to_new_path: u64,
    _length_new_path: u64,
    _ptr_to_old_path: u64,
    _length_old_path: u64,
) -> u8 {
    return 1;
}

#[test]
fn test_internet() {
    // Arrange:
    let mut buf = [99u8; 1024];
    if sdk::filesystem::path_exists("./test") {
        sdk::filesystem::create_dir("./test").unwrap();
    } else {
        let mut file = FileWriter::new("./text").unwrap();
        let len = file.write(buf.as_mut_slice()).unwrap();
        assert_eq!(1024, len);
        sdk::filesystem::move_filesystem_entry("./text", "./test/text").unwrap();
        let size = sdk::filesystem::file_size("./test/text").unwrap();
        assert_eq!(1024, size);
    }
}