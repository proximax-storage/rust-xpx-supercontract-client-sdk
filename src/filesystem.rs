use super::file::buffer_size;
use std::io::Error;

mod import_function {
    extern "C" {
        pub fn remove_file(ptr_to_path: u64, length_path: u64) -> u32;
        pub fn rename_file(
            ptr_to_new_path: u64,
            length_new_path: u64,
            ptr_to_old_path: u64,
            length_old_path: u64,
        ) -> u32;
        pub fn listdir(ptr_to_path: u64, length_path: u64, ptr_to_write: u64) -> i64;
        pub fn path_exists(ptr_to_path: u64, length_path: u64) -> u32;
        pub fn is_file(ptr_to_path: u64, length_path: u64) -> u32;
        pub fn create_dir(ptr_to_path: u64, length_path: u64) -> u32;
    }
}

pub unsafe fn remove_file(path: String) -> std::io::Result<()> {
    let ret = import_function::remove_file(path.as_ptr() as u64, path.len() as u64);
    if ret == 0 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to remove file at {}", path),
        ));
    }
    Ok(())
}

pub unsafe fn rename_file(path: String, new_path: String) -> std::io::Result<()> {
    let ret = import_function::rename_file(
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

pub unsafe fn listdir(dir: String) -> std::io::Result<Vec<String>> {
    let mut filenames = Vec::new();
    filenames.resize(buffer_size() as usize, 0u8);
    let res = import_function::listdir(
        dir.as_ptr() as u64,
        dir.len() as u64,
        filenames.as_mut_ptr() as u64,
    );
    if res < 0 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to retrieve entries at {}", dir),
        ));
    }
    let filenames = filenames
        .split(|x| *x == '\0'.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>();
    let files = filenames
        .into_iter()
        .map(|x| String::from_utf8(x.to_vec()).unwrap())
        .collect::<Vec<_>>();
    Ok(files)
}

pub unsafe fn path_exists(path: String) -> bool {
    let res = import_function::path_exists(path.as_ptr() as u64, path.len() as u64);
    if res == 0 {
        return false;
    }
    true
}

pub unsafe fn is_file(path: String) -> bool {
    let res = import_function::is_file(path.as_ptr() as u64, path.len() as u64);
    if res == 0 {
        return false;
    }
    true
}

pub unsafe fn create_dir(path: String) -> std::io::Result<()> {
    let res = import_function::create_dir(path.as_ptr() as u64, path.len() as u64);
    if res == 0 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Failed to create a directory at the given location",
        ));
    }
    Ok(())
}
