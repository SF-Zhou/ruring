use bitflags::bitflags;

bitflags! {
    #[repr(C)]
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct SetupFlags: u32 {
        const IOPOLL = 1;
        const SQPOLL = 2;
        const SQ_AFF = 4;
        const CQSIZE = 8;
        const CLAMP = 16;
        const ATTACH_WQ = 32;
        const R_DISABLED = 64;
        const SUBMIT_ALL = 128;
        const COOP_TASKRUN = 256;
        const TASKRUN_FLAG = 512;
        const SQE128 = 1024;
        const CQE32 = 2048;
        const SINGLE_ISSUER = 4096;
        const DEFER_TASKRUN = 8192;
        const NO_MMAP = 16384;
        const REGISTERED_FD_ONLY = 32768;
        const NO_SQARRAY = 65536;
        const _ = !0;
    }

    #[repr(C)]
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct FeatureFlags: u32 {
        const SINGLE_MMAP = 1;
        const NODROP = 2;
        const SUBMIT_STABLE = 4;
        const RW_CUR_POS = 8;
        const CUR_PERSONALITY = 16;
        const FAST_POLL = 32;
        const POLL_32BITS = 64;
        const SQPOLL_NONFIXED = 128;
        const EXT_ARG = 256;
        const NATIVE_WORKERS = 512;
        const RSRC_TAGS = 1024;
        const CQE_SKIP = 2048;
        const LINKED_FILE = 4096;
        const REG_REG_RING = 8192;
        const _ = !0;
    }

    #[repr(C)]
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct SQFlags: u32 {
        const NEED_WAKEUP = 1;
        const CQ_OVERFLOW = 2;
        const TASKRUN = 4;
        const _ = !0;
    }

    #[repr(C)]
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct CQFlags: u32 {
        const EVENTFD_DISABLED = 1;
        const _ = !0;
    }

    #[repr(C)]
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct EnterFlags: u32 {
        const GETEVENTS = 1;
        const SQ_WAKEUP = 2;
        const SQ_WAIT = 4;
        const EXT_ARG = 8;
        const REGISTERED_RING = 16;
        const _ = !0;
    }

    #[repr(C)]
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ProbeOpFlags: u16 {
        const SUPPORTED = 1;
        const _ = !0;
    }
}

unsafe impl bytemuck::NoUninit for SQFlags {}
unsafe impl bytemuck::NoUninit for CQFlags {}
