use crate::{
    buffer::BufferGroup,
    cq::CompleteQueue,
    entry::{Entry, OpType},
    flags::*,
    kernel, mmap,
    sq::SubmitQueue,
    syscall,
};
use std::sync::Arc;
use std::{os::fd::AsRawFd, sync::atomic::Ordering};

#[derive(Debug)]
pub struct IoUring {
    ring_fd: Arc<std::fs::File>,
    sq: SubmitQueue,
    cq: CompleteQueue,
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
            ring_fd: Arc::new(fd),
            sq,
            cq,
            flags: p.flags,
            features: p.features,
        })
    }

    pub fn submit(&mut self, submitted: u32, wait_nr: u32) -> std::io::Result<u32> {
        let cq_needs_enter = wait_nr != 0 || self.cq_needs_enter();

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

    pub fn probe(&self) -> std::io::Result<Box<crate::kernel::IoUringProbe>> {
        let mut probe: Box<crate::kernel::IoUringProbe> = Box::default();
        let ptr = &mut *probe as *mut _;
        let _ = syscall::io_uring_register(
            &self.ring_fd,
            kernel::IORING_REGISTER_PROBE,
            ptr as _,
            256,
        )?;
        Ok(probe)
    }

    pub fn for_each_cqe<F>(&mut self, f: F) -> u32
    where
        F: FnMut(&Entry),
    {
        self.cq.for_each_cqe(f)
    }

    #[inline]
    pub fn sq_flush(&mut self) -> u32 {
        self.sq.flush()
    }

    #[inline]
    pub fn nop(&mut self) -> std::io::Result<()> {
        let sqe = self.sq.get_sqe()?;
        let entry = Box::new(Entry {
            op_type: OpType::Nop,
            multishot: false,
            ..Default::default()
        });
        sqe.prepare(
            kernel::IORING_OP_NOP,
            &self.ring_fd,
            0,
            Default::default(),
            entry,
        );
        Ok(())
    }

    #[inline]
    pub fn prep_read<F: AsRawFd>(
        &mut self,
        fd: &F,
        offset: u64,
        buf: &mut [u8],
    ) -> std::io::Result<()> {
        let sqe = self.sq.get_sqe()?;
        let entry = Box::new(Entry {
            op_type: OpType::Read,
            multishot: false,
            ..Default::default()
        });
        sqe.prepare(kernel::IORING_OP_READ, fd, offset, buf, entry);
        Ok(())
    }

    #[inline]
    pub fn prep_write<F: AsRawFd>(
        &mut self,
        fd: &F,
        offset: u64,
        buf: &mut [u8],
    ) -> std::io::Result<()> {
        let sqe = self.sq.get_sqe()?;
        let entry = Box::new(Entry {
            op_type: OpType::Write,
            multishot: false,
            ..Default::default()
        });
        sqe.prepare(kernel::IORING_OP_WRITE, fd, offset, buf, entry);
        Ok(())
    }

    #[inline]
    pub fn prep_accept<F: AsRawFd>(&mut self, fd: &F) -> std::io::Result<()> {
        let sqe = self.sq.get_sqe()?;
        let entry = Box::new(Entry {
            op_type: OpType::Accept,
            multishot: true,
            ..Default::default()
        });
        sqe.prepare(kernel::IORING_OP_ACCEPT, fd, 0, Default::default(), entry);
        sqe.addr = 0;
        sqe.ioprio |= kernel::IORING_ACCEPT_MULTISHOT;
        Ok(())
    }

    #[inline]
    pub fn prep_recv<F: AsRawFd>(&mut self, fd: &F) -> std::io::Result<()> {
        let sqe = self.sq.get_sqe()?;
        let entry = Box::new(Entry {
            op_type: OpType::Receive,
            multishot: true,
            ..Default::default()
        });
        sqe.prepare(kernel::IORING_OP_RECV, fd, 0, Default::default(), entry);
        sqe.addr = 0;
        sqe.ioprio |= kernel::IORING_RECV_MULTISHOT;
        sqe.flags |= SQEFlags::BUFFER_SELECT;
        sqe.buf_index = 0;
        Ok(())
    }

    #[inline]
    fn cq_needs_flush(&self) -> bool {
        self.sq
            .kflags
            .load(Ordering::Relaxed)
            .intersects(SQFlags::CQ_OVERFLOW | SQFlags::TASKRUN)
    }

    #[inline]
    fn cq_needs_enter(&self) -> bool {
        self.flags.contains(SetupFlags::IOPOLL) || self.cq_needs_flush()
    }

    pub fn setup_buffer_ring(&mut self, entries: u32, bgid: u16) -> std::io::Result<BufferGroup> {
        let mut buffer = mmap::Buffer::<kernel::IoUringBuf>::anonymous(entries as usize)?;

        let mut reg = kernel::IoUringBufReg {
            ring_addr: buffer.as_raw_addr(),
            ring_entries: entries,
            bgid,
            ..Default::default()
        };

        syscall::io_uring_register(
            &self.ring_fd,
            kernel::IORING_REGISTER_PBUF_RING,
            &mut reg as *mut _ as _,
            1,
        )?;

        Ok(BufferGroup::new(self.ring_fd.clone(), buffer, bgid))
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
        for _ in 0..entries {
            ring.nop()?;
        }
        assert_eq!(ring.sq_flush(), entries);

        let submitted = ring.submit(entries, entries)?;
        assert_eq!(entries, submitted);

        let mut index = 0;
        let consumed = ring.for_each_cqe(|_| {
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

        ring.prep_read(&file, 0, buf.as_mut_slice())?;
        assert_eq!(ring.sq_flush(), 1);

        let submitted = ring.submit(1, 1)?;
        assert_eq!(submitted, 1);

        let consumed = ring.for_each_cqe(|entry| {
            assert_eq!(entry.res, LEN as i32);
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

        ring.prep_write(&writer, 0, writer_buf.as_mut_slice())?;
        ring.prep_read(&reader, 0, reader_buf.as_mut_slice())?;
        assert_eq!(ring.sq_flush(), 2);

        let submitted = ring.submit(2, 2)?;
        assert_eq!(submitted, 2);

        let consumed = ring.for_each_cqe(|entry| {
            assert_eq!(entry.res, LEN as i32);
            if entry.op_type == entry::OpType::Read {
                assert_eq!(writer_buf, reader_buf);
            }
        });
        assert_eq!(consumed, 2);

        Ok(())
    }

    #[test]
    fn probe() -> std::io::Result<()> {
        use crate::*;
        use std::process::Command;

        fn get_linux_kernel_version() -> Option<String> {
            let output = Command::new("uname").arg("-r").output().ok()?;
            let version_str = String::from_utf8(output.stdout).ok()?;
            Some(version_str.trim().to_string())
        }

        let result = get_linux_kernel_version();
        if let Some(kernel_version) = result {
            if kernel_version.as_str() < "5.6" {
                println!("Test skipped on kernel versions before 5.6");
                return Ok(());
            }
        }

        let mut params = kernel::IoUringParams::default();
        let ring = IoUring::new(4096, &mut params)?;
        let probe = ring.probe()?;

        let supported_op_cnt = (1..kernel::IORING_OP_LAST)
            .map(|op| {
                probe.ops[op as usize]
                    .flags
                    .contains(flags::ProbeOpFlags::SUPPORTED)
            })
            .fold(0u32, |a, b| a + b as u32);
        println!("supported op cnt: {supported_op_cnt}");
        assert_ne!(supported_op_cnt, 0);

        Ok(())
    }

    #[test]
    fn accept() -> std::io::Result<()> {
        use crate::*;
        use std::net::{TcpListener, TcpStream};

        let listener = TcpListener::bind("127.0.0.1:0")?;

        let mut params = kernel::IoUringParams::default();
        let mut ring = IoUring::new(4096, &mut params)?;
        ring.prep_accept(&listener)?;
        ring.sq_flush();
        let submitted = ring.submit(1, 0)?;
        assert_eq!(submitted, 1);

        for _ in 0..3 {
            let _ = TcpStream::connect(listener.local_addr()?)?;
            let _ = TcpStream::connect(listener.local_addr()?)?;

            ring.submit(0, 2)?;

            let mut entry_addr = 0u64;
            let consumed = ring.for_each_cqe(|entry| {
                assert!(entry.res >= 0);
                assert!(entry.op_type == entry::OpType::Accept);
                if entry_addr == 0 {
                    entry_addr = entry as *const entry::Entry as u64;
                } else {
                    assert_eq!(entry_addr, entry as *const entry::Entry as u64);
                }
            });
            assert_eq!(consumed, 2);
            assert_ne!(entry_addr, 0);
        }

        Ok(())
    }

    #[test]
    fn buffer_group() -> std::io::Result<()> {
        use crate::*;

        let mut params = kernel::IoUringParams::default();
        let mut ring = IoUring::new(64, &mut params)?;

        let _ = ring.setup_buffer_ring(4, 1)?;

        Ok(())
    }
}
