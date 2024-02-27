use crate::flags;
use ::std::os::raw::{c_uint, c_ulonglong};

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct IoSqringOffsets {
    pub head: c_uint,
    pub tail: c_uint,
    pub ring_mask: c_uint,
    pub ring_entries: c_uint,
    pub flags: c_uint,
    pub dropped: c_uint,
    pub array: c_uint,
    pub resv1: c_uint,
    pub user_addr: c_ulonglong,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct IoCqringOffsets {
    pub head: c_uint,
    pub tail: c_uint,
    pub ring_mask: c_uint,
    pub ring_entries: c_uint,
    pub overflow: c_uint,
    pub cqes: c_uint,
    pub flags: c_uint,
    pub resv1: c_uint,
    pub user_addr: c_ulonglong,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct IoUringParams {
    pub sq_entries: c_uint,
    pub cq_entries: c_uint,
    pub flags: flags::SetupFlags,
    pub sq_thread_cpu: c_uint,
    pub sq_thread_idle: c_uint,
    pub features: flags::FeatureFlags,
    pub wq_fd: c_uint,
    pub resv: [c_uint; 3usize],
    pub sq_off: IoSqringOffsets,
    pub cq_off: IoCqringOffsets,
}
