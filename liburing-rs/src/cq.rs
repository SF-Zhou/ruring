use crate::{kernel, mmap};
use std::fs::File;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

pub struct CompleteQueue {
    pub ring: Arc<mmap::Buffer<u8>>,
    pub khead: &'static mut AtomicU32,
    pub ktail: &'static mut AtomicU32,
    pub kflags: &'static mut AtomicU32,
    pub koverflow: &'static mut AtomicU32,
    pub cqes: &'static mut [kernel::IoUringCqe],
    pub flags: u32,
    pub ring_mask: u32,
    pub ring_entries: u32,
}

impl CompleteQueue {
    pub fn new(
        fd: &File,
        p: &kernel::IoUringParams,
        buffer: &Arc<mmap::Buffer<u8>>,
    ) -> std::io::Result<Self> {
        if p.flags & kernel::IORING_SETUP_CQE32 != 0 {
            unimplemented!("IORING_SETUP_CQE32");
        }

        let ring = if p.features & kernel::IORING_FEAT_SINGLE_MMAP != 0 {
            buffer.clone()
        } else {
            const CQE_SZ: usize = std::mem::size_of::<kernel::IoUringCqe>();
            let cq_ring_sz = p.cq_off.cqes as usize + p.cq_entries as usize * CQE_SZ;
            Arc::new(mmap::Buffer::new(
                fd,
                kernel::IORING_OFF_CQ_RING,
                cq_ring_sz,
            )?)
        };

        Ok(Self {
            khead: ring.offset_as_mut(p.cq_off.head as usize),
            ktail: ring.offset_as_mut(p.cq_off.tail as usize),
            kflags: ring.offset_as_mut(p.cq_off.flags as usize),
            koverflow: ring.offset_as_mut(p.cq_off.overflow as usize),
            cqes: ring.offset_as_mut_slice(p.cq_off.cqes as usize, p.cq_entries as usize),
            flags: p.flags,
            ring_mask: *ring.offset_as_mut(p.cq_off.ring_mask as usize),
            ring_entries: *ring.offset_as_mut(p.cq_off.ring_entries as usize),
            ring,
        })
    }

    #[inline]
    pub fn needs_flush(&self) -> bool {
        self.kflags.load(Ordering::Relaxed)
            & (kernel::IORING_SQ_CQ_OVERFLOW | kernel::IORING_SQ_TASKRUN)
            != 0
    }

    #[inline]
    pub fn needs_enter(&self) -> bool {
        self.flags & kernel::IORING_SETUP_IOPOLL != 0 || self.needs_flush()
    }

    pub fn for_each_cqe<F>(&mut self, mut f: F) -> u32
    where
        F: FnMut(&kernel::IoUringCqe),
    {
        let mut entries = 0u32;
        let mut head = self.khead.load(Ordering::Relaxed);
        while head != self.ktail.load(Ordering::Acquire) {
            let index = head & self.ring_mask;
            let cqe = &self.cqes[index as usize];
            f(cqe);
            head += 1;
            entries += 1;
        }
        self.khead.store(head, Ordering::Release);
        entries
    }
}
