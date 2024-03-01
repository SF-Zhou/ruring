use crate::{constants, syscall, Config, MmapBuffer};
use std::os::{
    fd::AsRawFd,
    raw::{c_uint, c_ulonglong, c_ushort},
};
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::Arc;

#[repr(C)]
#[derive(Default, Debug)]
pub struct IoUringBuf {
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
    pub index_mark: u16,
    pub buffer_size: usize,
    pub current_tail: AtomicU16,
    pub kernel_tail: &'static mut AtomicU16,
    pub data_buffer: Vec<u8>,
    pub ring_buffer: MmapBuffer,
    pub ring_array: &'static mut [IoUringBuf],
}

impl RecvBuffer {
    pub fn new(ring_fd: &impl AsRawFd, config: &Config) -> std::io::Result<RecvBuffer> {
        // 1. prepare ring buffer.
        let bgid = 0;
        let ring_buffer_sz = config.recv_buffer_count * std::mem::size_of::<IoUringBuf>();
        let ring_buffer = MmapBuffer::anonymous(ring_buffer_sz)?;

        let mut reg = IoUringBufReg {
            ring_addr: ring_buffer.as_ptr() as _,
            ring_entries: config.recv_buffer_count as _,
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
        let data_buffer_sz = config.recv_buffer_size * config.recv_buffer_count;
        let layout = std::alloc::Layout::from_size_align(data_buffer_sz, 4096).unwrap();
        let mut data_buffer = unsafe {
            Vec::<u8>::from_raw_parts(std::alloc::alloc(layout), data_buffer_sz, data_buffer_sz)
        };

        // 4. register data buffer.
        let mut iovec = libc::iovec {
            iov_base: data_buffer.as_mut_ptr() as _,
            iov_len: data_buffer.len(),
        };

        syscall::io_uring_register(
            ring_fd,
            constants::RegisterCode::REGISTER_BUFFERS.bits(),
            &mut iovec as *mut _ as _,
            1,
        )?;

        // 5. fill ring buffer.
        let ring_array: &'static mut [IoUringBuf] =
            ring_buffer.offset_as_mut_slice(0, config.recv_buffer_count as _);
        for (index, buf) in ring_array.iter_mut().enumerate() {
            buf.addr = (&mut data_buffer[config.recv_buffer_size * index]) as *mut _ as _;
            buf.len = config.recv_buffer_size as _;
            buf.bid = 1u16 + index as u16;
        }

        let kernel_tail = ring_buffer.offset_as_mut::<AtomicU16>(24);
        kernel_tail.store(config.recv_buffer_size as u16, Ordering::Release);

        Ok(RecvBuffer {
            index_mark: (config.recv_buffer_count.next_power_of_two() - 1) as _,
            buffer_size: config.recv_buffer_size,
            current_tail: AtomicU16::new(config.recv_buffer_size as _),
            kernel_tail,
            data_buffer,
            ring_buffer,
            ring_array,
        })
    }

    fn addr(&self, bid: u16) -> *const u8 {
        self.data_buffer
            .as_ptr()
            .wrapping_byte_add((bid - 1) as usize * self.buffer_size)
    }

    fn recycle(&self, bid: u16) {
        let tail = self.current_tail.fetch_add(1, Ordering::AcqRel);

        let offset = (tail & self.index_mark) as usize * std::mem::size_of::<IoUringBuf>();
        let buf: &mut IoUringBuf = self.ring_buffer.offset_as_mut(offset);
        buf.addr = self.addr(bid) as _;
        buf.len = self.buffer_size as _;
        buf.bid = bid;

        let backoff = crate::backoff::Backoff::new();
        while self.kernel_tail.load(Ordering::Acquire) != tail {
            backoff.spin_light();
        }
        self.kernel_tail
            .store(tail.wrapping_add(1), Ordering::Release);
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
