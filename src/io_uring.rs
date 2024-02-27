use crate::{flags, syscall, CompleteQueue, Config, IoUringParams, RecvBuffer, SubmitQueue};
use std::os::fd::OwnedFd;
use std::sync::atomic::Ordering;
use std::sync::Arc;

pub struct IoUring {
    ring_fd: OwnedFd,
    sq: SubmitQueue,
    cq: CompleteQueue,
    recv_buffer: Arc<RecvBuffer>,
    pub flags: flags::SetupFlags,
    pub features: flags::FeatureFlags,
}

impl IoUring {
    pub fn new(config: &Config) -> std::io::Result<IoUring> {
        // 1. init io_uring fd
        let mut p = IoUringParams::default();
        let ring_fd = syscall::io_uring_setup(config.entries, &mut p)?;
        let sq = SubmitQueue::new(&ring_fd, &p)?;
        let cq = CompleteQueue::new(&ring_fd, &p, &sq.ring_buffer)?;

        // 2. init recv buffer
        let recv_buffer = Arc::new(RecvBuffer::new(&ring_fd, config)?);

        Ok(IoUring {
            ring_fd,
            sq,
            cq,
            recv_buffer,
            flags: p.flags,
            features: p.features,
        })
    }

    #[inline]
    fn cq_needs_flush(&self) -> bool {
        self.sq
            .kflags
            .load(Ordering::Relaxed)
            .intersects(flags::SQFlags::CQ_OVERFLOW | flags::SQFlags::TASKRUN)
    }

    #[inline]
    fn cq_needs_enter(&self) -> bool {
        self.flags.contains(flags::SetupFlags::IOPOLL) || self.cq_needs_flush()
    }

    pub fn submit(&mut self, submitted: u32, wait_nr: u32) -> std::io::Result<u32> {
        let cq_needs_enter = wait_nr != 0 || self.cq_needs_enter();

        let mut flags = flags::EnterFlags::default();
        if self.sq.needs_enter(submitted, &mut flags) || cq_needs_enter {
            if cq_needs_enter {
                flags.insert(flags::EnterFlags::GETEVENTS);
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
    fn io_uring_setup() -> std::io::Result<()> {
        use super::*;

        let config = Config::new(1024);
        let mut io_uring = IoUring::new(&config)?;

        io_uring.submit(0, 0)?;

        Ok(())
    }
}
