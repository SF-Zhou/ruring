use crate::{constants, flags, CompleteQueueEntry, IoUringParams, MmapBuffer, SubmitQueueEntry};
use std::os::fd::AsRawFd;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

pub struct SubmitQueue {
    pub khead: &'static mut AtomicU32,
    pub ktail: &'static mut AtomicU32,
    pub kflags: &'static mut atomic::Atomic<flags::SQFlags>,
    pub kdropped: &'static mut AtomicU32,
    pub array: &'static mut [AtomicU32],
    pub sqes: &'static mut [SubmitQueueEntry],
    pub flags: flags::SetupFlags,
    pub sqe_head: u32,
    pub sqe_tail: u32,
    pub ring_mask: u32,
    pub ring_entries: u32,
    pub ring_buffer: Arc<MmapBuffer>,
    pub sqes_buffer: MmapBuffer,
}

impl ::std::fmt::Debug for SubmitQueue {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("SubmitQueue")
    }
}

impl SubmitQueue {
    pub fn new(fd: &impl AsRawFd, p: &IoUringParams) -> std::io::Result<Self> {
        if p.flags.contains(flags::SetupFlags::SQE128) {
            unimplemented!("IORING_SETUP_SQE128");
        }

        const U32_SZ: usize = std::mem::size_of::<u32>();
        let mut sq_ring_sz = p.sq_off.array as usize + p.sq_entries as usize * U32_SZ;
        if p.features.contains(flags::FeatureFlags::SINGLE_MMAP) {
            const CQE_SZ: usize = std::mem::size_of::<CompleteQueueEntry>();
            let cq_ring_sz = p.cq_off.cqes as usize + p.cq_entries as usize * CQE_SZ;
            if cq_ring_sz > sq_ring_sz {
                sq_ring_sz = cq_ring_sz;
            }
        }

        let sq_ring = MmapBuffer::file_mapping(fd, constants::Offset::SQ_RING.bits(), sq_ring_sz)?;
        let ring_mask = *sq_ring.offset_as_mut(p.sq_off.ring_mask as usize);
        let ring_entries = *sq_ring.offset_as_mut(p.sq_off.ring_entries as usize);
        let array: &mut [AtomicU32] =
            sq_ring.offset_as_mut_slice(p.sq_off.array as usize, p.sq_entries as usize);
        for index in 0..ring_entries {
            array[index as usize].store(index, Ordering::Release);
        }

        let sqes_size = p.sq_entries as usize * std::mem::size_of::<SubmitQueueEntry>();
        let sqes_buffer = MmapBuffer::file_mapping(fd, constants::Offset::SQES.bits(), sqes_size)?;

        Ok(Self {
            khead: sq_ring.offset_as_mut(p.sq_off.head as usize),
            ktail: sq_ring.offset_as_mut(p.sq_off.tail as usize),
            kflags: sq_ring.offset_as_mut(p.sq_off.flags as usize),
            kdropped: sq_ring.offset_as_mut(p.sq_off.dropped as usize),
            array,
            sqes: sqes_buffer.offset_as_mut_slice(0, p.sq_entries as _),
            flags: p.flags,
            sqe_head: 0,
            sqe_tail: 0,
            ring_mask,
            ring_entries,
            ring_buffer: std::sync::Arc::new(sq_ring),
            sqes_buffer,
        })
    }

    pub fn get_sqe(&mut self) -> std::io::Result<&mut SubmitQueueEntry> {
        let next = self.sqe_tail + 1;

        let head = if self.flags.contains(flags::SetupFlags::SQPOLL) {
            self.khead.load(Ordering::Acquire)
        } else {
            self.khead.load(Ordering::Relaxed)
        };

        if next - head <= self.ring_entries {
            let index = (self.sqe_tail & self.ring_mask) as usize;
            self.sqe_tail = next;
            Ok(&mut self.sqes[index])
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::OutOfMemory,
                "sqe ring buffer is empty!",
            ))
        }
    }

    pub fn flush(&mut self) -> u32 {
        let tail = self.sqe_tail;

        if self.sqe_head != tail {
            self.sqe_head = tail;

            if self.flags.contains(flags::SetupFlags::SQPOLL) {
                self.ktail.store(tail, Ordering::Release);
            } else {
                self.ktail.store(tail, Ordering::Relaxed);
            }
        }

        tail - self.khead.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn needs_enter(&self, submitted: u32, flags: &mut flags::EnterFlags) -> bool {
        if submitted == 0 {
            return false;
        }

        if !self.flags.contains(flags::SetupFlags::SQPOLL) {
            return true;
        }

        std::sync::atomic::fence(Ordering::SeqCst);

        if self
            .kflags
            .load(Ordering::Relaxed)
            .contains(flags::SQFlags::NEED_WAKEUP)
        {
            flags.insert(flags::EnterFlags::SQ_WAKEUP);
            true
        } else {
            false
        }
    }
}
