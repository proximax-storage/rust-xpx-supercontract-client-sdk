use std::io::{Error, Read, Write};

extern "C" {
    fn read_file_stream(identifier: i64, ptr_to_write: u32) -> u32;
    fn write_file_stream(identifier: i64, ptr_to_buffer: u32, length_buffer: u32) -> u64;
    fn open_file(ptr_to_path: u32, length_path: u32, ptr_to_mode: u32, length_mode: u32) -> i64;
    fn close_file(identifier: i64) -> u32;
    fn open_connection(ptr_to_url: u32, length_url: u32) -> i64;
    fn read_from_internet(identifier: i64, ptr_to_write: u32) -> u32;
    fn close_connection(identifier: i64) -> u32;
    fn flush(identifier: i64);
}

pub struct FileWriter(i64);

impl Write for FileWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        unsafe {
            return Ok(write_file_stream(self.0, buf.as_ptr() as u32, buf.len() as u32) as usize);
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        unsafe { Ok(flush(self.0)) }
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
        Ok(Self(id))
    }
}

impl Drop for FileWriter {
    fn drop(&mut self) {
        unsafe {
            close_file(self.0);
        }
    }
}

pub struct FileReader(i64);

impl Read for FileReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        unsafe {
            return Ok(read_file_stream(self.0, buf.as_mut_ptr() as u32) as usize);
        }
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
        Ok(Self(id))
    }
}

impl Drop for FileReader {
    fn drop(&mut self) {
        unsafe {
            close_file(self.0);
        }
    }
}

pub struct Internet {
    id: i64,
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
        Ok(Self { id })
    }
}

impl Read for Internet {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        unsafe { return Ok(read_from_internet(self.id, buf.as_mut_ptr() as u32) as usize) }
    }
}

impl Drop for Internet {
    fn drop(&mut self) {
        unsafe {
            close_connection(self.id);
        } // The execution will be interrupted if the import function returns an error
    }
}
