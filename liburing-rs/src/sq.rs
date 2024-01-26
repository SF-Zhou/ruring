use crate::{kernel, mmap};
use std::fs::File;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

pub struct SubmitQueue {
    pub ring: Arc<mmap::Buffer<u8>>,
    pub sqes: mmap::Buffer<kernel::IoUringSqe>,
    pub khead: &'static mut AtomicU32,
    pub ktail: &'static mut AtomicU32,
    pub kflags: &'static mut AtomicU32,
    pub kdropped: &'static mut AtomicU32,
    pub array: &'static mut [AtomicU32],
    pub flags: u32,
    pub sqe_head: u32,
    pub sqe_tail: u32,
    pub ring_mask: u32,
    pub ring_entries: u32,
}

impl SubmitQueue {
    pub fn new(fd: &File, p: &kernel::IoUringParams) -> std::io::Result<Self> {
        if p.flags & kernel::IORING_SETUP_SQE128 != 0 {
            unimplemented!("IORING_SETUP_SQE128");
        }

        const U32_SZ: usize = std::mem::size_of::<u32>();
        let mut sq_ring_sz = p.sq_off.array as usize + p.sq_entries as usize * U32_SZ;
        if p.features & kernel::IORING_FEAT_SINGLE_MMAP != 0 {
            const CQE_SZ: usize = std::mem::size_of::<kernel::IoUringCqe>();
            let cq_ring_sz = p.cq_off.cqes as usize + p.cq_entries as usize * CQE_SZ;
            if cq_ring_sz > sq_ring_sz {
                sq_ring_sz = cq_ring_sz;
            }
        }

        let sq_ring = mmap::Buffer::new(fd, kernel::IORING_OFF_SQ_RING, sq_ring_sz)?;
        let ring_mask = *sq_ring.offset_as_mut(p.sq_off.ring_mask as usize);
        let ring_entries = *sq_ring.offset_as_mut(p.sq_off.ring_entries as usize);
        let array: &mut [AtomicU32] =
            sq_ring.offset_as_mut_slice(p.sq_off.array as usize, p.sq_entries as usize);
        for index in 0..ring_entries {
            array[index as usize].store(index, Ordering::Release);
        }

        Ok(Self {
            sqes: mmap::Buffer::new(fd, kernel::IORING_OFF_SQES, p.sq_entries as usize)?,
            khead: sq_ring.offset_as_mut(p.sq_off.head as usize),
            ktail: sq_ring.offset_as_mut(p.sq_off.tail as usize),
            kflags: sq_ring.offset_as_mut(p.sq_off.flags as usize),
            kdropped: sq_ring.offset_as_mut(p.sq_off.dropped as usize),
            array,
            flags: p.flags,
            sqe_head: 0,
            sqe_tail: 0,
            ring_mask,
            ring_entries,
            ring: std::sync::Arc::new(sq_ring),
        })
    }

    pub(crate) fn get_sqe(&mut self) -> std::io::Result<&mut kernel::IoUringSqe> {
        let next = self.sqe_tail + 1;

        let head = if self.flags & kernel::IORING_SETUP_SQPOLL != 0 {
            self.khead.load(Ordering::Relaxed)
        } else {
            self.khead.load(Ordering::Acquire)
        };

        if next - head <= self.ring_entries {
            let index = (self.sqe_tail & self.ring_mask) as usize;
            self.sqe_tail = next;
            Ok(&mut self.sqes[index])
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::OutOfMemory,
                "sqe ring is empty!",
            ))
        }
    }

    pub fn nop(&mut self, user_data: u64) -> std::io::Result<()> {
        let sqe = self.get_sqe()?;
        *sqe = kernel::IoUringSqe {
            opcode: kernel::IORING_OP_NOP,
            fd: -1,
            user_data,
            ..Default::default()
        };
        Ok(())
    }

    pub fn flush(&mut self) -> u32 {
        let tail = self.sqe_tail;

        if self.sqe_head != tail {
            self.sqe_head = tail;

            if self.flags & kernel::IORING_SETUP_SQPOLL != 0 {
                self.ktail.store(tail, Ordering::Relaxed);
            } else {
                self.ktail.store(tail, Ordering::Release);
            }
        }

        tail - self.khead.load(Ordering::Relaxed)
    }

    #[inline]
    pub(crate) fn needs_enter(&self, submitted: u32, flags: &mut u32) -> bool {
        if submitted == 0 {
            return false;
        }

        if self.flags & kernel::IORING_SETUP_SQPOLL == 0 {
            return true;
        }

        std::sync::atomic::fence(Ordering::SeqCst);

        if self.kflags.load(Ordering::Relaxed) & kernel::IORING_SQ_NEED_WAKEUP != 0 {
            *flags |= kernel::IORING_ENTER_SQ_WAKEUP;
            true
        } else {
            false
        }
    }
}

mod tests {
    #[test]
    fn get_sqe() -> std::io::Result<()> {
        use crate::*;

        let entries = 128;
        let mut params = kernel::IoUringParams::default();
        let mut io_uring = IoUring::new(entries, &mut params)?;

        for _ in 0..entries {
            io_uring.sq.get_sqe()?;
        }
        let result = io_uring.sq.get_sqe();
        assert!(result.is_err());

        Ok(())
    }
}