use atomic::Ordering;

use crate::{kernel, mmap, syscall};
use std::sync::atomic::AtomicU16;
use std::sync::Arc;

pub struct BufferGroup {
    pub bgid: u16,
    pub tail: &'static mut AtomicU16,
    ring_fd: Arc<std::fs::File>,
    _buffer: mmap::Buffer<kernel::IoUringBuf>,
}

impl BufferGroup {
    pub fn new(
        ring_fd: Arc<std::fs::File>,
        buffer: mmap::Buffer<kernel::IoUringBuf>,
        bgid: u16,
    ) -> Self {
        let tail = buffer.offset_as_mut::<AtomicU16>(0xE);
        tail.store(0u16, Ordering::Relaxed);
        Self {
            bgid,
            tail,
            ring_fd,
            _buffer: buffer,
        }
    }
}

impl Drop for BufferGroup {
    fn drop(&mut self) {
        let mut reg = kernel::IoUringBufReg {
            bgid: self.bgid,
            ..Default::default()
        };
        let _ = syscall::io_uring_register(
            &self.ring_fd,
            kernel::IORING_UNREGISTER_PBUF_RING,
            &mut reg as *mut _ as _,
            1,
        );
    }
}
