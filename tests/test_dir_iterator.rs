use sdk::dir_iterator::{DirIterator, DirIteratorEntry};
use serial_test::serial;

static mut POINTER: usize = 0;
static mut ITERATOR_VALUES: Vec<DirIteratorEntry> = Vec::new();

#[no_mangle]
pub extern "C" fn create_dir_iterator(ptr_to_path: u64, length_path: u64, _recursive: u8) -> i64 {
    let ptr = ptr_to_path as *mut u8;
    let mut buffer = Vec::new();
    buffer.resize(length_path as usize, 0);
    for i in 0..length_path as usize {
        buffer[i] = unsafe { *ptr.add(i) };
    }
    let path = String::from_utf8(buffer).unwrap();
    assert_eq!(path, "home");
    return 11;
}

#[no_mangle]
pub extern "C" fn destroy_dir_iterator(identifier: i64) -> u8 {
    assert_eq!(identifier, 11);
    return 1;
}

#[no_mangle]
pub extern "C" fn has_next_dir_iterator(identifier: i64) -> u8 {
    assert_eq!(identifier, 11);
    return unsafe { POINTER < ITERATOR_VALUES.len() } as u8;
}

#[no_mangle]
pub extern "C" fn next_dir_iterator(identifier: i64, ptr_to_write: u64) -> u8 {
    assert_eq!(identifier, 11);
    unsafe {
        let ptr = ptr_to_write as *mut u8;
        let entry = &ITERATOR_VALUES[POINTER];
        POINTER += 1;

        let mut buffer: Vec<u8> = Vec::new();
        buffer.extend_from_slice(&entry.depth.to_le_bytes());
        buffer.extend_from_slice(&(entry.name.len() as u16).to_le_bytes());
        buffer.extend_from_slice(entry.name.as_bytes());

        let length = buffer.len();
        for x in 0..length {
            *ptr.add(x) = buffer[x];
        }
    }

    return 1;
}

#[test]
#[serial]
fn test_iterator() {
    let iterator = DirIterator::new("home", false);
    let expected: Vec<DirIteratorEntry> = vec![
        DirIteratorEntry {
            name: "rust".to_string(),
            depth: 0
        },
        DirIteratorEntry {
            name: "wasmer".to_string(),
            depth: 0
        },
        DirIteratorEntry {
            name: "sirius".to_string(),
            depth: 0
        },
    ];
    unsafe {
        POINTER = 0;
        ITERATOR_VALUES = expected.clone();
    }
    let actual = iterator.collect::<Vec<_>>();
    assert_eq!(actual.len(), expected.len());
    for i in 0..expected.len() {
        assert_eq!(actual[i].depth, expected[i].depth);
        assert_eq!(actual[i].name, expected[i].name);
    }
}

#[test]
#[serial]
fn test_iterator_recurse() {
    let iterator = DirIterator::new("home", false);
    let expected: Vec<DirIteratorEntry> = vec![
        DirIteratorEntry {
            name: "rust".to_string(),
            depth: 0
        },
        DirIteratorEntry {
            name: "wasmer".to_string(),
            depth: 1
        },
        DirIteratorEntry {
            name: "sirius".to_string(),
            depth: 1
        },
    ];
    unsafe {
        POINTER = 0;
        ITERATOR_VALUES = expected.clone();
    }
    let actual = iterator.collect::<Vec<_>>();
    assert_eq!(actual.len(), expected.len());
    for i in 0..expected.len() {
        assert_eq!(actual[i].depth, expected[i].depth);
        assert_eq!(actual[i].name, expected[i].name);
    }
}

#[test]
#[serial]
#[should_panic]
fn test_iterator_invalid_path() {
    DirIterator::new("", true);
}
