use std::os::fd::AsRawFd;

// A wrapper of buffer allocated by mmap with ownership.
pub struct MmapBuffer(&'static mut [u8]);

impl std::ops::Deref for MmapBuffer {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl std::ops::DerefMut for MmapBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

impl Drop for MmapBuffer {
    fn drop(&mut self) {
        unsafe { libc::munmap(self.0.as_mut_ptr() as _, self.0.len()) };
    }
}

impl MmapBuffer {
    pub fn file_mapping(fd: &impl AsRawFd, offset: i64, len: usize) -> std::io::Result<Self> {
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

        Ok(MmapBuffer(unsafe {
            std::slice::from_raw_parts_mut(ptr, len)
        }))
    }

    pub fn offset_as_mut<O>(&self, offset: usize) -> &'static mut O {
        unsafe { &mut *(self.0.as_ptr().wrapping_add(offset) as *mut O) }
    }

    pub fn offset_as_mut_slice<O>(&self, offset: usize, len: usize) -> &'static mut [O] {
        unsafe { std::slice::from_raw_parts_mut(self.0.as_ptr().wrapping_add(offset) as _, len) }
    }
}

mod tests {
    #[test]
    fn mmap_buffer_anonymous() -> std::io::Result<()> {
        use super::*;

        const LEN: usize = 1024;
        let mut mmap_buffer = MmapBuffer::anonymous(LEN)?;
        assert_eq!(mmap_buffer.len(), LEN);
        assert_eq!(mmap_buffer.as_ptr(), mmap_buffer.as_mut_ptr());

        let a: &mut [u8; LEN] = mmap_buffer.offset_as_mut(0);
        for (idx, value) in a.iter_mut().enumerate() {
            *value = idx as _;
        }

        const OFFSET: usize = 233;
        let b: &mut [u8; LEN / 2] = mmap_buffer.offset_as_mut(OFFSET);
        for (idx, &value) in b.iter().enumerate() {
            assert_eq!(value, (idx + OFFSET) as _);
        }

        let c: &mut [u8] = mmap_buffer.offset_as_mut_slice(0, LEN);
        assert_eq!(a, c);

        Ok(())
    }
}
