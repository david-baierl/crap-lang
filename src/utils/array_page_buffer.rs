use std::{alloc, mem, ptr};
extern crate page_size;

pub struct ArrayPageBuffer<T: Sized> {
    positive: Vec<ptr::NonNull<T>>,
    negative: Vec<ptr::NonNull<T>>,
    start: isize,
    end: isize,
}

impl<T> ArrayPageBuffer<T> {
    pub fn new() -> Self {
        ArrayPageBuffer::<T> {
            positive: vec![],
            negative: vec![],
            start: 0,
            end: 0,
        }
    }

    fn page_limit() -> usize {
        page_size::get() / mem::size_of::<T>()
    }

    unsafe fn alloc() -> ptr::NonNull<T> {
        let layout = alloc::Layout::array::<T>(ArrayPageBuffer::<T>::page_limit()).unwrap();
        assert!(layout.size() <= isize::MAX as usize, "Allocation too large");

        let ptr = alloc::alloc(layout);

        match ptr::NonNull::new(ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(layout),
        }
    }

    unsafe fn ptr(&self, index: isize) -> *mut T {
        let page_size = ArrayPageBuffer::<T>::page_limit();

        let (offset, buffer) = if index < 0 {
            (!(index as usize), &self.negative)
        } else {
            (index as usize, &self.positive)
        };

        buffer[offset / page_size].as_ptr().add(offset % page_size)
    }

    pub fn at(&self, index: usize) -> Option<&T> {
        if index > self.len() {
            return None;
        }

        let index = (index as i128 + self.start as i128) as isize;
        unsafe { Some(&*self.ptr(index)) }
    }

    pub fn len(&self) -> usize {
        (self.end as i128 - self.start as i128) as usize
    }

    /// adds the elements to the end
    pub fn push(&mut self, item: T) {
        assert!(
            self.end < isize::MAX,
            "ArrayPageBuffer::push max size reached"
        );

        // --- capacity --- //

        let cap = (self.positive.len() * ArrayPageBuffer::<T>::page_limit()) as isize;

        if cap == self.end {
            let page = unsafe { ArrayPageBuffer::<T>::alloc() };
            self.positive.push(page);
        }

        // --- write --- //

        unsafe { ptr::write(self.ptr(self.end), item) };
        self.end += 1;
    }

    /// adds the elements to the beginning
    pub fn unshift(&mut self, item: T) {
        assert!(
            self.start > isize::MIN,
            "ArrayPageBuffer::unshift max size reached"
        );

        // --- capacity --- //

        let cap = 0 - (self.negative.len() * ArrayPageBuffer::<T>::page_limit()) as isize;

        if cap == self.start {
            let page = unsafe { ArrayPageBuffer::<T>::alloc() };
            self.negative.push(page);
        }

        // --- write --- //

        unsafe { ptr::write(self.ptr(self.start), item) };
        self.start -= 1;
    }
}

impl<T: Copy> ArrayPageBuffer<T> {
    /// removes the last element and returns it
    pub fn pop(&mut self) -> Option<T> {
        if self.len() == 0 {
            return None;
        }

        self.end -= 1;
        unsafe { Some(ptr::read(self.ptr(self.end))) }
    }

    /// removes the first element and returns it
    pub fn shift(&mut self) -> Option<T> {
        if self.len() == 0 {
            return None;
        }

        self.start += 1;
        unsafe { Some(ptr::read(self.ptr(self.start))) }
    }
}

// unsafe impl<T: Send> Send for ArrayPageBuffer<T> {}
// unsafe impl<T: Sync> Sync for ArrayPageBuffer<T> {}

impl<T> Drop for ArrayPageBuffer<T> {
    fn drop(&mut self) {
        if !mem::needs_drop::<T>() {
            return;
        }

        for i in self.start..self.end {
            unsafe { ptr::drop_in_place(self.ptr(i)) };
        }
    }
}

// impl<T> From<&Vec<T>> for ArrayPageBuffer<T> {
//     fn from(value: &Vec<T>) -> Self {
//         todo!();
//     }
// }

// impl<T> From<Vec<T>> for ArrayPageBuffer<T> {
//     fn from(value: Vec<T>) -> Self {
//         todo!();
//     }
// }
