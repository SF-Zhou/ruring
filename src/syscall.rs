use crate::{flags::*, params::IoUringParams};
use libc::{c_long, c_uint, c_void, sigset_t, syscall};
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};

const NR_IO_URING_SETUP: c_long = 425;
const NR_IO_URING_ENTER: c_long = 426;
const NR_IO_URING_REGISTER: c_long = 427;

pub fn io_uring_setup(entries: c_uint, p: &mut IoUringParams) -> std::io::Result<OwnedFd> {
    let ret = unsafe {
        syscall(
            NR_IO_URING_SETUP,
            entries,
            p as *mut IoUringParams as c_long,
        )
    };
    if ret < 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(unsafe { OwnedFd::from_raw_fd(RawFd::try_from(ret).unwrap()) })
    }
}

pub fn io_uring_enter(
    fd: &impl AsRawFd,
    to_submit: c_uint,
    min_complete: c_uint,
    flags: EnterFlags,
    sig: *mut sigset_t,
) -> std::io::Result<c_uint> {
    loop {
        let ret = unsafe {
            syscall(
                NR_IO_URING_ENTER,
                fd.as_raw_fd(),
                to_submit,
                min_complete,
                flags,
                sig as c_long,
                std::mem::size_of::<sigset_t>() as c_long,
            )
        };
        if ret < 0 {
            let err = std::io::Error::last_os_error();
            if err.kind() == std::io::ErrorKind::Interrupted {
                continue;
            }
            return Err(err);
        } else {
            return Ok(c_uint::try_from(ret).unwrap());
        }
    }
}

pub fn io_uring_register(
    fd: &impl AsRawFd,
    opcode: c_uint,
    arg: *mut c_void,
    nr_args: c_uint,
) -> std::io::Result<c_uint> {
    let ret = unsafe {
        syscall(
            NR_IO_URING_REGISTER,
            fd.as_raw_fd(),
            opcode,
            arg as c_long,
            nr_args,
        )
    };
    if ret < 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(c_uint::try_from(ret).unwrap())
    }
}

mod tests {
    #[test]
    fn io_uring_setup() -> std::io::Result<()> {
        use crate::flags::*;
        use std::io::Read;
        use std::os::fd::{FromRawFd, IntoRawFd};

        let mut params = super::IoUringParams::default();
        let result = super::io_uring_setup(0, &mut params);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidInput);

        let mut params = super::IoUringParams {
            resv: [1u32; 3],
            ..Default::default()
        };
        let result = super::io_uring_setup(1, &mut params);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidInput);

        let mut params = super::IoUringParams {
            flags: SetupFlags::from_bits_retain(!0u32),
            ..Default::default()
        };
        let result = super::io_uring_setup(1, &mut params);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidInput);

        let mut params = super::IoUringParams {
            flags: SetupFlags::SQ_AFF,
            ..Default::default()
        };
        let result = super::io_uring_setup(1, &mut params);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidInput);

        let mut params = super::IoUringParams::default();
        assert_eq!(params.cq_entries, 0);
        let io_uring = super::io_uring_setup(1, &mut params)?;
        assert_eq!(params.cq_entries, 2); // mutated by io_uring_setup.
        assert_eq!(params.sq_entries, 1); // mutated by io_uring_setup.

        let mut buffer = String::new();
        let mut file = unsafe { std::fs::File::from_raw_fd(io_uring.into_raw_fd()) };
        let read_reuslt = file.read_to_string(&mut buffer);
        assert!(read_reuslt.is_err());

        Ok(())
    }

    #[test]
    fn io_uring_enter() -> std::io::Result<()> {
        let mut params = super::IoUringParams::default();
        assert_eq!(params.cq_entries, 0);
        let io_uring = super::io_uring_setup(1, &mut params)?;

        crate::syscall::io_uring_enter(&io_uring, 0, 0, Default::default(), std::ptr::null_mut())?;
        Ok(())
    }

    #[test]
    fn io_uring_register_buffers() -> std::io::Result<()> {
        let mut params = super::IoUringParams::default();
        assert_eq!(params.cq_entries, 0);
        let io_uring = super::io_uring_setup(1, &mut params)?;

        const IORING_REGISTER_BUFFERS: u32 = 0;
        let mut buffer = [0u8; 4096];
        let mut iovec = libc::iovec {
            iov_base: buffer.as_mut_ptr() as _,
            iov_len: buffer.len(),
        };
        crate::syscall::io_uring_register(
            &io_uring,
            IORING_REGISTER_BUFFERS,
            &mut iovec as *mut _ as _,
            1,
        )?;
        Ok(())
    }
}
