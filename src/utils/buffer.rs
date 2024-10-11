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

    pub unsafe fn realloc<T>(ptr: *mut T, from: usize, to: usize) -> ptr::NonNull<T> {
        assert!(to > 0, "Allocation too small");
        let layout = alloc::Layout::array::<T>(to).unwrap();
        assert!(layout.size() <= isize::MAX as usize, "Allocation too large");

        let old_layout = alloc::Layout::array::<T>(from).unwrap();

        let new_ptr = alloc::realloc(ptr as *mut u8, old_layout, layout.size());

        match ptr::NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(layout),
        }
    }

    pub unsafe fn copy<T>(source: *const T, target: *mut T, size: usize) {
        ptr::copy_nonoverlapping(source, target, size);
    }

    pub unsafe fn free<T>(ptr: *mut T, size: usize) {
        if size == 0 {
            return
        }

        drop::<T>(ptr, 0, size - 1);
        let layout = alloc::Layout::array::<T>(size).unwrap();
        alloc::dealloc(ptr as *mut u8, layout);
    }

    pub unsafe fn drop<T>(ptr: *mut T, start: usize, stop: usize) {
        if !mem::needs_drop::<T>() {
            return;
        }

        for i in start..stop {
            ptr::drop_in_place(ptr.add(i));
        }
    }

    pub unsafe fn insert<T>(ptr: *mut T, index: usize, elem: T) {
        ptr::write(ptr.add(index), elem);
    }

    pub unsafe fn at<'a, T>(ptr: *const T, index: usize) -> &'a T {
        &*ptr.add(index.into())
    }
}

// ------------------------------------------
// buffer
// ------------------------------------------

pub struct Buffer<T> {
    ptr: ptr::NonNull<T>,
}

impl<T> Buffer<T> {
    pub unsafe fn new(size: usize) -> Self {
        Buffer {
            ptr: unsafe { heap::alloc::<T>(size) },
        }
    }

    pub unsafe fn drop(&self, size: usize) {
        unsafe { heap::free::<T>(self.ptr.as_ptr(), size) };
    }
}

impl<T> From<&Vec<T>> for Buffer<T> {
    fn from(value: &Vec<T>) -> Self {
        let buf = Buffer {
            ptr: unsafe { heap::alloc::<T>(value.len()) },
        };

        unsafe { heap::copy::<T>(value.as_ptr(), buf.ptr.as_ptr(), value.len()) };

        buf
    }
}

pub trait BufferReader<T> {
    fn len(&self) -> usize;
    fn buf(&self) -> &Buffer<T>;

    fn at(&self, index: usize) -> Option<&T> {
        if index >= self.len() {
            return None;
        }

        unsafe { Some(heap::at::<T>(self.buf().ptr.as_ptr(), index)) }
    }
}

pub trait BufferWriter<T>: BufferReader<T> {
    fn cap(&self) -> usize;
    fn set_cap(&mut self, size: usize);
    fn set_len(&mut self, size: usize);
    fn mut_buf(&mut self) -> &mut Buffer<T>;

    fn extract(&mut self) -> Buffer<T> {
        let buf = Buffer {
            ptr: self.buf().ptr,
        };

        self.mut_buf().ptr = ptr::NonNull::<T>::dangling();
        self.set_cap(0);
        self.set_len(0);

        buf
    }

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
                heap::alloc::<T>(to)
            } else {
                if to < len {
                    heap::drop::<T>(ptr.as_ptr(), to - 1, len - 1);
                }
                heap::realloc::<T>(ptr.as_ptr(), cap, to)
            }
        };

        self.set_cap(to);

        if to < len {
            self.set_len(to);
        }
    }

    fn append(&mut self, elem: T) {
        let index = self.len();
        let length = self.len() + 1;

        if length > self.cap() {
            self.resize(length);
        }

        unsafe { heap::insert::<T>(self.buf().ptr.as_ptr(), index, elem) };
        self.set_len(length)
    }

    fn concat(&mut self, source: &dyn BufferReader<T>) {
        let old_length = self.len();
        let new_length = self.len() + source.len();

        if new_length > self.cap() {
            self.resize(new_length);
        }

        unsafe {
            heap::copy::<T>(
                source.buf().ptr.as_ptr(),
                self.buf().ptr.as_ptr().add(old_length),
                source.len(),
            )
        };
        self.set_len(new_length)
    }
}
