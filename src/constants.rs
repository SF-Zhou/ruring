use bitflags::bitflags;

bitflags! {
    #[repr(C)]
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct OpCode: u8 {
        const NOP = 0;
        const READV = 1;
        const WRITEV = 2;
        const FSYNC = 3;
        const READ_FIXED = 4;
        const WRITE_FIXED = 5;
        const POLL_ADD = 6;
        const POLL_REMOVE = 7;
        const SYNC_FILE_RANGE = 8;
        const SENDMSG = 9;
        const RECVMSG = 10;
        const TIMEOUT = 11;
        const TIMEOUT_REMOVE = 12;
        const ACCEPT = 13;
        const ASYNC_CANCEL = 14;
        const LINK_TIMEOUT = 15;
        const CONNECT = 16;
        const FALLOCATE = 17;
        const OPENAT = 18;
        const CLOSE = 19;
        const FILES_UPDATE = 20;
        const STATX = 21;
        const READ = 22;
        const WRITE = 23;
        const FADVISE = 24;
        const MADVISE = 25;
        const SEND = 26;
        const RECV = 27;
        const OPENAT2 = 28;
        const EPOLL_CTL = 29;
        const SPLICE = 30;
        const PROVIDE_BUFFERS = 31;
        const REMOVE_BUFFERS = 32;
        const TEE = 33;
        const SHUTDOWN = 34;
        const RENAMEAT = 35;
        const UNLINKAT = 36;
        const MKDIRAT = 37;
        const SYMLINKAT = 38;
        const LINKAT = 39;
        const MSG_RING = 40;
        const FSETXATTR = 41;
        const SETXATTR = 42;
        const FGETXATTR = 43;
        const GETXATTR = 44;
        const SOCKET = 45;
        const URING_CMD = 46;
        const SEND_ZC = 47;
        const SENDMSG_ZC = 48;
        const READ_MULTISHOT = 49;
        const WAITID = 50;
        const FUTEX_WAIT = 51;
        const FUTEX_WAKE = 52;
        const FUTEX_WAITV = 53;
        const FIXED_FD_INSTALL = 54;
        const LAST = 55;
        const _ = !0;
    }
}

bitflags! {
    #[repr(C)]
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Offset: i64 {
        const SQ_RING = 0;
        const CQ_RING = 0x08000000;
        const SQES = 0x10000000;
        const PBUF_RING = 0x80000000;
        const PBUF_SHIFT = 16;
        const MMAP_MASK = 0xF8000000;
    }
}

bitflags! {
    #[repr(C)]
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct RegisterCode: u32 {
        const REGISTER_BUFFERS =  0;
        const UNREGISTER_BUFFERS =  1;
        const REGISTER_FILES =  2;
        const UNREGISTER_FILES =  3;
        const REGISTER_EVENTFD =  4;
        const UNREGISTER_EVENTFD =  5;
        const REGISTER_FILES_UPDATE =  6;
        const REGISTER_EVENTFD_ASYNC =  7;
        const REGISTER_PROBE =  8;
        const REGISTER_PERSONALITY =  9;
        const UNREGISTER_PERSONALITY =  10;
        const REGISTER_RESTRICTIONS =  11;
        const REGISTER_ENABLE_RINGS =  12;
        const REGISTER_FILES2 =  13;
        const REGISTER_FILES_UPDATE2 =  14;
        const REGISTER_BUFFERS2 =  15;
        const REGISTER_BUFFERS_UPDATE =  16;
        const REGISTER_IOWQ_AFF =  17;
        const UNREGISTER_IOWQ_AFF =  18;
        const REGISTER_IOWQ_MAX_WORKERS =  19;
        const REGISTER_RING_FDS =  20;
        const UNREGISTER_RING_FDS =  21;
        const REGISTER_PBUF_RING =  22;
        const UNREGISTER_PBUF_RING =  23;
        const REGISTER_SYNC_CANCEL =  24;
        const REGISTER_FILE_ALLOC_RANGE =  25;
        const REGISTER_LAST =  26;
        const REGISTER_USE_REGISTERED_RING =  2147483648;
    }
}
