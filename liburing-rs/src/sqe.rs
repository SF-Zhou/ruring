use std::os::fd::AsRawFd;

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct IoUringSqe {
    pub opcode: ::std::os::raw::c_uchar,
    pub flags: ::std::os::raw::c_uchar,
    pub ioprio: ::std::os::raw::c_ushort,
    pub fd: ::std::os::raw::c_int,
    pub off: ::std::os::raw::c_ulonglong,
    pub addr: ::std::os::raw::c_ulonglong,
    pub len: ::std::os::raw::c_uint,
    pub op_flags: ::std::os::raw::c_uint,
    pub user_data: ::std::os::raw::c_ulonglong,
    pub buf_index: ::std::os::raw::c_ushort,
    pub personality: ::std::os::raw::c_ushort,
    pub optlen: ::std::os::raw::c_uint,
    pub optval: ::std::os::raw::c_ulonglong,
    pub padding: ::std::os::raw::c_ulonglong,
}

impl IoUringSqe {
    #[inline]
    pub fn prepare(
        &mut self,
        opcode: u8,
        fd: &impl AsRawFd,
        offset: u64,
        buf: &mut [u8],
        entry: Box<crate::entry::Entry>,
    ) {
        *self = Self {
            opcode,
            fd: fd.as_raw_fd(),
            off: offset,
            addr: buf.as_mut_ptr() as u64,
            len: buf.len() as u32,
            user_data: Box::into_raw(entry) as _,
            ..Default::default()
        }
    }
}

mod tests {
    #[test]
    fn check_io_uring_sqe_size() {
        assert_eq!(std::mem::size_of::<super::IoUringSqe>(), 64);
    }
}
