use crate::{flags::*, kernel, mmap};
use std::fs::File;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;

pub struct CompleteQueue {
    _ring: Arc<mmap::Buffer<u8>>,
    pub khead: &'static mut AtomicU32,
    pub ktail: &'static mut AtomicU32,
    _kflags: &'static mut atomic::Atomic<CQFlags>,
    _koverflow: &'static mut AtomicU32,
    pub cqes: &'static mut [kernel::IoUringCqe],
    pub ring_mask: u32,
    _ring_entries: u32,
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
            _kflags: ring.offset_as_mut(p.cq_off.flags as usize),
            _koverflow: ring.offset_as_mut(p.cq_off.overflow as usize),
            cqes: ring.offset_as_mut_slice(p.cq_off.cqes as usize, p.cq_entries as usize),
            ring_mask: *ring.offset_as_mut(p.cq_off.ring_mask as usize),
            _ring_entries: *ring.offset_as_mut(p.cq_off.ring_entries as usize),
            _ring: ring,
        })
    }
}
