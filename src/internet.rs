use super::file::buffer_size;
use std::{
    cmp::min,
    io::{Error, Read},
};

// I changed the memory addresses (ptr_to_something) to u64 becuase my machine is a 64-bit system
extern "C" {
    fn open_connection(ptr_to_url: u64, length_url: u64) -> i64;
    fn read_from_internet(identifier: i64, ptr_to_write: u64) -> i64; // will return -1 if fail
    fn close_connection(identifier: i64) -> u32;
}

pub struct Internet {
    id: i64,
    buffer: Vec<u8>,
    size: usize,
}

impl Internet {
    pub unsafe fn new(url: String) -> Result<Self, Error> {
        let id = open_connection(url.as_ptr() as u64, url.len() as u64);
        if id < 0 {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Connection cannot be opened, negative id returned by the Sirius Chain",
            ));
        }
        Ok(Self {
            id,
            buffer: Vec::new(),
            size: buffer_size() as usize,
        })
    }

    unsafe fn read_through_rpc(&mut self) -> std::io::Result<()> {
        let mut ret = self.size as i64;
        let mut subarray = Vec::new();
        subarray.resize(self.size, 0);
        // I can't forsee how large the next line is, so I'll just store it and stop calling RPC if exceeding
        while ret > 0 && self.buffer.len() < self.size as usize {
            ret = read_from_internet(self.id, subarray.as_mut_ptr() as u64);
            self.buffer.append(&mut subarray[..ret as usize].to_vec());
        }
        if ret < 0 {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "File cannot be read, Sirius Chain returned negative length of the buffer",
            ));
        }
        Ok(())
    }
}

impl Read for Internet {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut i = 0;
        let mut len = buf.len();
        while i < buf.len() && len > 0 {
            unsafe {
                self.read_through_rpc()?;
            }
            len = min(self.buffer.len(), buf.len() - i);
            // Fill the given buffer                      // Save the exceeding part for future use
            buf[i..i + len].clone_from_slice(&self.buffer.drain(..len).collect::<Vec<u8>>());
            i += len;
        }
        Ok(i)
    }
}

impl Drop for Internet {
    fn drop(&mut self) {
        unsafe {
            close_connection(self.id);
        }
    }
}
