use std::{fs::File, os::fd::AsRawFd};

pub struct Buffer<T: 'static> {
    buf: &'static mut [T],
}

impl<T> Buffer<T> {
    pub fn new(file: &File, offset: i64, len: usize) -> std::io::Result<Self> {
        let ptr = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                std::mem::size_of::<T>() * len,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED | libc::MAP_POPULATE,
                file.as_raw_fd(),
                offset,
            ) as *mut T
        };

        if Self::is_err(ptr) {
            return Err(Self::ptr_err(ptr));
        }

        Ok(Buffer {
            buf: unsafe { std::slice::from_raw_parts_mut(ptr, len) },
        })
    }

    pub fn offset_as_mut<O>(&self, offset: usize) -> &'static mut O {
        unsafe { &mut *(self.buf.as_ptr().wrapping_add(offset) as *mut O) }
    }

    pub fn offset_as_mut_slice<O>(&self, offset: usize, len: usize) -> &'static mut [O] {
        unsafe { std::slice::from_raw_parts_mut(self.buf.as_ptr().wrapping_add(offset) as _, len) }
    }

    #[inline]
    fn ptr_err<U>(ptr: *mut U) -> std::io::Error {
        std::io::Error::from_raw_os_error(-(ptr as i32))
    }

    #[inline]
    fn is_err<U>(ptr: *mut U) -> bool {
        ptr as usize >= -4095i64 as usize
    }
}

impl<T> Drop for Buffer<T> {
    fn drop(&mut self) {
        if !self.buf.is_empty() {
            unsafe { libc::munmap(self.buf.as_mut_ptr() as _, self.buf.len()) };
        }
    }
}

impl<T> std::ops::Index<usize> for Buffer<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buf[index]
    }
}

impl<T> std::ops::IndexMut<usize> for Buffer<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buf[index]
    }
}
