use std::{
    cmp::min,
    io::{Error, Read, Write},
};

extern "C" {
    fn read_file_stream(identifier: i64, ptr_to_write: u32) -> u32;
    fn write_file_stream(identifier: i64, ptr_to_buffer: u32, length_buffer: u32) -> u64;
    fn open_file(ptr_to_path: u32, length_path: u32, ptr_to_mode: u32, length_mode: u32) -> i64;
    fn close_file(identifier: i64) -> u32;
    fn flush(identifier: i64) -> u32;
    fn buffer_size() -> u32;
}

pub struct FileWriter {
    id: i64,
    buffer_size: u32,
}

impl Write for FileWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // let buf = &buf[..min(self.buffer_size as usize, buf.len())];
        if buf.len() > self.buffer_size as usize {
            return Err(Error::new(std::io::ErrorKind::Other, "Buffer size is exceeding the maximum transmission size allowed, please use [write_all] instead"));
        }
        let len = buf.len();
        unsafe { Ok(write_file_stream(self.id, buf.as_ptr() as u32, len as u32) as usize) }
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
            path.as_ptr() as u32,
            path.len() as u32,
            mode.as_ptr() as u32,
            mode.len() as u32,
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
        let len = min(self.buffer.len(), buf.len());
        buf[..len].clone_from_slice(&self.buffer.drain(..len).collect::<Vec<u8>>());
        Ok(len)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        if self.buffer.len() < buf.len() {
            return Err(Error::new(std::io::ErrorKind::Other, "The given buffer size is larger than the size of the buffer to be read, cannot read exactly the size given"));
        }
        let len = buf.len();
        buf[..len].clone_from_slice(&self.buffer.drain(..len).collect::<Vec<u8>>());
        Ok(())
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> std::io::Result<usize> {
        let len = self.buffer.len();
        *buf = self.buffer.clone();
        self.buffer.clear();
        return Ok(len);
    }
}

impl FileReader {
    pub unsafe fn new(path: String) -> Result<Self, Error> {
        let mode = "r";
        let id = open_file(
            path.as_ptr() as u32,
            path.len() as u32,
            mode.as_ptr() as u32,
            mode.len() as u32,
        );
        if id < 0 {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "File cannot be opened, negative id returned by the Sirius Chain",
            ));
        }
        Ok(Self {
            id,
            buffer: FileReader::read_from_bc(id),
        })
    }

    unsafe fn read_from_bc(id: i64) -> Vec<u8> {
        let buf_size = buffer_size();
        let mut buffer = vec![];
        let mut subarray: Vec<u8> = vec![];
        for _ in 0..buf_size {
            subarray.push(0);
        }
        let mut ret = buf_size;
        while ret > 0 {
            ret = read_file_stream(id, subarray.as_mut_ptr() as u32);
            buffer.append(&mut subarray);
            // subarray.fill(0);
        }
        return buffer;
    }
}

impl Drop for FileReader {
    fn drop(&mut self) {
        unsafe {
            close_file(self.id);
        }
    }
}
