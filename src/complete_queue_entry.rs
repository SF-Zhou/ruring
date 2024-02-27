use std::os::raw::{c_int, c_uint, c_ulonglong};

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct CompleteQueueEntry {
    pub user_data: c_ulonglong,
    pub res: c_int,
    pub flags: c_uint,
}
