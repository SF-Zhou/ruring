use crate::{cq::CompleteQueue, flags::*, kernel, sq::SubmitQueue, syscall};

#[derive(Debug)]
pub struct IoUring {
    pub ring_fd: std::fs::File,
    pub sq: SubmitQueue,
    pub cq: CompleteQueue,
    pub flags: SetupFlags,
    pub features: FeatureFlags,
}

impl IoUring {
    pub fn new(entries: u32, p: &mut kernel::IoUringParams) -> std::io::Result<IoUring> {
        if p.flags.contains(SetupFlags::REGISTERED_FD_ONLY)
            && !p.flags.contains(SetupFlags::NO_MMAP)
        {
            return Err(std::io::ErrorKind::InvalidInput.into());
        }

        if p.flags.contains(SetupFlags::NO_MMAP) {
            unimplemented!("IORING_SETUP_NO_MMAP");
        }
        if p.flags.contains(SetupFlags::REGISTERED_FD_ONLY) {
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

        let mut flags = EnterFlags::default();
        if self.sq.needs_enter(submitted, &mut flags) || cq_needs_enter {
            if cq_needs_enter {
                flags.insert(EnterFlags::GETEVENTS);
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
        let ring = IoUring::new(4096, &mut params)?;
        println!("ring is {ring:#?}");

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

    #[test]
    fn do_read() -> std::io::Result<()> {
        use crate::*;

        let mut params = kernel::IoUringParams::default();
        let mut ring = IoUring::new(4096, &mut params)?;

        let file = std::fs::File::open("/dev/random")?;
        const LEN: usize = 64;
        let mut buf = vec![0u8; LEN];

        ring.sq.prep_read(&file, 0, buf.as_mut_slice(), 0xbeef)?;
        assert_eq!(ring.sq.flush(), 1);

        let submitted = ring.submit(1, 1)?;
        assert_eq!(submitted, 1);

        let consumed = ring.cq.for_each_cqe(|cqe| {
            assert_eq!(cqe.res, LEN as i32);
            assert_eq!(cqe.user_data, 0xbeef);
            assert_ne!(buf, vec![0u8; 64]);
        });
        assert_eq!(consumed, 1);

        Ok(())
    }

    #[test]
    fn do_read_and_write() -> std::io::Result<()> {
        use crate::*;
        use std::fs::File;
        use std::os::fd::FromRawFd;

        let mut params = kernel::IoUringParams::default();
        let mut ring = IoUring::new(4096, &mut params)?;

        let mut fds = [0i32, 0i32];
        let ret = unsafe { libc::pipe(fds.as_mut_ptr()) };
        assert_eq!(ret, 0);

        let reader = unsafe { File::from_raw_fd(fds[0]) };
        let writer = unsafe { File::from_raw_fd(fds[1]) };

        const LEN: usize = 64;
        let mut reader_buf = vec![0u8; LEN];
        let mut writer_buf = vec![1u8; LEN];

        ring.sq
            .prep_write(&writer, 0, writer_buf.as_mut_slice(), 1)?;
        ring.sq
            .prep_read(&reader, 0, reader_buf.as_mut_slice(), 2)?;
        assert_eq!(ring.sq.flush(), 2);

        let submitted = ring.submit(2, 2)?;
        assert_eq!(submitted, 2);

        let consumed = ring.cq.for_each_cqe(|cqe| {
            assert_eq!(cqe.res, LEN as i32);
            if cqe.user_data == 2 {
                assert_eq!(writer_buf, reader_buf);
            }
        });
        assert_eq!(consumed, 2);

        Ok(())
    }
}
