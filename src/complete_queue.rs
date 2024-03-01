use crate::{constants, flags, syscall, CompleteQueueEntry, IoUringParams, MmapBuffer};
use std::os::fd::OwnedFd;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

pub struct CompleteQueue {
    pub ring_fd: Arc<OwnedFd>,
    _ring: Arc<MmapBuffer>,
    pub khead: &'static mut AtomicU32,
    pub ktail: &'static mut AtomicU32,
    _kflags: &'static mut atomic::Atomic<flags::CQFlags>,
    _koverflow: &'static mut AtomicU32,
    pub cqes: &'static mut [CompleteQueueEntry],
    pub ring_mask: u32,
    _ring_entries: u32,
}

impl CompleteQueue {
    pub fn new(
        ring_fd: Arc<OwnedFd>,
        p: &IoUringParams,
        sq_ring: &Arc<MmapBuffer>,
    ) -> std::io::Result<Self> {
        if p.flags.contains(flags::SetupFlags::CQE32) {
            unimplemented!("IORING_SETUP_CQE32");
        }

        let ring = if p.features.contains(flags::FeatureFlags::SINGLE_MMAP) {
            sq_ring.clone()
        } else {
            const CQE_SZ: usize = std::mem::size_of::<CompleteQueueEntry>();
            let cq_ring_sz = p.cq_off.cqes as usize + p.cq_entries as usize * CQE_SZ;
            Arc::new(MmapBuffer::file_mapping(
                &ring_fd,
                constants::Offset::CQ_RING.bits(),
                cq_ring_sz,
            )?)
        };

        Ok(Self {
            ring_fd,
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

    pub fn reap(&mut self, wait_nr: u32) -> std::io::Result<()> {
        syscall::io_uring_enter(
            &self.ring_fd,
            0,
            wait_nr,
            flags::EnterFlags::GETEVENTS,
            std::ptr::null_mut(),
        )
        .map(|_| ())
    }

    pub fn for_each_cqe<F>(&mut self, mut f: F) -> u32
    where
        F: FnMut(&CompleteQueueEntry),
    {
        let mut entries = 0u32;
        let mut head = self.khead.load(Ordering::Relaxed);
        while head != self.ktail.load(Ordering::Acquire) {
            let index = head & self.ring_mask;
            let cqe = &self.cqes[index as usize];

            f(cqe);

            head = head.wrapping_add(1);
            entries += 1;
        }
        self.khead.store(head, Ordering::Release);
        entries
    }
}
