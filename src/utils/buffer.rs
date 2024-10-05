use std::ptr;

// ------------------------------------------
// heap
// ------------------------------------------

mod heap {
    use std::{alloc, mem, ptr};

    pub unsafe fn alloc<T>(size: usize) -> ptr::NonNull<T> {
        if size == 0 {
            return ptr::NonNull::<T>::dangling();
        }

        let layout = alloc::Layout::array::<T>(size).unwrap();
        assert!(layout.size() <= isize::MAX as usize, "Allocation too large");

        let ptr = alloc::alloc(layout);

        match ptr::NonNull::new(ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(layout),
        }
    }

    pub unsafe fn realloc<T>(ptr: ptr::NonNull<T>, from: usize, to: usize) -> ptr::NonNull<T> {
        assert!(to > 0, "Allocation too small");
        let layout = alloc::Layout::array::<T>(to).unwrap();
        assert!(layout.size() <= isize::MAX as usize, "Allocation too large");

        let old_ptr = ptr.as_ptr() as *mut u8;
        let old_layout = alloc::Layout::array::<T>(from).unwrap();

        let new_ptr = alloc::realloc(old_ptr, old_layout, layout.size());

        match ptr::NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(layout),
        }
    }

    pub unsafe fn copy<T>(source: ptr::NonNull<T>, target: ptr::NonNull<T>, size: usize) {
        ptr::copy_nonoverlapping(source.as_ptr(), target.as_ptr(), size);
    }

    pub unsafe fn free<T>(ptr: ptr::NonNull<T>, size: usize) {
        drop(ptr, 0, size - 1);
        let layout = alloc::Layout::array::<T>(size).unwrap();
        alloc::dealloc(ptr.as_ptr() as *mut u8, layout);
    }

    pub unsafe fn drop<T>(ptr: ptr::NonNull<T>, start: usize, stop: usize) {
        if !mem::needs_drop::<T>() {
            return;
        }

        for i in start..stop {
            ptr::drop_in_place(ptr.as_ptr().add(i));
        }
    }

    pub unsafe fn insert<T>(ptr: ptr::NonNull<T>, index: usize, elem: T) {
        ptr::write(ptr.as_ptr().add(index), elem);
    }

    pub unsafe fn at<'a, T>(ptr: ptr::NonNull<T>, index: usize) -> &'a T {
        &*ptr.as_ptr().add(index.into())
    }
}

// ------------------------------------------
// buffer
// ------------------------------------------

pub struct Buffer<T> {
    ptr: ptr::NonNull<T>,
}

impl<T> Buffer<T> {
    pub unsafe fn new(size: usize) -> Buffer<T> {
        Buffer {
            ptr: unsafe { heap::alloc(size) },
        }
    }

    pub unsafe fn drop(&mut self, size: usize) {
        unsafe { heap::free(self.ptr, size) };
    }
}

pub trait BufferReader<T> {
    fn len(&self) -> usize;
    fn buf(&self) -> &Buffer<T>;

    fn at(&self, index: usize) -> Option<&T> {
        if index >= self.len() {
            return None;
        }

        unsafe { Some(heap::at(self.buf().ptr, index)) }
    }
}

pub trait BufferWriter<T>: BufferReader<T> {
    fn cap(&self) -> usize;
    fn set_cap(&mut self, size: usize);
    fn set_len(&mut self, size: usize);
    fn mut_buf(&mut self) -> &mut Buffer<T>;

    fn resize(&mut self, to: usize) {
        assert!(to != 0, "Resizing to 0 is not allowed");

        let cap = self.cap();
        let len = self.len();
        let ptr: ptr::NonNull<T> = self.buf().ptr;

        if cap == to {
            return;
        }

        self.mut_buf().ptr = unsafe {
            if cap == 0 {
                heap::alloc(to)
            } else {
                if to < len {
                    heap::drop(ptr, to - 1, len - 1);
                }
                heap::realloc(ptr, cap, to)
            }
        };

        self.set_cap(to);

        if to < len {
            self.set_len(to);
        }
    }

    fn append(&mut self, elem: T) {
        let index = self.len();
        assert!(index < self.cap(), "writing buffer out of bound");

        unsafe { heap::insert(self.buf().ptr, index, elem) };
        self.set_len(index + 1)
    }
}

// ------------------------------------------
// example
// ------------------------------------------

// pub struct SmallVec<T> {
//     buf: Buffer<T>,
//     len: u16,
//     cap: u16,
// }

// impl<T> SmallVec<T> {
//     pub fn new() -> SmallVec<T> {
//         SmallVec {
//             buf: unsafe { Buffer::new(0) },
//             len: 0,
//             cap: 0,
//         }
//     }
// }

// impl<T> Drop for SmallVec<T> {
//     fn drop(&mut self) {
//         unsafe { self.buf.drop(self.cap()) };
//     }
// }

// impl<T> BufferReader<T> for SmallVec<T> {
//     fn buf(&self) -> &Buffer<T> {
//         &self.buf
//     }
//     fn len(&self) -> usize {
//         self.len.into()
//     }
// }

// impl<T> BufferWriter<T> for SmallVec<T> {
//     fn cap(&self) -> usize {
//         self.cap.into()
//     }
//     fn set_cap(&mut self, size: usize) {
//         assert!(size < u16::MAX.into(), "Allocation too large");
//         self.cap = size.try_into().unwrap();
//     }
//     fn mut_buf(&mut self) -> &mut Buffer<T> {
//         &mut self.buf
//     }
//     fn set_len(&mut self, size: usize) {
//         self.len = size.try_into().unwrap();
//     }
// }
