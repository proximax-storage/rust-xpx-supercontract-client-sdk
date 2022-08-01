use std::{
    cmp::min,
    io::{Error, Read, Write},
};

// I changed the memory addresses (ptr_to_something) to u64 becuase my machine is a 64-bit system
extern "C" {
    fn read_file_stream(identifier: i64, ptr_to_write: u64) -> i64; // it will return -1 if fail
    fn write_file_stream(identifier: i64, ptr_to_buffer: u64, length_buffer: u64) -> i64; // it will return -1 if fail
    fn open_file(ptr_to_path: u64, length_path: u64, ptr_to_mode: u64, length_mode: u64) -> i64;
    fn close_file(identifier: i64) -> u32;
    fn flush(identifier: i64) -> u32;
    pub fn buffer_size() -> u64;
    fn remove_file(ptr_to_path: u64, length_path: u64) -> u32;
    fn rename_file(
        ptr_to_new_path: u64,
        length_new_path: u64,
        ptr_to_old_path: u64,
        length_old_path: u64,
    ) -> u32;
    fn listdir(ptr_to_path: u64, length_path: u64, ptr_to_write: u64) -> i64;
    fn path_exists(ptr_to_path: u64, length_path: u64) -> u32;
    fn is_file(ptr_to_path: u64, length_path: u64) -> u32;
    fn create_dir(ptr_to_path: u64, length_path: u64) -> u32;
}

pub struct FileWriter {
    id: i64,
    buffer_size: u64,
}

impl Write for FileWriter {
    fn write(&mut self, mut buf: &[u8]) -> std::io::Result<usize> {
        let mut subarray: &[u8];
        let mut ret = 0;
        let mut written_length = i64::MAX;
        while buf.len() > self.buffer_size as usize && written_length > 0 {
            (subarray, buf) = buf.split_at(self.buffer_size as usize);
            let len = subarray.len();
            unsafe {
                written_length = write_file_stream(self.id, subarray.as_ptr() as u64, len as u64);
            }
            if written_length < 0 {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "File cannot be written, Sirius Chain returned negative value for bytes written",
                ));
            }
            ret += written_length;
        }
        if buf.len() <= self.buffer_size as usize {
            unsafe {
                (subarray, buf) = buf.split_at(buf.len() as usize);
                let len = subarray.len();
                ret += write_file_stream(self.id, subarray.as_ptr() as u64, len as u64);
            }
        }
        if buf.len() > 0 {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "The file is too small to store all the data in the given buffer",
            ));
        }
        Ok(ret as usize)
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
    size: usize,
}

impl Read for FileReader {
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
            size: buffer_size() as usize,
        })
    }

    unsafe fn read_through_rpc(&mut self) -> std::io::Result<()> {
        let mut ret = self.size as i64;
        let mut subarray = Vec::new();
        subarray.resize(self.size, 0);
        // I can't forsee how large the next line is, so I'll just store it and stop calling RPC if exceeding
        while ret > 0 && self.buffer.len() < self.size as usize {
            ret = read_file_stream(self.id, subarray.as_mut_ptr() as u64);
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

impl Drop for FileReader {
    fn drop(&mut self) {
        unsafe {
            close_file(self.id);
        }
    }
}

pub unsafe fn remove_file_sdk(path: String) -> std::io::Result<()> {
    let ret = remove_file(path.as_ptr() as u64, path.len() as u64);
    if ret == 0 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Failed to remove file at {}, Sirius Chain returned false from the operation",
                path
            ),
        ));
    }
    Ok(())
}

pub unsafe fn rename_file_sdk(path: String, new_path: String) -> std::io::Result<()> {
    let ret = rename_file(
        new_path.as_ptr() as u64,
        new_path.len() as u64,
        path.as_ptr() as u64,
        path.len() as u64,
    );
    if ret == 0 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Failed to remove file at {}, Sirius Chain returned false from the operation",
                path
            ),
        ));
    }
    Ok(())
}

pub unsafe fn listdir_sdk(dir: String) -> std::io::Result<Vec<String>> {
    let mut filenames = Vec::new();
    filenames.resize(buffer_size() as usize, 0u8);
    let num_file = listdir(
        dir.as_ptr() as u64,
        dir.len() as u64,
        filenames.as_mut_ptr() as u64,
    );
    if num_file < 0 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Failed to retrieve entries at {}, Sirius Chain returned negative number for the number of entries",
                dir
            ),
        ));
    }
    let filenames = filenames
        .split(|x| *x == '\n'.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>();
    let files = filenames
        .into_iter()
        .map(|x| String::from_utf8(x.to_vec()).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(num_file as usize, files.len());
    Ok(files)
}

pub unsafe fn path_exists_sdk(path: String) -> bool {
    let res = path_exists(path.as_ptr() as u64, path.len() as u64);
    if res == 0 {
        return false;
    }
    true
}

pub unsafe fn is_file_sdk(path: String) -> bool {
    let res = is_file(path.as_ptr() as u64, path.len() as u64);
    if res == 0 {
        return false;
    }
    true
}

pub unsafe fn create_dir_sdk(path: String) -> std::io::Result<()> {
    let res = create_dir(path.as_ptr() as u64, path.len() as u64);
    if res == 0 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Sirius Chain failed to create a directory at the given location",
        ));
    }
    Ok(())
}
