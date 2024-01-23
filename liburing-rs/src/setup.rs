use crate::*;
use std::os::raw::{c_uint, c_void};
use std::{fs::File, os::fd::AsRawFd};

fn ptr_err<T>(ptr: *mut T) -> std::io::Error {
    std::io::Error::from_raw_os_error(-(ptr as i32))
}

fn is_err<T>(ptr: *mut T) -> bool {
    ptr as usize >= -4095i64 as usize
}

fn io_uring_unmap_rings(sq: &mut IoUringSq, cq: &mut IoUringCq) {
    if sq.ring_sz != 0 {
        unsafe { libc::munmap(sq.ring_ptr, sq.ring_sz) };
    }
    if !cq.ring_ptr.is_null() && cq.ring_sz != 0 && cq.ring_ptr != sq.ring_ptr {
        unsafe { libc::munmap(cq.ring_ptr, cq.ring_sz) };
    }
}

fn io_uring_setup_ring_pointers(
    p: &mut io_uring::IoUringParams,
    sq: &mut IoUringSq,
    cq: &mut IoUringCq,
) {
    sq.khead = sq.ring_ptr.wrapping_add(p.sq_off.head as usize) as *mut c_uint;
    sq.ktail = sq.ring_ptr.wrapping_add(p.sq_off.tail as usize) as *mut c_uint;
    sq.kflags = sq.ring_ptr.wrapping_add(p.sq_off.flags as usize) as *mut c_uint;
    sq.kdropped = sq.ring_ptr.wrapping_add(p.sq_off.dropped as usize) as *mut c_uint;
    sq.array = sq.ring_ptr.wrapping_add(p.sq_off.array as usize) as *mut c_uint;

    cq.khead = cq.ring_ptr.wrapping_add(p.cq_off.head as usize) as *mut c_uint;
    cq.ktail = cq.ring_ptr.wrapping_add(p.cq_off.tail as usize) as *mut c_uint;
    cq.koverflow = cq.ring_ptr.wrapping_add(p.cq_off.overflow as usize) as *mut c_uint;
    cq.cqes = cq.ring_ptr.wrapping_add(p.cq_off.cqes as usize) as *mut io_uring::IoUringCqe;
    if p.cq_off.flags != 0 {
        cq.kflags = cq.ring_ptr.wrapping_add(p.cq_off.flags as usize) as *mut c_uint;
    }

    unsafe {
        sq.ring_mask = *(sq.ring_ptr.wrapping_add(p.sq_off.ring_mask as usize) as *mut c_uint);
        sq.ring_entries =
            *(sq.ring_ptr.wrapping_add(p.sq_off.ring_entries as usize) as *mut c_uint);
        cq.ring_mask = *(cq.ring_ptr.wrapping_add(p.cq_off.ring_mask as usize) as *mut c_uint);
        cq.ring_entries =
            *(cq.ring_ptr.wrapping_add(p.cq_off.ring_entries as usize) as *mut c_uint);
    }
}

fn io_uring_mmap(
    fd: &File,
    p: &mut io_uring::IoUringParams,
    sq: &mut IoUringSq,
    cq: &mut IoUringCq,
) -> std::io::Result<()> {
    let mut size = std::mem::size_of::<io_uring::IoUringCqe>();
    if (p.flags & io_uring::IORING_SETUP_CQE32) != 0 {
        size += std::mem::size_of::<io_uring::IoUringCqe>();
    }

    sq.ring_sz = p.sq_off.array as usize + p.sq_entries as usize * std::mem::size_of::<u32>();
    cq.ring_sz = p.cq_off.cqes as usize + p.cq_entries as usize * size;

    if (p.features & io_uring::IORING_FEAT_SINGLE_MMAP) != 0 {
        if cq.ring_sz > sq.ring_sz {
            sq.ring_sz = cq.ring_sz;
        }
        cq.ring_sz = sq.ring_sz;
    }

    sq.ring_ptr = unsafe {
        libc::mmap(
            std::ptr::null_mut(),
            sq.ring_sz,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED | libc::MAP_POPULATE,
            fd.as_raw_fd(),
            io_uring::IORING_OFF_SQ_RING,
        )
    };
    if is_err(sq.ring_ptr) {
        return Err(ptr_err(sq.ring_ptr));
    }

    if (p.features & io_uring::IORING_FEAT_SINGLE_MMAP) != 0 {
        cq.ring_ptr = sq.ring_ptr;
    } else {
        cq.ring_ptr = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                cq.ring_sz,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED | libc::MAP_POPULATE,
                fd.as_raw_fd(),
                io_uring::IORING_OFF_CQ_RING,
            )
        };
        if is_err(cq.ring_ptr) {
            let err = ptr_err(cq.ring_ptr);
            cq.ring_ptr = std::ptr::null_mut();
            io_uring_unmap_rings(sq, cq);
            return Err(err);
        }
    }

    let mut size = std::mem::size_of::<io_uring::IoUringSqe>();
    if p.flags & io_uring::IORING_SETUP_SQE128 != 0 {
        size += 64;
    }
    sq.sqes = unsafe {
        libc::mmap(
            std::ptr::null_mut(),
            size * p.sq_entries as usize,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED | libc::MAP_POPULATE,
            fd.as_raw_fd(),
            io_uring::IORING_OFF_SQES,
        ) as *mut io_uring::IoUringSqe
    };
    if is_err(sq.sqes) {
        let err = ptr_err(sq.sqes);
        io_uring_unmap_rings(sq, cq);
        return Err(err);
    }

    io_uring_setup_ring_pointers(p, sq, cq);
    Ok(())
}

pub fn io_uring_queue_init_params(
    entries: u32,
    p: &mut io_uring::IoUringParams,
) -> std::io::Result<IoUring> {
    if p.flags & io_uring::IORING_SETUP_REGISTERED_FD_ONLY != 0
        && p.flags & io_uring::IORING_SETUP_NO_MMAP == 0
    {
        return Err(std::io::ErrorKind::InvalidInput.into());
    }

    if p.flags & io_uring::IORING_SETUP_NO_MMAP != 0 {
        unimplemented!("IORING_SETUP_NO_MMAP");
    }
    if p.flags & io_uring::IORING_SETUP_REGISTERED_FD_ONLY != 0 {
        unimplemented!("IORING_SETUP_REGISTERED_FD_ONLY");
    }

    let fd = syscall::io_uring_setup(entries, p)?;

    let mut sq = IoUringSq::default();
    let mut cq = IoUringCq::default();
    io_uring_mmap(&fd, p, &mut sq, &mut cq)?;

    for index in 0..sq.ring_entries {
        unsafe { *sq.array.wrapping_add(index as usize) = index };
    }

    Ok(IoUring {
        ring_fd: fd,
        sq,
        cq,
        flags: p.flags,
        features: p.features,
    })
}

impl Drop for IoUring {
    fn drop(&mut self) {
        let mut sqe_size = std::mem::size_of::<io_uring::IoUringSqe>();
        if self.flags & io_uring::IORING_SETUP_SQE128 != 0 {
            sqe_size += 64;
        }
        unsafe {
            libc::munmap(
                self.sq.sqes as *mut c_void,
                sqe_size * self.sq.ring_entries as usize,
            );
        }
        io_uring_unmap_rings(&mut self.sq, &mut self.cq);
    }
}

mod tests {
    #[test]
    fn io_uring_mmap() -> std::io::Result<()> {
        use super::*;

        let mut params = io_uring::IoUringParams::default();
        let ring_fd = syscall::io_uring_setup(1, &mut params)?;

        let mut sq = IoUringSq::default();
        let mut cq = IoUringCq::default();
        io_uring_mmap(&ring_fd, &mut params, &mut sq, &mut cq)?;

        Ok(())
    }

    #[test]
    fn io_uring_queue_init_params() -> std::io::Result<()> {
        use super::*;

        let mut params = IoUringParams::default();
        let ring = io_uring_queue_init_params(4096, &mut params)?;
        println!("ring is {ring:#?}");

        Ok(())
    }
}
