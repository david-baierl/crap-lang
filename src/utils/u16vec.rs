use super::buffer::{Buffer, BufferReader, BufferWriter};

pub struct U16Vec<T> {
    buf: Buffer<T>,
    length: u16,
    capacity: u16,
}

impl<T> U16Vec<T> {
    pub fn u16_len(&self) -> u16 {
        self.length
    }

    // pub fn u16_cap(&self) -> u16 {
    //     self.length
    // }

    pub fn new() -> Self {
        U16Vec::<T> {
            buf: unsafe { Buffer::new(0) },
            capacity: 0,
            length: 0,
        }
    }
}

impl<T> Drop for U16Vec<T> {
    fn drop(&mut self) {
        unsafe { self.buf.drop(self.cap()) };
    }
}

impl<T> BufferReader<T> for U16Vec<T> {
    fn buf(&self) -> &Buffer<T> {
        &self.buf
    }

    fn len(&self) -> usize {
        self.length.into()
    }
}

impl<T> BufferWriter<T> for U16Vec<T> {
    fn cap(&self) -> usize {
        self.capacity.into()
    }

    fn set_cap(&mut self, size: usize) {
        self.capacity = size.try_into().unwrap()
    }

    fn set_len(&mut self, size: usize) {
        self.length = size.try_into().unwrap()
    }

    fn mut_buf(&mut self) -> &mut Buffer<T> {
        &mut self.buf
    }
}
