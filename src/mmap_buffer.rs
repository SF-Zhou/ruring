use std::os::fd::AsRawFd;

// A wrapper of buffer allocated by mmap with ownership.
pub struct MmapBuffer {
    buf: &'static mut [u8],
}

impl Drop for MmapBuffer {
    fn drop(&mut self) {
        unsafe { libc::munmap(self.buf.as_mut_ptr() as _, self.buf.len()) };
    }
}

impl MmapBuffer {
    pub fn at_offset(fd: &impl AsRawFd, offset: i64, len: usize) -> std::io::Result<Self> {
        let ptr = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                len,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED | libc::MAP_POPULATE,
                fd.as_raw_fd(),
                offset,
            )
        };

        Self::from_ptr_and_len(ptr as _, len)
    }

    pub fn anonymous(len: usize) -> std::io::Result<Self> {
        let ptr = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                len,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_ANONYMOUS | libc::MAP_PRIVATE,
                -1,
                0,
            )
        };

        Self::from_ptr_and_len(ptr as _, len)
    }

    #[inline]
    fn from_ptr_and_len(ptr: *mut u8, len: usize) -> std::io::Result<Self> {
        if ptr as usize >= -4095i64 as usize {
            return Err(std::io::Error::from_raw_os_error(-(ptr as i32)));
        }

        Ok(MmapBuffer {
            buf: unsafe { std::slice::from_raw_parts_mut(ptr, len) },
        })
    }

    fn as_ptr(&self) -> *const u8 {
        self.buf.as_ptr()
    }

    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.buf.as_mut_ptr()
    }

    fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn offset_as_mut<O>(&self, offset: usize) -> &'static mut O {
        unsafe { &mut *(self.buf.as_ptr().wrapping_add(offset) as *mut O) }
    }

    pub fn offset_as_mut_slice<O>(&self, offset: usize, len: usize) -> &'static mut [O] {
        unsafe { std::slice::from_raw_parts_mut(self.buf.as_ptr().wrapping_add(offset) as _, len) }
    }
}

mod tests {
    #[test]
    fn mmap_buffer_anonymous() -> std::io::Result<()> {
        use super::*;

        const LEN: usize = 1024;
        let mmap_buffer = MmapBuffer::anonymous(LEN)?;
        assert_eq!(mmap_buffer.len(), LEN);

        let a: &mut [u8; LEN] = mmap_buffer.offset_as_mut(0);
        for i in 0..LEN {
            a[i] = i as _;
        }

        const OFFSET: usize = 233;
        let b: &mut [u8; LEN / 2] = mmap_buffer.offset_as_mut(OFFSET);
        for i in 0..b.len() {
            assert_eq!(b[i], (i + OFFSET) as _);
        }

        let c: &mut [u8] = mmap_buffer.offset_as_mut_slice(0, LEN);
        assert_eq!(a, c);

        Ok(())
    }
}
