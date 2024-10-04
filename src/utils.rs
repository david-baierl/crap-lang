use std::{
    alloc::{self, Layout},
    ops::{Deref, DerefMut},
    ptr::{self, NonNull},
    slice,
};

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

pub struct SmallVec<T> {
    ptr: NonNull<T>,
    len: u16,
}

impl<T> SmallVec<T> {
    fn new() -> SmallVec<T> {
        SmallVec {
            ptr: NonNull::dangling(),
            len: 0,
        }
    }

    fn grow(&mut self, amount: u16) {
        let size = amount + self.len;
        let new_layout = Layout::array::<T>(size.into()).unwrap();

        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Allocation too large"
        );

        let new_ptr = if self.len == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(size.into()).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        self.len = size;
        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
    }

    pub fn push(&mut self, elem: T) {
        let index: usize = self.len.into();
        self.grow(1);

        unsafe {
            ptr::write(self.ptr.as_ptr().add(index), elem);
        }
    }

    pub fn at(&self, index: u16) -> Option<T> {
        if self.len == 0 || index >= self.len {
            return None;
        }

        unsafe { Some(ptr::read(self.ptr.as_ptr().add(index.into()))) }
    }
}

impl<T> From<Vec<T>> for SmallVec<T> {
    fn from(mut source: Vec<T>) -> Self {
        let mut target = SmallVec::<T>::new();
        target.grow(source.len().try_into().unwrap());

        while let Some(elem) = source.pop() {
            let index = source.len() - 1;
            unsafe {
                ptr::write(target.ptr.as_ptr().add(index), elem);
            }
        }

        target
    }
}

impl<T> Drop for SmallVec<T> {
    fn drop(&mut self) {
        if self.len == 0 {
            return;
        }

        let layout = Layout::array::<T>(self.len.into()).unwrap();
        unsafe {
            alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
    }
}

impl<T> Deref for SmallVec<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len.into()) }
    }
}

impl<T> DerefMut for SmallVec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len.into()) }
    }
}
