use atomic::Ordering;

use crate::{kernel, mmap, syscall};
use std::sync::Arc;

pub struct BufferGroup {
    pub bgid: u16,
    pub entries: u32,
    pub size: usize,
    ring_fd: Arc<std::fs::File>,
    pub bufs: mmap::Buffer<kernel::IoUringBuf>,
    vec: Vec<u8>,
}

impl BufferGroup {
    pub fn new(
        ring_fd: Arc<std::fs::File>,
        mut bufs: mmap::Buffer<kernel::IoUringBuf>,
        bgid: u16,
        entries: u32,
        size: usize,
    ) -> Self {
        let sz = size * entries as usize;
        let layout = std::alloc::Layout::from_size_align(sz, 4096).unwrap();
        let mut vec = unsafe { Vec::<u8>::from_raw_parts(std::alloc::alloc(layout), sz, sz) };

        for bid in 0..entries {
            let buf = &mut bufs[bid as usize];
            buf.addr = (&mut vec[size * bid as usize]) as *mut _ as _;
            buf.len = size as _;
            buf.bid = 1u16 + bid as u16;
        }
        bufs[0].tail.store(entries as u16, Ordering::Release);

        Self {
            bgid,
            entries,
            size,
            ring_fd,
            bufs,
            vec,
        }
    }

    pub fn iovec(&mut self) -> libc::iovec {
        libc::iovec {
            iov_base: &mut self.vec[0] as *mut _ as _,
            iov_len: self.vec.len(),
        }
    }

    pub fn addr(&mut self, bid: u16) -> u64 {
        &mut self.vec[self.size * (bid - 1) as usize] as *mut _ as _
    }

    pub fn recycle(&mut self, bid: u16) {
        let tail = self.bufs[0].tail.load(Ordering::Relaxed);
        let addr = self.addr(bid);
        let buf = &mut self.bufs[(tail as u32 & (self.entries - 1)) as usize];
        buf.addr = addr;
        buf.len = self.size as _;
        buf.bid = bid;
        self.bufs[0].tail.fetch_add(1, Ordering::Relaxed);
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
