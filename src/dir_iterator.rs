use std::mem::size_of;

extern "C" {
    fn create_dir_iterator(ptr_to_path: u64, length_path: u64, recursive: u8) -> i64;
    fn destroy_dir_iterator(identifier: i64) -> u8;
    fn next_dir_iterator(identifier: i64, ptr_to_write: u64) -> u8;
    fn has_next_dir_iterator(identifier: i64) -> u8;
}

pub struct DirIterator {
    id: i64,
}

#[derive(Debug, Clone)]
pub struct DirIteratorEntry {
    pub name: String,
    pub depth: u32
}

impl Iterator for DirIterator {
    type Item = DirIteratorEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if unsafe { has_next_dir_iterator(self.id) } == 0 {
            return None;
        }
        // sizeof(depth) + string size + string data
        let capacity = size_of::<u32>() + size_of::<u16>() + DirIterator::MAX_NAME_SIZE;
        let mut iterator_value_buffer: Vec<u8> = Vec::new();
        iterator_value_buffer.resize(capacity, 0);
        let success = unsafe {
            next_dir_iterator(self.id, iterator_value_buffer.as_mut_ptr() as u64)
        } != 0;
        if !success {
            return None;
        }

        let (depth_bytes, tail) = iterator_value_buffer.split_at(size_of::<u32>());
        let depth = u32::from_le_bytes(depth_bytes.try_into().unwrap());

        let (name_size_bytes, tail) = tail.split_at(size_of::<u16>());
        let name_size = u16::from_le_bytes(name_size_bytes.try_into().unwrap()) as usize;

        let name = String::from_utf8(tail[0..name_size].to_vec());

        if let Ok(name) = name {
            return Some(DirIteratorEntry{
                name,
                depth
            });
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

    const MAX_NAME_SIZE: usize = 256;

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
}
