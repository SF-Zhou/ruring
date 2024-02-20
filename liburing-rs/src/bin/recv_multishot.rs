use std::net::{TcpListener, TcpStream};
use std::os::fd::FromRawFd;

use liburing_rs::entry::OpType;
use liburing_rs::*;

use std::sync::atomic::AtomicU64;
use std::sync::Arc;

/// Test with:
///   cat /dev/zero | pv | nc 127.0.0.1 8000

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8000")?;

    let mut params = IoUringParams::default();
    params
        .flags
        .insert(flags::SetupFlags::SINGLE_ISSUER | flags::SetupFlags::COOP_TASKRUN);
    let mut ring = IoUring::new(4096, &mut params)?;

    let mut buffer_group = ring.setup_buffer_ring(1, 1024, 256 * 1024usize)?;

    ring.prep_accept(&listener)?;
    ring.sq_flush();
    let submitted = ring.submit(1, 0)?;
    assert_eq!(submitted, 1);

    let count = Arc::new(AtomicU64::default());
    let count_copy = count.clone();
    let consumed = Arc::new(AtomicU64::default());
    let consumed_copy = consumed.clone();
    let bytes = Arc::new(AtomicU64::default());
    let bytes_copy = bytes.clone();
    let write_bytes = Arc::new(AtomicU64::default());
    let write_bytes_copy = write_bytes.clone();
    let _ = std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let consume = consumed_copy.swap(0, atomic::Ordering::SeqCst);
        let count = count_copy.swap(0, atomic::Ordering::SeqCst);
        println!(
            "consume {} count {} avg {} bytes {} write {}",
            consume,
            count,
            consume as f64 / count as f64,
            bytes_copy.swap(0, atomic::Ordering::SeqCst),
            write_bytes_copy.swap(0, atomic::Ordering::SeqCst),
        );
    });

    loop {
        let mut total = 0u32;
        // std::thread::sleep(std::time::Duration::from_micros(1));
        count.fetch_add(1, atomic::Ordering::SeqCst);
        let consume = ring.for_each_cqe(|ring, cqe| {
            if cqe.op_type == OpType::Accept {
                if cqe.res >= 0 {
                    println!("accept {cqe:?}");
                    let sock = unsafe { TcpStream::from_raw_fd(cqe.res) };
                    match ring.prep_recv(std::sync::Arc::new(sock), 1) {
                        Ok(_) => total += 1,
                        Err(err) => println!("prep recv failed: {err:?}"),
                    }
                } else {
                    println!("Accept Error: {:#?}", cqe);
                }
            } else if cqe.op_type == OpType::Receive {
                if cqe.res == -libc::ENOBUFS {
                    let sock = cqe.fd.as_ref().unwrap().clone();
                    match ring.prep_recv(sock, 1) {
                        Ok(_) => total += 1,
                        Err(err) => println!("prep recv failed: {err:?}"),
                    }
                } else if cqe.res > 0 {
                    bytes.fetch_add(cqe.res as _, atomic::Ordering::SeqCst);
                    let sock = cqe.fd.as_ref().unwrap().clone();
                    match ring.prep_send_zc(sock, cqe.bid, buffer_group.addr(cqe.bid), cqe.res as _)
                    {
                        Ok(_) => total += 1,
                        Err(err) => println!("prep send failed: {err:?}"),
                    }
                } else {
                    println!("recv error: {cqe:?}");
                }
            } else if cqe.op_type == OpType::SendZeroCopy {
                if cqe.res >= 0 {
                    write_bytes.fetch_add(cqe.res as _, atomic::Ordering::SeqCst);
                    if cqe.flags.contains(flags::CQEFlags::MORE) {
                    } else if cqe.flags.contains(flags::CQEFlags::NOTIF) {
                        buffer_group.recycle(cqe.send_bid);
                    }
                } else {
                    println!("send error: {cqe:?}");
                }
            } else if cqe.op_type == OpType::Send {
                if cqe.res < 0 {
                    println!("send error: {cqe:?}");
                } else {
                    write_bytes.fetch_add(cqe.res as _, atomic::Ordering::SeqCst);
                    if cqe.len != cqe.res as _ {
                        println!("send error {} != {}", cqe.len, cqe.res);
                    }
                }
                buffer_group.recycle(cqe.send_bid);
            } else {
                println!("other {cqe:?}");
            }
        });
        consumed.fetch_add(consume as _, atomic::Ordering::SeqCst);

        if total > 0 {
            ring.sq_flush();
        }
        let _ = ring.submit(total, 1);
    }
}
