use crate::{flags::*, kernel, mmap};
use std::fs::File;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

pub struct CompleteQueue {
    pub ring: Arc<mmap::Buffer<u8>>,
    pub khead: &'static mut AtomicU32,
    pub ktail: &'static mut AtomicU32,
    pub kflags: &'static mut atomic::Atomic<CQFlags>,
    pub sq_kflags: &'static mut atomic::Atomic<SQFlags>,
    pub koverflow: &'static mut AtomicU32,
    pub cqes: &'static mut [kernel::IoUringCqe],
    pub flags: SetupFlags,
    pub ring_mask: u32,
    pub ring_entries: u32,
}

impl ::std::fmt::Debug for CompleteQueue {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("CompleteQueue")
    }
}

impl CompleteQueue {
    pub fn new(
        fd: &File,
        p: &kernel::IoUringParams,
        sq_ring: &Arc<mmap::Buffer<u8>>,
    ) -> std::io::Result<Self> {
        if p.flags.contains(SetupFlags::CQE32) {
            unimplemented!("IORING_SETUP_CQE32");
        }

        let ring = if p.features.contains(FeatureFlags::SINGLE_MMAP) {
            sq_ring.clone()
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
            sq_kflags: sq_ring.offset_as_mut(p.sq_off.flags as usize),
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
        self.sq_kflags
            .load(Ordering::Relaxed)
            .intersects(SQFlags::CQ_OVERFLOW | SQFlags::TASKRUN)
    }

    #[inline]
    pub fn needs_enter(&self) -> bool {
        self.flags.contains(SetupFlags::IOPOLL) || self.needs_flush()
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
