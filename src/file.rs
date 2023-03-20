use std::{
    cmp::min,
    collections::VecDeque,
    io::{Error, Read, Write},
};

extern "C" {
    fn read_file_stream(identifier: i64, ptr_to_write: u64) -> i64; // it will return -1 if fail
    fn write_file_stream(identifier: i64, ptr_to_buffer: u64, length_buffer: u64) -> i64; // it will return -1 if fail
    fn open_file(ptr_to_path: u64, length_path: u64, ptr_to_mode: u64, length_mode: u64) -> i64;
    fn close_file(identifier: i64) -> u32;
    fn flush(identifier: i64) -> u32;
    pub fn buffer_size() -> u64;
}

pub struct FileWriter {
    id: i64,
    buffer_size: u64,
}

impl Write for FileWriter {
    fn write(&mut self, mut buf: &[u8]) -> std::io::Result<usize> {
        let mut subarray: &[u8];
        let mut ret = 0;
        while buf.len() > 0 {
            (subarray, buf) = buf.split_at(min(buf.len(), self.buffer_size as usize));
            let len = subarray.len();
            let written_length =
                unsafe { write_file_stream(self.id, subarray.as_ptr() as u64, len as u64) };
            if written_length < len as i64 {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "File cannot be written",
                ));
            }
            ret += written_length;
        }
        Ok(ret as usize)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let res = unsafe { flush(self.id) };
        if res == 0 {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Failed to flush the written buffer",
            ));
        }
        Ok(())
    }
}

impl FileWriter {
    pub unsafe fn new(path: &str) -> Result<Self, Error> {
        let mode = "w";
        let id = open_file(
            path.as_ptr() as u64,
            path.len() as u64,
            mode.as_ptr() as u64,
            mode.len() as u64,
        );
        if id < 0 {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "File cannot be opened",
            ));
        }
        Ok(Self {
            id,
            buffer_size: buffer_size(),
        })
    }
}

impl Drop for FileWriter {
    fn drop(&mut self) {
        unsafe {
            close_file(self.id);
        }
    }
}

pub struct FileReader {
    id: i64,
    buffer: VecDeque<u8>,
    size: usize,
}

impl Read for FileReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut i = 0;
        let mut len = buf.len();
        while i < buf.len() && len > 0 {
            unsafe {
                self.load_buffer()?;
            }
            len = min(self.buffer.len(), buf.len() - i);
            for x in 0..len {
                buf[i + x] = self.buffer.pop_front().unwrap();
            }
            i += len;
        }
        Ok(i)
    }
}

impl FileReader {
    pub unsafe fn new(path: &str) -> Result<Self, Error> {
        let mode = "r";
        let id = open_file(
            path.as_ptr() as u64,
            path.len() as u64,
            mode.as_ptr() as u64,
            mode.len() as u64,
        );
        if id < 0 {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "File cannot be opened",
            ));
        }
        Ok(Self {
            id,
            buffer: VecDeque::new(),
            size: buffer_size() as usize,
        })
    }

    unsafe fn load_buffer(&mut self) -> std::io::Result<()> {
        let mut ret = self.size as i64;
        let mut subarray = Vec::new();
        subarray.resize(self.size, 0);
        while ret > 0 && self.buffer.len() < self.size as usize {
            ret = read_file_stream(self.id, subarray.as_mut_ptr() as u64);
            self.buffer
                .append(&mut VecDeque::from(subarray[..ret as usize].to_vec()));
        }
        if ret < 0 {
            return Err(Error::new(std::io::ErrorKind::Other, "File cannot be read"));
        }
        Ok(())
    }
}

impl Drop for FileReader {
    fn drop(&mut self) {
        unsafe {
            close_file(self.id);
        }
    }
}
