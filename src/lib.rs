mod acceptor;
mod backoff;
mod complete_queue;
mod complete_queue_entry;
mod config;
mod constants;
mod flags;
mod mmap_buffer;
mod params;
mod recv_buffer;
mod submit_queue;
mod submit_queue_entry;
mod syscall;

use complete_queue::CompleteQueue;
use complete_queue_entry::CompleteQueueEntry;
pub use config::Config;
pub use mmap_buffer::MmapBuffer;
use params::IoUringParams;
pub use recv_buffer::{RecvBuffer, RecvBufferGuard};
use submit_queue::SubmitQueue;
pub use submit_queue_entry::SubmitQueueEntry;

pub fn create(config: &Config) -> std::io::Result<(SubmitQueue, CompleteQueue)> {
    let mut p = IoUringParams::default();
    let ring_fd = std::sync::Arc::new(syscall::io_uring_setup(config.entries, &mut p)?);
    let sq = SubmitQueue::new(ring_fd.clone(), &p)?;
    let cq = CompleteQueue::new(ring_fd.clone(), &p, &sq.ring_buffer)?;

    Ok((sq, cq))
}

mod tests {
    #[test]
    fn io_uring_setup() -> std::io::Result<()> {
        use super::*;

        let config = Config::new(1024);
        let (mut sq, mut cq) = create(&config)?;

        let sqe = sq.get_sqe()?;
        *sqe = SubmitQueueEntry {
            opcode: constants::OpCode::NOP,
            ..Default::default()
        };

        assert_eq!(sq.flush()?, 1);

        cq.reap(1)?;

        let reap_count = cq.for_each_cqe(|cqe| {
            println!("cqe: {cqe:?}");
        });
        assert_eq!(reap_count, 1);

        Ok(())
    }

    #[test]
    fn io_uring_multithreads() -> std::io::Result<()> {
        use super::*;

        let config = Config::new(1024);
        let (mut sq, mut cq) = create(&config)?;

        const SUBMIT_COUNT: u32 = 1000000;
        const BATCH_SIZE: u32 = 1000;
        let submit_thread = std::thread::spawn(move || {
            for i in 1..=SUBMIT_COUNT {
                let sqe = sq.get_sqe()?;
                *sqe = SubmitQueueEntry {
                    opcode: constants::OpCode::NOP,
                    ..Default::default()
                };

                if i % BATCH_SIZE == 0 {
                    assert_eq!(sq.flush()?, BATCH_SIZE as _);
                }
            }

            std::io::Result::Ok(())
        });

        let mut reap_count = 0u32;
        while reap_count < SUBMIT_COUNT {
            cq.reap(1)?;
            reap_count += cq.for_each_cqe(|_| {});
        }

        let _ = submit_thread.join();
        Ok(())
    }
}
