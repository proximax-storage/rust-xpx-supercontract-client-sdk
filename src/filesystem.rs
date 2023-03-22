use std::io;
use std::io::Error;

mod import_function {
    extern "C" {
        pub fn remove_filesystem_entry(ptr_to_path: u64, length_path: u64) -> u8;
        pub fn move_filesystem_entry(
            ptr_to_new_path: u64,
            length_new_path: u64,
            ptr_to_old_path: u64,
            length_old_path: u64,
        ) -> u8;
        pub fn path_exists(ptr_to_path: u64, length_path: u64) -> u8;
        pub fn is_file(ptr_to_path: u64, length_path: u64) -> i8;
        pub fn file_size(ptr_to_path: u64, length_path: u64) -> i64;
        pub fn create_dir(ptr_to_path: u64, length_path: u64) -> u8;
    }
}

pub fn remove_filesystem_entry(path: &str) -> std::io::Result<()> {
    let ret = unsafe {
        import_function::remove_filesystem_entry(path.as_ptr() as u64, path.len() as u64)
    };
    if ret == 0 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Failed to remove entry",
        ));
    }
    Ok(())
}

pub fn move_file(path: &str, new_path: &str) -> std::io::Result<()> {
    let ret = unsafe {
        import_function::move_filesystem_entry(
            new_path.as_ptr() as u64,
            new_path.len() as u64,
            path.as_ptr() as u64,
            path.len() as u64,
        )
    };
    if ret == 0 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Failed to move file"
        ));
    }
    Ok(())
}

pub fn path_exists(path: &str) -> bool {
    let res = unsafe {
        import_function::path_exists(path.as_ptr() as u64, path.len() as u64)
    };

    res != 0
}

pub fn is_file(path: &str) -> io::Result<bool> {
    let res = unsafe {
        import_function::is_file(path.as_ptr() as u64, path.len() as u64)
    };

    if res < 0 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Failed to check whether is file"
        ));
    }

    Ok(res != 0)
}

pub fn file_size(path: &str) -> std::io::Result<u64> {
    let res = unsafe {
        import_function::file_size(path.as_ptr() as u64, path.len() as u64)
    };

    if res < 0 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Failed to obtain file size",
        ));
    }

    return Ok(res as u64)
}


pub fn create_dir(path: &str) -> std::io::Result<()> {
    let res = unsafe {
        import_function::create_dir(path.as_ptr() as u64, path.len() as u64)
    };
    if res == 0 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Failed to create a directory",
        ));
    }
    Ok(())
}
