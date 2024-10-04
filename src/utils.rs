use std::{alloc, mem, ptr};

pub type Byte = u8;
pub type Bit = u8;

pub const BIT_1: Bit = 0b_0000_0001;
// pub const BIT_2: Bit = 0b_0000_0010;
// pub const BIT_3: Bit = 0b_0000_0100;
// pub const BIT_4: Bit = 0b_0000_1000;
// pub const BIT_5: Bit = 0b_0001_0000;
// pub const BIT_6: Bit = 0b_0010_0000;
// pub const BIT_7: Bit = 0b_0100_0000;
// pub const BIT_8: Bit = 0b_1000_0000;

pub trait BitArray {
    fn bit(&self) -> Bit;

    fn has(&self, flags: Byte) -> bool {
        (self.bit() & flags) != 0
    }
}

// ------------------------------------------
// buffer
// ------------------------------------------

pub struct Buffer<T>(ptr::NonNull<T>);

impl<T> Buffer<T> {
    pub unsafe fn new(size: usize) -> Buffer<T> {
        if size == 0 {
            return Buffer(ptr::NonNull::<T>::dangling());
        }

        let layout = alloc::Layout::array::<T>(size).unwrap();
        let ptr = alloc::alloc(layout);

        assert!(layout.size() <= isize::MAX as usize, "Allocation too large");

        Buffer(match ptr::NonNull::new(ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(layout),
        })
    }

    pub unsafe fn grow(&mut self, from: usize, to: usize) {
        let new_layout = alloc::Layout::array::<T>(to).unwrap();

        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Allocation too large"
        );

        let old_layout = alloc::Layout::array::<T>(from).unwrap();
        let old_ptr = self.0.as_ptr() as *mut u8;
        let new_ptr = alloc::realloc(old_ptr, old_layout, new_layout.size());

        self.0 = match ptr::NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
    }

    pub unsafe fn insert(&mut self, index: usize, elem: T) {
        ptr::write(self.0.as_ptr().add(index), elem);
    }

    pub unsafe fn at(&self, index: usize) -> &T {
        &*self.0.as_ptr().add(index.into())
    }

    unsafe fn free(&self, at: usize, size: usize) {
        let layout = alloc::Layout::array::<T>(size).unwrap();
        alloc::dealloc(self.0.as_ptr().add(at) as *mut u8, layout);
    }

    pub unsafe fn drop(&self, size: usize) {
        if size == 0 {
            return;
        }

        if mem::needs_drop::<T>() {
            for i in 0..(size - 1) {
                ptr::drop_in_place(self.0.as_ptr().add(i));
            }
        }

        self.free(0, size);
    }
}

pub trait BufferReader<T> {
    fn buf(&self) -> &Buffer<T>;
    fn len(&self) -> usize;

    fn at(&self, index: usize) -> Option<&T> {
        if index >= self.len() {
            return None;
        }

        unsafe { Some(self.buf().at(index)) }
    }

    fn drop_buf(&mut self) {
        let size = self.len();

        if size == 0 {
            return;
        }

        unsafe { self.buf().drop(size) };
    }
}

pub trait BufferWriter<T>: BufferReader<T> {
    fn mut_buf(&mut self) -> &mut Buffer<T>;
    fn set_len(&mut self, size: usize);

    fn grow(&mut self, amount: usize) {
        let from: usize = self.len().into();
        let to: usize = amount + from;

        self.set_len(to);

        unsafe { self.mut_buf().grow(from, to) };
    }

    fn insert(&mut self, index: usize, elem: T) {
        if self.len() == 0 || index >= self.len() {
            panic!("writing buffer out of bound")
        }

        unsafe { self.mut_buf().insert(index, elem) };
    }
}

// ------------------------------------------
// example
// ------------------------------------------

pub struct SmallVec<T> {
    buf: Buffer<T>,
    len: u16,
}

impl<T> SmallVec<T> {
    fn new() -> SmallVec<T> {
        SmallVec {
            buf: unsafe { Buffer::new(0) },
            len: 0,
        }
    }
}

impl<T> BufferReader<T> for SmallVec<T> {
    fn buf(&self) -> &Buffer<T> {
        &self.buf
    }

    fn len(&self) -> usize {
        self.len.into()
    }
}

impl<T> BufferWriter<T> for SmallVec<T> {
    fn mut_buf(&mut self) -> &mut Buffer<T> {
        &mut self.buf
    }

    fn set_len(&mut self, size: usize) {
        if size > u16::MAX.into() {
            panic!("Allocation too large")
        }

        self.len = size.try_into().unwrap();
    }
}

impl<T> Drop for SmallVec<T> {
    fn drop(&mut self) {
        self.drop_buf();
    }
}
