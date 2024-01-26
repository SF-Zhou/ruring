use crate::{cq::CompleteQueue, kernel, sq::SubmitQueue, syscall};

pub struct IoUring {
    pub ring_fd: std::fs::File,
    pub sq: SubmitQueue,
    pub cq: CompleteQueue,
    pub flags: u32,
    pub features: u32,
}

impl IoUring {
    pub fn new(entries: u32, p: &mut kernel::IoUringParams) -> std::io::Result<IoUring> {
        if p.flags & kernel::IORING_SETUP_REGISTERED_FD_ONLY != 0
            && p.flags & kernel::IORING_SETUP_NO_MMAP == 0
        {
            return Err(std::io::ErrorKind::InvalidInput.into());
        }

        if p.flags & kernel::IORING_SETUP_NO_MMAP != 0 {
            unimplemented!("IORING_SETUP_NO_MMAP");
        }
        if p.flags & kernel::IORING_SETUP_REGISTERED_FD_ONLY != 0 {
            unimplemented!("IORING_SETUP_REGISTERED_FD_ONLY");
        }

        let fd = syscall::io_uring_setup(entries, p)?;
        let sq = SubmitQueue::new(&fd, p)?;
        let cq = CompleteQueue::new(&fd, p, &sq.ring)?;

        Ok(IoUring {
            ring_fd: fd,
            sq,
            cq,
            flags: p.flags,
            features: p.features,
        })
    }

    pub fn submit(&mut self, submitted: u32, wait_nr: u32) -> std::io::Result<u32> {
        let cq_needs_enter = wait_nr != 0 || self.cq.needs_enter();

        let mut flags = 0;
        if self.sq.needs_enter(submitted, &mut flags) || cq_needs_enter {
            if cq_needs_enter {
                flags |= kernel::IORING_ENTER_GETEVENTS;
            }

            syscall::io_uring_enter(
                &self.ring_fd,
                submitted,
                wait_nr,
                flags,
                std::ptr::null_mut(),
            )
        } else {
            Ok(submitted)
        }
    }
}

mod tests {
    #[test]
    fn io_uring_queue_init_params() -> std::io::Result<()> {
        use super::*;

        let mut params = kernel::IoUringParams::default();
        let _ = IoUring::new(4096, &mut params)?;

        Ok(())
    }

    #[test]
    fn get_sqe_and_submit() -> std::io::Result<()> {
        use crate::*;

        let mut params = kernel::IoUringParams::default();
        let mut ring = IoUring::new(4096, &mut params)?;

        let entries = 16;
        for i in 0..entries {
            ring.sq.nop(i as u64)?;
        }
        assert_eq!(ring.sq.flush(), entries);

        let submitted = ring.submit(entries, entries)?;
        assert_eq!(entries, submitted);

        let mut index = 0;
        let consumed = ring.cq.for_each_cqe(|cqe| {
            assert_eq!(index, cqe.user_data);
            index += 1;
        });
        assert_eq!(consumed, submitted);

        Ok(())
    }
}
