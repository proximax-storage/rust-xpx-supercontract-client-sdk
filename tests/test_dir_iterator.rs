use sdk::dir_iterator::DirIterator;
use serial_test::serial;
use std::{
    fs,
    sync::{Arc, Mutex},
};

static mut ITERATOR: Option<Arc<Mutex<fs::ReadDir>>> = None;
static mut ITERATOR_STACK: Option<Vec<Arc<Mutex<fs::ReadDir>>>> = None;
static mut IDENTIFIER: i64 = 0;
static mut RECURSE: bool = false;
static mut CURRENT: Option<String> = None;

#[no_mangle]
pub extern "C" fn create_dir_iterator(ptr_to_path: u64, length_path: u64, recursive: u8) -> i64 {
    let ptr = ptr_to_path as *mut u8;
    let mut buffer = Vec::new();
    buffer.resize(length_path as usize, 0);
    for i in 0..length_path as usize {
        buffer[i] = unsafe { *ptr.add(i) };
    }
    let path = String::from_utf8(buffer).unwrap();
    unsafe {
        ITERATOR_STACK = Some(vec![]);
        ITERATOR = Some(Arc::new(Mutex::new(fs::read_dir(path).unwrap())));
        IDENTIFIER += 1;
        RECURSE = if recursive == 1 { true } else { false };
        return IDENTIFIER;
    };
}

#[no_mangle]
pub extern "C" fn destroy_dir_iterator(identifier: i64) -> u8 {
    assert_eq!(unsafe { IDENTIFIER }, identifier);
    unsafe {
        ITERATOR = None;
        RECURSE = false;
        CURRENT = None;
    }
    return 1;
}

#[no_mangle]
pub extern "C" fn has_next_dir_iterator(_identifier: i64) -> u8 {
    // TODO: idk how to implement this, cuz Rust iterator does not provide has_next() function
    return 1;
}

#[no_mangle]
pub extern "C" fn next_dir_iterator(identifier: i64, ptr_to_write: u64) -> u64 {
    assert_eq!(unsafe { IDENTIFIER }, identifier);
    let ptr = ptr_to_write as *mut u8;
    if unsafe { RECURSE } {
        if let Some(iterator) = unsafe { ITERATOR.as_mut() } {
            let path = iterator.lock().unwrap().next();
            if let Some(path) = path {
                if let Ok(path) = path {
                    let path = path.path();
                    if path.is_dir() {
                        if path.to_str().unwrap() == "./target"
                            || path.to_str().unwrap() == "./.git"
                        {
                            return 0;
                        }
                        let temp = unsafe { ITERATOR.as_ref().unwrap().clone() };
                        unsafe {
                            let vec = ITERATOR_STACK.as_mut().unwrap();
                            vec.push(temp.clone());
                            ITERATOR = Some(Arc::new(Mutex::new(fs::read_dir(path).unwrap())))
                        };
                        let res = next_dir_iterator(identifier, ptr_to_write);
                        return res;
                    } else {
                        let path = path.to_str();
                        if let Some(path) = path {
                            unsafe {
                                CURRENT = Some(path.to_string());
                            }
                            let path = path.as_bytes().to_vec();
                            let length = path.len();
                            for x in 0..length {
                                unsafe { *ptr.add(x) = path[x] };
                            }
                            return length as u64;
                        } else {
                            return 0;
                        }
                    }
                } else {
                    return 0;
                }
            } else {
                if unsafe { ITERATOR_STACK.as_mut().unwrap().len() != 0 } {
                    unsafe { ITERATOR = Some(ITERATOR_STACK.as_mut().unwrap().pop().unwrap()) };
                    return next_dir_iterator(identifier, ptr_to_write);
                }
                return 0;
            }
        } else {
            return 0;
        }
    } else {
        if let Some(iterator) = unsafe { ITERATOR.as_mut() } {
            let path = iterator.lock().unwrap().next();
            if let Some(path) = path {
                if let Ok(path) = path {
                    let path = path.path();
                    let path = path.to_str();
                    if let Some(path) = path {
                        unsafe {
                            CURRENT = Some(path.to_string());
                        }
                        let path = path.as_bytes().to_vec();
                        let length = path.len();
                        for x in 0..length {
                            unsafe { *ptr.add(x) = path[x] };
                        }
                        return length as u64;
                    } else {
                        return 0;
                    }
                } else {
                    return 0;
                }
            } else {
                return 0;
            }
        } else {
            return 0;
        }
    }
}

#[no_mangle]
pub extern "C" fn remove_dir_iterator(identifier: i64) -> u8 {
    assert_eq!(unsafe { IDENTIFIER }, identifier);
    if let Some(path) = unsafe { CURRENT.as_ref() } {
        let dir = fs::metadata(path).unwrap();
        if dir.is_dir() {
            fs::remove_dir(path).unwrap();
        } else {
            fs::remove_file(path).unwrap();
        }
        return 1;
    }
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn buffer_size() -> u32 {
    return 16 * 1024;
}

#[test]
#[serial]
fn test_iterator() {
    let iterator = DirIterator::new("./", false);
    let expected = vec![
        "./src",
        "./.gitignore",
        "./tests",
        "./README.md",
        "./.git",
        "./Cargo.lock",
        "./Cargo.toml",
        "./target",
        "./pkg",
    ];
    let actual = iterator.collect::<Vec<_>>();
    assert_eq!(actual, expected);
}

#[test]
#[serial]
fn test_iterator_recurse() {
    let iterator = DirIterator::new("./", true);
    let expected = vec![
        "./src/internet.rs",
        "./src/blockchain.rs",
        "./src/lib.rs",
        "./src/file.rs",
        "./src/dir_iterator.rs",
        "./src/filesystem.rs",
        "./.gitignore",
        "./tests/test_builtin_bufwriter.rs",
        "./tests/test_write.rs",
        "./tests/test_builtin_bufreader.rs",
        "./tests/test_read_16mb.rs",
        "./tests/test_read_empty.rs",
        "./tests/test_write_to_empty_file.rs",
        "./tests/test_read_1gb.rs",
        "./tests/test_read_16kb.rs",
        "./tests/test_dir_iterator.rs",
        "./README.md",
    ];
    let actual = iterator.collect::<Vec<_>>();
    assert_eq!(actual, expected);
}

#[test]
#[serial]
#[should_panic]
fn test_iterator_invalid_path() {
    DirIterator::new("", true);
}

#[test]
#[serial]
fn test_iterator_remove() {
    let mut iterator = DirIterator::new("./", true);
    fs::File::create("tests/dummy.txt").unwrap();
    while let Some(file) = iterator.next() {
        if file == "./tests/dummy.txt" {
            iterator.remove();
        }
    }
}
