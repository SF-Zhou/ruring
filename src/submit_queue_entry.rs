use crate::{constants, flags};
use std::os::raw::{c_int, c_uint, c_ulonglong, c_ushort};

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct SubmitQueueEntry {
    pub opcode: constants::OpCode,
    pub flags: flags::SQEFlags,
    pub ioprio: c_ushort,
    pub fd: c_int,
    pub off: c_ulonglong,
    pub addr: c_ulonglong,
    pub len: c_uint,
    pub op_flags: c_uint,
    pub user_data: c_ulonglong,
    pub buf_index: c_ushort,
    pub personality: c_ushort,
    pub optlen: c_uint,
    pub optval: c_ulonglong,
    pub padding: c_ulonglong,
}

mod tests {
    #[test]
    fn check_io_uring_sqe_size() {
        assert_eq!(std::mem::size_of::<super::SubmitQueueEntry>(), 64);
    }
}
