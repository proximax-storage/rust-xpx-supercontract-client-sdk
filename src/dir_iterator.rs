use std::{error::Error, io::Error as ioError};

use crate::file::buffer_size;

extern "C" {
    fn create_dir_iterator(ptr_to_path: u64, length_path: u64, recursive: u8) -> i64;
    fn destroy_dir_iterator(identifier: i64) -> u8;
    fn next_dir_iterator(identifier: i64, ptr_to_write: u64) -> i64;
    fn remove_dir_iterator(identifier: i64) -> u8;
    fn has_next_dir_iterator(identifier: i64) -> u8;
}

pub struct DirIterator {
    id: i64,
}

impl Iterator for DirIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if unsafe { has_next_dir_iterator(self.id) } == 0 {
            return None;
        }
        let mut dir = Vec::new();
        dir.resize(unsafe { buffer_size() } as usize, 0);
        let res = unsafe { next_dir_iterator(self.id, dir.as_mut_ptr() as u64) };
        if res == -1 {
            return None;
        }
        dir.truncate(res as usize);
        let dir = String::from_utf8(dir);
        if let Ok(dir) = dir {
            return Some(dir);
        } else {
            return None;
        }
    }
}

impl Drop for DirIterator {
    fn drop(&mut self) {
        unsafe { destroy_dir_iterator(self.id) };
    }
}

impl DirIterator {
    pub fn new(path: &str, recursive: bool) -> Self {
        let string_buff = path.as_bytes().to_vec();
        let id = unsafe {
            create_dir_iterator(
                string_buff.as_ptr() as u64,
                string_buff.len() as u64,
                if recursive { 1 } else { 0 },
            )
        };
        Self { id }
    }

    pub fn remove(&self) -> Result<(), Box<dyn Error>> {
        let res = unsafe { remove_dir_iterator(self.id) };
        if res == 1 {
            return Ok(());
        }
        return Err(Box::new(ioError::new(
            std::io::ErrorKind::Other,
            "Failed to remove the file/folder",
        )));
    }
}
