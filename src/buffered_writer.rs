extern "C" {
    fn write_file_stream(identifier: i64, ptr_to_buffer: u32, length_buffer: u32) -> u64;
}

pub struct BufferedWriter;

impl BufferedWriter {
    pub fn new() -> Self {
        Self
    }

    pub unsafe fn write(&mut self, id: i64, buf: Vec<u8>) {
        write_file_stream(id, buf.as_ptr() as u32, buf.len() as u32);
    }

    // Again, I'm not sure whether this is needed, so I'll just comment it out first
    // pub unsafe fn write_vectored(&mut self, id: i64, buf: Vec<Vec<u8>>) {
    //     for b in buf {
    //         write_file_stream(id, b.as_ptr() as u32, b.len() as u32);
    //     }
    // }
}
