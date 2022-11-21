use std::io::Error;

mod import_function {
    extern "C" {
        pub fn remove_file(ptr_to_path: u64, length_path: u64) -> u32;
        pub fn move_file(
            ptr_to_new_path: u64,
            length_new_path: u64,
            ptr_to_old_path: u64,
            length_old_path: u64,
        ) -> u32;
        pub fn path_exists(ptr_to_path: u64, length_path: u64) -> u32;
        pub fn is_file(ptr_to_path: u64, length_path: u64) -> u32;
        pub fn create_dir(ptr_to_path: u64, length_path: u64) -> u32;
    }
}

pub unsafe fn remove_file(path: &str) -> std::io::Result<()> {
    let ret = import_function::remove_file(path.as_ptr() as u64, path.len() as u64);
    if ret == 0 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to remove file at {}", path),
        ));
    }
    Ok(())
}

pub unsafe fn move_file(path: &str, new_path: &str) -> std::io::Result<()> {
    let ret = import_function::move_file(
        new_path.as_ptr() as u64,
        new_path.len() as u64,
        path.as_ptr() as u64,
        path.len() as u64,
    );
    if ret == 0 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to remove file at {}", path),
        ));
    }
    Ok(())
}

pub unsafe fn path_exists(path: &str) -> bool {
    let res = import_function::path_exists(path.as_ptr() as u64, path.len() as u64);
    if res == 0 {
        return false;
    }
    true
}

pub unsafe fn is_file(path: &str) -> bool {
    let res = import_function::is_file(path.as_ptr() as u64, path.len() as u64);
    if res == 0 {
        return false;
    }
    true
}

pub unsafe fn create_dir(path: &str) -> std::io::Result<()> {
    let res = import_function::create_dir(path.as_ptr() as u64, path.len() as u64);
    if res == 0 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Failed to create a directory at the given location",
        ));
    }
    Ok(())
}
