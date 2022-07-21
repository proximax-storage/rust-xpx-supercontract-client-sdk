use std::io::{Error, Read};

extern "C" {
    fn open_connection(ptr_to_url: u32, length_url: u32) -> i64;
    fn read_from_internet(identifier: i64, ptr_to_write: u32) -> u32;
    fn close_connection(identifier: i64) -> u32;
}

pub struct Internet(i64);

impl Internet {
    pub unsafe fn new(url: String) -> Result<Self, Error> {
        let id = open_connection(url.as_ptr() as u32, url.len() as u32);
        if id < 0 {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Connection cannot be opened, negative id returned by the Sirius Chain",
            ));
        }
        Ok(Self(id))
    }
}

impl Read for Internet {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if buf.len() > 16 * 1024 {
            let mut res = 0;
            for i in (0..buf.len()).step_by(16 * 1024) {
                if i + 16 * 1024 < buf.len() {
                    let subarray = &buf[i..16 * 1024];
                    unsafe {
                        res += read_from_internet(self.0, subarray.as_ptr() as u32);
                    }
                } else {
                    let subarray = &buf[i..buf.len()];
                    unsafe {
                        res += read_from_internet(self.0, subarray.as_ptr() as u32);
                    }
                }
            }
            Ok(res as usize)
        } else {
            unsafe { Ok(read_from_internet(self.0, buf.as_ptr() as u32) as usize) }
        }
    }
}

impl Drop for Internet {
    fn drop(&mut self) {
        unsafe {
            close_connection(self.0);
        }
    }
}
