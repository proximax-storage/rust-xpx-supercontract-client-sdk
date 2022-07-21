use std::io::{Read, Write};

extern "C" {
    fn read_file_stream(identifier: i64, ptr_to_write: u32) -> u32;
    fn write_file_stream(identifier: i64, ptr_to_buffer: u32, length_buffer: u32) -> u64;
    fn open_file(ptr_to_path: u32, length_path: u32, ptr_to_mode: u32, length_mode: u32) -> i64;
    fn close_file(identifier: i64) -> u32;
    fn open_connection(ptr_to_url: u32, length_url: u32) -> i64;
    fn read_from_internet(identifier: i64, ptr_to_write: u32) -> u32;
    fn close_connection(identifier: i64) -> u32;
}

// We have different function signatures for both File and Internet, so I do not make the constructor part of this trait
// We do not need to write to Internet, only File will need to write something, so I do not inherit Write trait for this trait
#[allow(drop_bounds)]
pub trait Stream: Read + Drop {}

pub struct File {
    id: i64,
}

impl Stream for File {}

impl File {
    pub unsafe fn new(path: String, mode: String) -> Self {
        Self {
            id: open_file(
                path.as_ptr() as u32,
                path.len() as u32,
                mode.as_ptr() as u32,
                mode.len() as u32,
            ),
        }
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        unsafe {
            return Ok(read_file_stream(self.id, buf.as_mut_ptr() as u32) as usize);
        }
    }
}

impl Write for File {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        unsafe {
            return Ok(write_file_stream(self.id, buf.as_ptr() as u32, buf.len() as u32) as usize);
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Drop for File {
    fn drop(&mut self) {
        let res: u32;
        unsafe {
            res = close_file(self.id);
        }
        if res == 0 {
            panic!("File does not close successfully")
        }
    }
}

pub struct Internet {
    id: i64,
}

impl Stream for Internet {}

impl Internet {
    pub unsafe fn new(url: String) -> Self {
        Self {
            id: open_connection(url.as_ptr() as u32, url.len() as u32),
        }
    }
}

impl Read for Internet {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        unsafe { return Ok(read_from_internet(self.id, buf.as_mut_ptr() as u32) as usize) }
    }
}

impl Drop for Internet {
    fn drop(&mut self) {
        let res: u32;
        unsafe {
            res = close_connection(self.id);
        }
        if res == 0 {
            panic!("File does not close successfully")
        }
    }
}
