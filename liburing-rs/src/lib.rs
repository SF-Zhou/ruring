pub mod io_uring;
mod setup;
pub mod syscall;

pub use io_uring::IoUringParams;
pub use setup::*;
use std::os::raw::{c_uint, c_void};

#[derive(Debug, Copy, Clone)]
pub struct IoUringSq {
    pub khead: *mut c_uint,
    pub ktail: *mut c_uint,
    pub kflags: *mut c_uint,
    pub kdropped: *mut c_uint,
    pub array: *mut c_uint,
    pub sqes: *mut io_uring::IoUringSqe,
    pub sqe_head: c_uint,
    pub sqe_tail: c_uint,
    pub ring_sz: usize,
    pub ring_ptr: *mut c_void,
    pub ring_mask: c_uint,
    pub ring_entries: c_uint,
}

impl Default for IoUringSq {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct IoUringCq {
    pub khead: *mut c_uint,
    pub ktail: *mut c_uint,
    pub kflags: *mut c_uint,
    pub koverflow: *mut c_uint,
    pub cqes: *mut io_uring::IoUringCqe,
    pub ring_sz: usize,
    pub ring_ptr: *mut c_void,
    pub ring_mask: c_uint,
    pub ring_entries: c_uint,
}

impl Default for IoUringCq {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[derive(Debug)]
pub struct IoUring {
    pub ring_fd: std::fs::File,
    pub sq: IoUringSq,
    pub cq: IoUringCq,
    pub flags: c_uint,
    pub features: c_uint,
}
