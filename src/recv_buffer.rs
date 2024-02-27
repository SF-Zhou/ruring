use crate::{constants, syscall, Config, MmapBuffer};
use std::os::{
    fd::AsRawFd,
    raw::{c_uint, c_ulonglong, c_ushort},
};
use std::sync::atomic::AtomicU16;
use std::sync::Arc;

#[repr(C)]
#[derive(Default, Debug)]
struct IoUringBuf {
    pub addr: c_ulonglong,
    pub len: c_uint,
    pub bid: c_ushort,
    pub tail: AtomicU16,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
struct IoUringBufReg {
    pub ring_addr: c_ulonglong,
    pub ring_entries: c_uint,
    pub bgid: c_ushort,
    pub flags: c_ushort,
    pub resv: [c_ulonglong; 3usize],
}

pub struct RecvBuffer {
    pub ring_buffer: MmapBuffer,
    pub data_buffer: Vec<u8>,
}

impl RecvBuffer {
    pub fn new(ring_fd: &impl AsRawFd, config: &Config) -> std::io::Result<RecvBuffer> {
        // 1. prepare ring buffer.
        let bgid = 0;
        let ring_buffer_sz = config.recv_buffer_count as usize * std::mem::size_of::<IoUringBuf>();
        let ring_buffer = MmapBuffer::anonymous(ring_buffer_sz)?;

        let mut reg = IoUringBufReg {
            ring_addr: ring_buffer.as_ptr() as _,
            ring_entries: config.recv_buffer_count,
            bgid,
            ..Default::default()
        };

        // 2. register ring buffer.
        syscall::io_uring_register(
            ring_fd,
            constants::RegisterCode::REGISTER_PBUF_RING.bits(),
            &mut reg as *mut _ as _,
            1,
        )?;

        // 3. prepare data buffer.
        let sz = config.recv_buffer_size as usize * config.recv_buffer_count as usize;
        let layout = std::alloc::Layout::from_size_align(sz, 4096).unwrap();
        let mut data_buffer =
            unsafe { Vec::<u8>::from_raw_parts(std::alloc::alloc(layout), sz, sz) };

        let mut iovec = libc::iovec {
            iov_base: data_buffer.as_mut_ptr() as _,
            iov_len: data_buffer.len(),
        };

        // 4. register data buffer.
        syscall::io_uring_register(
            ring_fd,
            constants::RegisterCode::REGISTER_BUFFERS.bits(),
            &mut iovec as *mut _ as _,
            1,
        )?;

        Ok(RecvBuffer {
            ring_buffer,
            data_buffer,
        })
    }

    pub fn recycle(&self, _bid: u16) {
        // let tail = self.bufs[0].tail.load(Ordering::Relaxed);
        // let addr = self.addr(bid);
        // let buf = &mut self.bufs[(tail as u32 & (self.entries - 1)) as usize];
        // buf.addr = addr;
        // buf.len = self.size as _;
        // buf.bid = bid;
        // self.bufs[0].tail.fetch_add(1, Ordering::Relaxed);
    }
}

pub struct RecvBufferGuard {
    buffer: &'static [u8],
    recv_buffer: Arc<RecvBuffer>,
    bid: u16,
}

impl std::ops::Deref for RecvBufferGuard {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.buffer
    }
}

impl Drop for RecvBufferGuard {
    fn drop(&mut self) {
        self.recv_buffer.recycle(self.bid);
    }
}
