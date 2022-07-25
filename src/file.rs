use std::{
    cmp::min,
    io::{Error, Read, Write},
};

extern "C" {
    fn read_file_stream(identifier: i64, ptr_to_write: u64) -> u64;
    fn write_file_stream(identifier: i64, ptr_to_buffer: u64, length_buffer: u64) -> u64;
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
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // let buf = &buf[..min(self.buffer_size as usize, buf.len())];
        if buf.len() > self.buffer_size as usize {
            return Err(Error::new(std::io::ErrorKind::Other, "Buffer size is exceeding the maximum transmission size allowed, please use [write_all] instead"));
        }
        let len = buf.len();
        unsafe { Ok(write_file_stream(self.id, buf.as_ptr() as u64, len as u64) as usize) }
    }

    fn write_all(&mut self, mut buf: &[u8]) -> std::io::Result<()> {
        let mut subarray: &[u8];
        while buf.len() > self.buffer_size as usize {
            (subarray, buf) = buf.split_at(self.buffer_size as usize);
            self.write(subarray)?;
        }
        self.write(buf)?;
        Ok(())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let res = unsafe { flush(self.id) };
        if res == 0 {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Failed to flush the written buffer on Sirius Chain side, it returned false",
            ));
        }
        Ok(())
    }
}

impl FileWriter {
    pub unsafe fn new(path: String) -> Result<Self, Error> {
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
                "File cannot be opened, negative id returned by the Sirius Chain",
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
    buffer: Vec<u8>,
}

impl Read for FileReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        unsafe {
            self.read_from_bc();
        }
        let len = min(self.buffer.len(), buf.len());
        // Fill the given buffer                      // Save the exceeding part for future use
        buf[..len].clone_from_slice(&self.buffer.drain(..len).collect::<Vec<u8>>());
        Ok(len)
    }
}

impl FileReader {
    pub unsafe fn new(path: String) -> Result<Self, Error> {
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
                "File cannot be opened, negative id returned by the Sirius Chain",
            ));
        }
        Ok(Self {
            id,
            buffer: Vec::new(),
        })
    }

    unsafe fn read_from_bc(&mut self) {
        let buf_size = buffer_size();
        let mut ret = buf_size;
        let mut subarray = Vec::new();
        for _ in 0..buf_size {
            subarray.push(0);
        }
        // I can't forsee how large the next line is, so I'll just store it and stop calling RPC if exceeding
        while ret > 0 && self.buffer.len() < buf_size as usize {
            ret = read_file_stream(self.id, subarray.as_mut_ptr() as u64);
            self.buffer.append(&mut subarray[..ret as usize].to_vec());
        }
    }
}

impl Drop for FileReader {
    fn drop(&mut self) {
        unsafe {
            close_file(self.id);
        }
    }
}
