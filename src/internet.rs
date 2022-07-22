use std::{
    cmp::min,
    io::{Error, Read},
};

extern "C" {
    fn open_connection(ptr_to_url: u32, length_url: u32) -> i64;
    fn read_from_internet(identifier: i64, ptr_to_write: u32) -> u32;
    fn close_connection(identifier: i64) -> u32;
    fn buffer_size() -> u32;
}

pub struct Internet {
    id: i64,
    buffer: Vec<u8>,
}

impl Internet {
    pub unsafe fn new(url: String) -> Result<Self, Error> {
        let id = open_connection(url.as_ptr() as u32, url.len() as u32);
        if id < 0 {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Connection cannot be opened, negative id returned by the Sirius Chain",
            ));
        }
        Ok(Self {
            id,
            buffer: Internet::read_from_bc(id),
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
            ret = read_from_internet(id, subarray.as_mut_ptr() as u32);
            buffer.append(&mut subarray);
            // subarray.fill(0);
        }
        return buffer;
    }
}

impl Read for Internet {
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

impl Drop for Internet {
    fn drop(&mut self) {
        unsafe {
            close_connection(self.id);
        }
    }
}
