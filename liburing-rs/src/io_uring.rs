/* automatically generated by rust-bindgen 0.69.1, modified by SF-Zhou */
/* C header file: https://github.com/axboe/liburing/blob/liburing-2.5/src/include/liburing/io_uring.h */
/* bindgen io_uring.h --no-layout-tests --no-prepend-enum-name --translate-enum-integer-types */

pub const IORING_FILE_INDEX_ALLOC: i32 = -1;
pub const IORING_SETUP_IOPOLL: u32 = 1;
pub const IORING_SETUP_SQPOLL: u32 = 2;
pub const IORING_SETUP_SQ_AFF: u32 = 4;
pub const IORING_SETUP_CQSIZE: u32 = 8;
pub const IORING_SETUP_CLAMP: u32 = 16;
pub const IORING_SETUP_ATTACH_WQ: u32 = 32;
pub const IORING_SETUP_R_DISABLED: u32 = 64;
pub const IORING_SETUP_SUBMIT_ALL: u32 = 128;
pub const IORING_SETUP_COOP_TASKRUN: u32 = 256;
pub const IORING_SETUP_TASKRUN_FLAG: u32 = 512;
pub const IORING_SETUP_SQE128: u32 = 1024;
pub const IORING_SETUP_CQE32: u32 = 2048;
pub const IORING_SETUP_SINGLE_ISSUER: u32 = 4096;
pub const IORING_SETUP_DEFER_TASKRUN: u32 = 8192;
pub const IORING_SETUP_NO_MMAP: u32 = 16384;
pub const IORING_SETUP_REGISTERED_FD_ONLY: u32 = 32768;
pub const IORING_SETUP_NO_SQARRAY: u32 = 65536;
pub const IORING_URING_CMD_FIXED: u32 = 1;
pub const IORING_FSYNC_DATASYNC: u32 = 1;
pub const IORING_TIMEOUT_ABS: u32 = 1;
pub const IORING_TIMEOUT_UPDATE: u32 = 2;
pub const IORING_TIMEOUT_BOOTTIME: u32 = 4;
pub const IORING_TIMEOUT_REALTIME: u32 = 8;
pub const IORING_LINK_TIMEOUT_UPDATE: u32 = 16;
pub const IORING_TIMEOUT_ETIME_SUCCESS: u32 = 32;
pub const IORING_TIMEOUT_MULTISHOT: u32 = 64;
pub const IORING_TIMEOUT_CLOCK_MASK: u32 = 12;
pub const IORING_TIMEOUT_UPDATE_MASK: u32 = 18;
pub const SPLICE_F_FD_IN_FIXED: u32 = 2147483648;
pub const IORING_POLL_ADD_MULTI: u32 = 1;
pub const IORING_POLL_UPDATE_EVENTS: u32 = 2;
pub const IORING_POLL_UPDATE_USER_DATA: u32 = 4;
pub const IORING_POLL_ADD_LEVEL: u32 = 8;
pub const IORING_ASYNC_CANCEL_ALL: u32 = 1;
pub const IORING_ASYNC_CANCEL_FD: u32 = 2;
pub const IORING_ASYNC_CANCEL_ANY: u32 = 4;
pub const IORING_ASYNC_CANCEL_FD_FIXED: u32 = 8;
pub const IORING_RECVSEND_POLL_FIRST: u32 = 1;
pub const IORING_RECV_MULTISHOT: u32 = 2;
pub const IORING_RECVSEND_FIXED_BUF: u32 = 4;
pub const IORING_SEND_ZC_REPORT_USAGE: u32 = 8;
pub const IORING_NOTIF_USAGE_ZC_COPIED: u32 = 2147483648;
pub const IORING_ACCEPT_MULTISHOT: u32 = 1;
pub const IORING_MSG_RING_CQE_SKIP: u32 = 1;
pub const IORING_MSG_RING_FLAGS_PASS: u32 = 2;
pub const IORING_FIXED_FD_NO_CLOEXEC: u32 = 1;
pub const IORING_CQE_F_BUFFER: u32 = 1;
pub const IORING_CQE_F_MORE: u32 = 2;
pub const IORING_CQE_F_SOCK_NONEMPTY: u32 = 4;
pub const IORING_CQE_F_NOTIF: u32 = 8;
pub const IORING_OFF_SQ_RING: i64 = 0;
pub const IORING_OFF_CQ_RING: i64 = 134217728;
pub const IORING_OFF_SQES: i64 = 268435456;
pub const IORING_OFF_PBUF_RING: i64 = 2147483648;
pub const IORING_OFF_PBUF_SHIFT: i64 = 16;
pub const IORING_OFF_MMAP_MASK: i64 = 4160749568;
pub const IORING_SQ_NEED_WAKEUP: u32 = 1;
pub const IORING_SQ_CQ_OVERFLOW: u32 = 2;
pub const IORING_SQ_TASKRUN: u32 = 4;
pub const IORING_CQ_EVENTFD_DISABLED: u32 = 1;
pub const IORING_ENTER_GETEVENTS: u32 = 1;
pub const IORING_ENTER_SQ_WAKEUP: u32 = 2;
pub const IORING_ENTER_SQ_WAIT: u32 = 4;
pub const IORING_ENTER_EXT_ARG: u32 = 8;
pub const IORING_ENTER_REGISTERED_RING: u32 = 16;
pub const IORING_FEAT_SINGLE_MMAP: u32 = 1;
pub const IORING_FEAT_NODROP: u32 = 2;
pub const IORING_FEAT_SUBMIT_STABLE: u32 = 4;
pub const IORING_FEAT_RW_CUR_POS: u32 = 8;
pub const IORING_FEAT_CUR_PERSONALITY: u32 = 16;
pub const IORING_FEAT_FAST_POLL: u32 = 32;
pub const IORING_FEAT_POLL_32BITS: u32 = 64;
pub const IORING_FEAT_SQPOLL_NONFIXED: u32 = 128;
pub const IORING_FEAT_EXT_ARG: u32 = 256;
pub const IORING_FEAT_NATIVE_WORKERS: u32 = 512;
pub const IORING_FEAT_RSRC_TAGS: u32 = 1024;
pub const IORING_FEAT_CQE_SKIP: u32 = 2048;
pub const IORING_FEAT_LINKED_FILE: u32 = 4096;
pub const IORING_FEAT_REG_REG_RING: u32 = 8192;
pub const IORING_RSRC_REGISTER_SPARSE: u32 = 1;
pub const IORING_REGISTER_FILES_SKIP: i32 = -2;
pub const IO_URING_OP_SUPPORTED: u32 = 1;
pub const IOSQE_FIXED_FILE_BIT: u32 = 0;
pub const IOSQE_IO_DRAIN_BIT: u32 = 1;
pub const IOSQE_IO_LINK_BIT: u32 = 2;
pub const IOSQE_IO_HARDLINK_BIT: u32 = 3;
pub const IOSQE_ASYNC_BIT: u32 = 4;
pub const IOSQE_BUFFER_SELECT_BIT: u32 = 5;
pub const IOSQE_CQE_SKIP_SUCCESS_BIT: u32 = 6;
pub const IORING_OP_NOP: u32 = 0;
pub const IORING_OP_READV: u32 = 1;
pub const IORING_OP_WRITEV: u32 = 2;
pub const IORING_OP_FSYNC: u32 = 3;
pub const IORING_OP_READ_FIXED: u32 = 4;
pub const IORING_OP_WRITE_FIXED: u32 = 5;
pub const IORING_OP_POLL_ADD: u32 = 6;
pub const IORING_OP_POLL_REMOVE: u32 = 7;
pub const IORING_OP_SYNC_FILE_RANGE: u32 = 8;
pub const IORING_OP_SENDMSG: u32 = 9;
pub const IORING_OP_RECVMSG: u32 = 10;
pub const IORING_OP_TIMEOUT: u32 = 11;
pub const IORING_OP_TIMEOUT_REMOVE: u32 = 12;
pub const IORING_OP_ACCEPT: u32 = 13;
pub const IORING_OP_ASYNC_CANCEL: u32 = 14;
pub const IORING_OP_LINK_TIMEOUT: u32 = 15;
pub const IORING_OP_CONNECT: u32 = 16;
pub const IORING_OP_FALLOCATE: u32 = 17;
pub const IORING_OP_OPENAT: u32 = 18;
pub const IORING_OP_CLOSE: u32 = 19;
pub const IORING_OP_FILES_UPDATE: u32 = 20;
pub const IORING_OP_STATX: u32 = 21;
pub const IORING_OP_READ: u32 = 22;
pub const IORING_OP_WRITE: u32 = 23;
pub const IORING_OP_FADVISE: u32 = 24;
pub const IORING_OP_MADVISE: u32 = 25;
pub const IORING_OP_SEND: u32 = 26;
pub const IORING_OP_RECV: u32 = 27;
pub const IORING_OP_OPENAT2: u32 = 28;
pub const IORING_OP_EPOLL_CTL: u32 = 29;
pub const IORING_OP_SPLICE: u32 = 30;
pub const IORING_OP_PROVIDE_BUFFERS: u32 = 31;
pub const IORING_OP_REMOVE_BUFFERS: u32 = 32;
pub const IORING_OP_TEE: u32 = 33;
pub const IORING_OP_SHUTDOWN: u32 = 34;
pub const IORING_OP_RENAMEAT: u32 = 35;
pub const IORING_OP_UNLINKAT: u32 = 36;
pub const IORING_OP_MKDIRAT: u32 = 37;
pub const IORING_OP_SYMLINKAT: u32 = 38;
pub const IORING_OP_LINKAT: u32 = 39;
pub const IORING_OP_MSG_RING: u32 = 40;
pub const IORING_OP_FSETXATTR: u32 = 41;
pub const IORING_OP_SETXATTR: u32 = 42;
pub const IORING_OP_FGETXATTR: u32 = 43;
pub const IORING_OP_GETXATTR: u32 = 44;
pub const IORING_OP_SOCKET: u32 = 45;
pub const IORING_OP_URING_CMD: u32 = 46;
pub const IORING_OP_SEND_ZC: u32 = 47;
pub const IORING_OP_SENDMSG_ZC: u32 = 48;
pub const IORING_OP_READ_MULTISHOT: u32 = 49;
pub const IORING_OP_WAITID: u32 = 50;
pub const IORING_OP_FUTEX_WAIT: u32 = 51;
pub const IORING_OP_FUTEX_WAKE: u32 = 52;
pub const IORING_OP_FUTEX_WAITV: u32 = 53;
pub const IORING_OP_FIXED_FD_INSTALL: u32 = 54;
pub const IORING_OP_LAST: u32 = 55;
pub const IORING_MSG_DATA: u32 = 0;
pub const IORING_MSG_SEND_FD: u32 = 1;
pub const IORING_CQE_BUFFER_SHIFT: u32 = 16;
pub const IORING_REGISTER_BUFFERS: u32 = 0;
pub const IORING_UNREGISTER_BUFFERS: u32 = 1;
pub const IORING_REGISTER_FILES: u32 = 2;
pub const IORING_UNREGISTER_FILES: u32 = 3;
pub const IORING_REGISTER_EVENTFD: u32 = 4;
pub const IORING_UNREGISTER_EVENTFD: u32 = 5;
pub const IORING_REGISTER_FILES_UPDATE: u32 = 6;
pub const IORING_REGISTER_EVENTFD_ASYNC: u32 = 7;
pub const IORING_REGISTER_PROBE: u32 = 8;
pub const IORING_REGISTER_PERSONALITY: u32 = 9;
pub const IORING_UNREGISTER_PERSONALITY: u32 = 10;
pub const IORING_REGISTER_RESTRICTIONS: u32 = 11;
pub const IORING_REGISTER_ENABLE_RINGS: u32 = 12;
pub const IORING_REGISTER_FILES2: u32 = 13;
pub const IORING_REGISTER_FILES_UPDATE2: u32 = 14;
pub const IORING_REGISTER_BUFFERS2: u32 = 15;
pub const IORING_REGISTER_BUFFERS_UPDATE: u32 = 16;
pub const IORING_REGISTER_IOWQ_AFF: u32 = 17;
pub const IORING_UNREGISTER_IOWQ_AFF: u32 = 18;
pub const IORING_REGISTER_IOWQ_MAX_WORKERS: u32 = 19;
pub const IORING_REGISTER_RING_FDS: u32 = 20;
pub const IORING_UNREGISTER_RING_FDS: u32 = 21;
pub const IORING_REGISTER_PBUF_RING: u32 = 22;
pub const IORING_UNREGISTER_PBUF_RING: u32 = 23;
pub const IORING_REGISTER_SYNC_CANCEL: u32 = 24;
pub const IORING_REGISTER_FILE_ALLOC_RANGE: u32 = 25;
pub const IORING_REGISTER_LAST: u32 = 26;
pub const IORING_REGISTER_USE_REGISTERED_RING: u32 = 2147483648;
pub const IO_WQ_BOUND: u32 = 0;
pub const IO_WQ_UNBOUND: u32 = 1;
pub const IOU_PBUF_RING_MMAP: u32 = 1;
pub const IORING_RESTRICTION_REGISTER_OP: u32 = 0;
pub const IORING_RESTRICTION_SQE_OP: u32 = 1;
pub const IORING_RESTRICTION_SQE_FLAGS_ALLOWED: u32 = 2;
pub const IORING_RESTRICTION_SQE_FLAGS_REQUIRED: u32 = 3;
pub const IORING_RESTRICTION_LAST: u32 = 4;
pub const SOCKET_URING_OP_SIOCINQ: u32 = 0;
pub const SOCKET_URING_OP_SIOCOUTQ: u32 = 1;
pub const SOCKET_URING_OP_GETSOCKOPT: u32 = 2;
pub const SOCKET_URING_OP_SETSOCKOPT: u32 = 3;

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct KernelTimespec {
    pub tv_sec: ::std::os::raw::c_longlong,
    pub tv_nsec: ::std::os::raw::c_longlong,
}

#[repr(C)]
pub struct IoUringSqe {
    pub opcode: ::std::os::raw::c_uchar,
    pub flags: ::std::os::raw::c_uchar,
    pub ioprio: ::std::os::raw::c_ushort,
    pub fd: ::std::os::raw::c_int,
    pub union1: IoUringSqeUnion1,
    pub union2: IoUringSqeUnion2,
    pub len: ::std::os::raw::c_uint,
    pub union3: IoUringSqeUnion3,
    pub user_data: ::std::os::raw::c_ulonglong,
    pub union4: IoUringSqeUnion4,
    pub personality: ::std::os::raw::c_ushort,
    pub union5: IoUringSqeUnion5,
    pub union6: IoUringSqeUnion6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union IoUringSqeUnion1 {
    pub off: ::std::os::raw::c_ulonglong,
    pub addr2: ::std::os::raw::c_ulonglong,
    pub cmd_op: IoUringSqeUnion1CmdOp,
}

impl Default for IoUringSqeUnion1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl ::std::fmt::Debug for IoUringSqeUnion1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("IoUringSqeUnion1")
    }
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct IoUringSqeUnion1CmdOp {
    pub cmd_op: ::std::os::raw::c_uint,
    pub __pad1: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union IoUringSqeUnion2 {
    pub addr: ::std::os::raw::c_ulonglong,
    pub splice_off_in: ::std::os::raw::c_ulonglong,
    pub level_and_optname: IoUringSqeUnion2LevelAndOptName,
}

impl Default for IoUringSqeUnion2 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl ::std::fmt::Debug for IoUringSqeUnion2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("IoUringSqeUnion2")
    }
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct IoUringSqeUnion2LevelAndOptName {
    pub level: ::std::os::raw::c_uint,
    pub optname: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union IoUringSqeUnion3 {
    pub rw_flags: ::std::os::raw::c_int,
    pub fsync_flags: ::std::os::raw::c_uint,
    pub poll_events: ::std::os::raw::c_ushort,
    pub poll32_events: ::std::os::raw::c_uint,
    pub sync_range_flags: ::std::os::raw::c_uint,
    pub msg_flags: ::std::os::raw::c_uint,
    pub timeout_flags: ::std::os::raw::c_uint,
    pub accept_flags: ::std::os::raw::c_uint,
    pub cancel_flags: ::std::os::raw::c_uint,
    pub open_flags: ::std::os::raw::c_uint,
    pub statx_flags: ::std::os::raw::c_uint,
    pub fadvise_advice: ::std::os::raw::c_uint,
    pub splice_flags: ::std::os::raw::c_uint,
    pub rename_flags: ::std::os::raw::c_uint,
    pub unlink_flags: ::std::os::raw::c_uint,
    pub hardlink_flags: ::std::os::raw::c_uint,
    pub xattr_flags: ::std::os::raw::c_uint,
    pub msg_ring_flags: ::std::os::raw::c_uint,
    pub uring_cmd_flags: ::std::os::raw::c_uint,
    pub waitid_flags: ::std::os::raw::c_uint,
    pub futex_flags: ::std::os::raw::c_uint,
    pub install_fd_flags: ::std::os::raw::c_uint,
}

impl Default for IoUringSqeUnion3 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl ::std::fmt::Debug for IoUringSqeUnion3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("IoUringSqeUnion3")
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union IoUringSqeUnion4 {
    pub buf_index: ::std::os::raw::c_ushort,
    pub buf_group: ::std::os::raw::c_ushort,
}

impl Default for IoUringSqeUnion4 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl ::std::fmt::Debug for IoUringSqeUnion4 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("IoUringSqeUnion4")
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union IoUringSqeUnion5 {
    pub splice_fd_in: ::std::os::raw::c_int,
    pub file_index: ::std::os::raw::c_uint,
    pub optlen: ::std::os::raw::c_uint,
    pub addr_len: IoUringSqeUnion5AddrLen,
}

impl Default for IoUringSqeUnion5 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl ::std::fmt::Debug for IoUringSqeUnion5 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("IoUringSqeUnion5")
    }
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct IoUringSqeUnion5AddrLen {
    pub addr_len: ::std::os::raw::c_ushort,
    pub __pad3: [::std::os::raw::c_ushort; 1usize],
}

#[repr(C)]
pub union IoUringSqeUnion6 {
    pub addr3: IoUringSqeUnion6Addr3,
    pub optval: ::std::os::raw::c_ulonglong,
    pub cmd: [::std::os::raw::c_uchar; 16usize],
}

impl Default for IoUringSqeUnion6 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl ::std::fmt::Debug for IoUringSqeUnion6 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("IoUringSqeUnion6")
    }
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct IoUringSqeUnion6Addr3 {
    pub addr3: ::std::os::raw::c_ulonglong,
    pub __pad2: [::std::os::raw::c_ulonglong; 1usize],
}

#[repr(C)]
#[derive(Debug)]
pub struct IoUringCqe {
    pub user_data: ::std::os::raw::c_ulonglong,
    pub res: ::std::os::raw::c_int,
    pub flags: ::std::os::raw::c_uint,
    pub big_cqe: [::std::os::raw::c_ulonglong; 0],
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct IoSqringOffsets {
    pub head: ::std::os::raw::c_uint,
    pub tail: ::std::os::raw::c_uint,
    pub ring_mask: ::std::os::raw::c_uint,
    pub ring_entries: ::std::os::raw::c_uint,
    pub flags: ::std::os::raw::c_uint,
    pub dropped: ::std::os::raw::c_uint,
    pub array: ::std::os::raw::c_uint,
    pub resv1: ::std::os::raw::c_uint,
    pub user_addr: ::std::os::raw::c_ulonglong,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct IoCqringOffsets {
    pub head: ::std::os::raw::c_uint,
    pub tail: ::std::os::raw::c_uint,
    pub ring_mask: ::std::os::raw::c_uint,
    pub ring_entries: ::std::os::raw::c_uint,
    pub overflow: ::std::os::raw::c_uint,
    pub cqes: ::std::os::raw::c_uint,
    pub flags: ::std::os::raw::c_uint,
    pub resv1: ::std::os::raw::c_uint,
    pub user_addr: ::std::os::raw::c_ulonglong,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct IoUringParams {
    pub sq_entries: ::std::os::raw::c_uint,
    pub cq_entries: ::std::os::raw::c_uint,
    pub flags: ::std::os::raw::c_uint,
    pub sq_thread_cpu: ::std::os::raw::c_uint,
    pub sq_thread_idle: ::std::os::raw::c_uint,
    pub features: ::std::os::raw::c_uint,
    pub wq_fd: ::std::os::raw::c_uint,
    pub resv: [::std::os::raw::c_uint; 3usize],
    pub sq_off: IoSqringOffsets,
    pub cq_off: IoCqringOffsets,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct IoUringFilesUpdate {
    pub offset: ::std::os::raw::c_uint,
    pub resv: ::std::os::raw::c_uint,
    pub fds: ::std::os::raw::c_ulonglong,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct IoUringRsrcRegister {
    pub nr: ::std::os::raw::c_uint,
    pub flags: ::std::os::raw::c_uint,
    pub resv2: ::std::os::raw::c_ulonglong,
    pub data: ::std::os::raw::c_ulonglong,
    pub tags: ::std::os::raw::c_ulonglong,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct IoUringRsrcUpdate {
    pub offset: ::std::os::raw::c_uint,
    pub resv: ::std::os::raw::c_uint,
    pub data: ::std::os::raw::c_ulonglong,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct IoUringRsrcUpdate2 {
    pub offset: ::std::os::raw::c_uint,
    pub resv: ::std::os::raw::c_uint,
    pub data: ::std::os::raw::c_ulonglong,
    pub tags: ::std::os::raw::c_ulonglong,
    pub nr: ::std::os::raw::c_uint,
    pub resv2: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct IoUringProbeOp {
    pub op: ::std::os::raw::c_uchar,
    pub resv: ::std::os::raw::c_uchar,
    pub flags: ::std::os::raw::c_ushort,
    pub resv2: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IoUringProbe {
    pub last_op: ::std::os::raw::c_uchar,
    pub ops_len: ::std::os::raw::c_uchar,
    pub resv: ::std::os::raw::c_ushort,
    pub resv2: [::std::os::raw::c_uint; 3usize],
    pub ops: [IoUringProbeOp; 256],
}

impl Default for IoUringProbe {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IoUringRestriction {
    pub opcode: ::std::os::raw::c_ushort,
    pub union1: IoUringRestrictionUnion1,
    pub resv: ::std::os::raw::c_uchar,
    pub resv2: [::std::os::raw::c_uint; 3usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union IoUringRestrictionUnion1 {
    pub register_op: ::std::os::raw::c_uchar,
    pub sqe_op: ::std::os::raw::c_uchar,
    pub sqe_flags: ::std::os::raw::c_uchar,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct IoUringBuf {
    pub addr: ::std::os::raw::c_ulonglong,
    pub len: ::std::os::raw::c_uint,
    pub bid: ::std::os::raw::c_ushort,
    pub tail: ::std::os::raw::c_ushort,
}

#[repr(C)]
pub struct IoUringBufRing {
    pub bufs: [IoUringBuf; 1usize],
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct IoUringBufReg {
    pub ring_addr: ::std::os::raw::c_ulonglong,
    pub ring_entries: ::std::os::raw::c_uint,
    pub bgid: ::std::os::raw::c_ushort,
    pub flags: ::std::os::raw::c_ushort,
    pub resv: [::std::os::raw::c_ulonglong; 3usize],
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct IoUringGeteventsArg {
    pub sigmask: ::std::os::raw::c_ulonglong,
    pub sigmask_sz: ::std::os::raw::c_uint,
    pub pad: ::std::os::raw::c_uint,
    pub ts: ::std::os::raw::c_ulonglong,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct IoUringSyncCancelReg {
    pub addr: ::std::os::raw::c_ulonglong,
    pub fd: ::std::os::raw::c_int,
    pub flags: ::std::os::raw::c_uint,
    pub timeout: KernelTimespec,
    pub pad: [::std::os::raw::c_ulonglong; 4usize],
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct IoUringFileIndexRange {
    pub off: ::std::os::raw::c_uint,
    pub len: ::std::os::raw::c_uint,
    pub resv: ::std::os::raw::c_ulonglong,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct IoUringRecvmsgOut {
    pub namelen: ::std::os::raw::c_uint,
    pub controllen: ::std::os::raw::c_uint,
    pub payloadlen: ::std::os::raw::c_uint,
    pub flags: ::std::os::raw::c_uint,
}

mod tests {
    #[test]
    fn check_io_uring_sqe_size() {
        assert_eq!(std::mem::size_of::<super::IoUringSqe>(), 64);
    }
}
