extern "C" {
    fn read_file_stream(identifier: i64, ptr_to_write: u32) -> u32;
    fn read_from_internet(identifier: i64, ptr_to_write: u32) -> u32;
}

pub struct BufferedReader(Vec<u8>);

impl BufferedReader {
    pub fn new() -> Self {
        Self(vec![0; 16 * 1024])
    }

    pub unsafe fn read_file_exact(&mut self, id: i64, bytes: u64) -> Vec<u8> {
        self.0.clear();
        read_file_stream(id, self.0.as_mut_ptr() as u32);
        return self.0[0..bytes as usize].to_vec();
    }

    pub unsafe fn read_file_to_end(&mut self, id: i64) -> Vec<u8> {
        self.0.clear();
        read_file_stream(id, self.0.as_mut_ptr() as u32);
        return self.0.clone();
    }

    // I'm not sure whether this function will be useful so I'll just comment out first
    // pub unsafe fn read_file_line(&mut self, id: i64) -> String {
    //     self.0.clear();
    //     read_file_stream(id, self.0.as_mut_ptr() as u32);
    //     let mut pos = self.0.len(); // EOF
    //     for i in 0..self.0.len() {
    //         if self.0[i] == 10 {
    //             // \n
    //             pos = i;
    //         }
    //     }
    //     return String::from_utf8(self.0[0..pos + 1].to_vec()).unwrap();
    // }

    pub unsafe fn read_internet_exact(&mut self, id: i64, bytes: u64) -> Vec<u8> {
        self.0.clear();
        read_from_internet(id, self.0.as_mut_ptr() as u32);
        return self.0[0..bytes as usize].to_vec();
    }

    pub unsafe fn read_internet_to_end(&mut self, id: i64) -> Vec<u8> {
        self.0.clear();
        read_from_internet(id, self.0.as_mut_ptr() as u32);
        return self.0.clone();
    }

    // I'm not sure whether this function will be useful so I'll just comment out first
    // pub unsafe fn read_internet_line(&mut self, id: i64) -> String {
    //     self.0.clear();
    //     read_from_internet(id, self.0.as_mut_ptr() as u32);
    //     let mut pos = self.0.len(); // EOF
    //     for i in 0..self.0.len() {
    //         if self.0[i] == 10 {
    //             // \n
    //             pos = i;
    //         }
    //     }
    //     return String::from_utf8(self.0[0..pos + 1].to_vec()).unwrap();
    // }
}
