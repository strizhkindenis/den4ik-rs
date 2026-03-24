#![deny(
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::pedantic,
    clippy::cargo
)]

use std::{
    num::NonZeroUsize,
    sync::{
        Arc, Condvar, Mutex,
        atomic::{self, AtomicBool},
        mpsc::{self, Receiver, Sender},
    },
    thread::{self, JoinHandle},
};

const FALLBACK_NUM_OF_THREADS: NonZeroUsize = NonZeroUsize::new(4).unwrap();
type Job = dyn FnOnce() + Send + 'static;

pub struct ThreadPool {
    threads: Vec<JoinHandle<()>>,
    job_tx: Sender<Box<Job>>,
    job_rx: Arc<Mutex<Receiver<Box<Job>>>>,
    job_cvar: Arc<Condvar>,
    panic_rx: Receiver<usize>,
    panic_tx: Sender<usize>,
    is_active: Arc<AtomicBool>,
}

impl Default for ThreadPool {
    fn default() -> Self {
        let num_of_threads = thread::available_parallelism()
            .inspect_err(|e| {
                eprintln!(
                    "Failed to get the number of available threads: {e:?}. \
               Using {FALLBACK_NUM_OF_THREADS}",
                );
            })
            .unwrap_or(FALLBACK_NUM_OF_THREADS);
        Self::new(num_of_threads.into())
    }
}

impl ThreadPool {
    #[must_use]
    pub fn new(num_of_threads: usize) -> Self {
        let (job_tx, job_rx) = mpsc::channel();
        let job_rx = Arc::new(Mutex::new(job_rx));
        let job_cvar = Arc::new(Condvar::new());
        let (panic_tx, panic_rx) = mpsc::channel();
        let is_active = Arc::new(AtomicBool::new(true));
        let threads = (0..num_of_threads)
            .map(|i| {
                start_worker(
                    job_rx.clone(),
                    job_cvar.clone(),
                    is_active.clone(),
                    i,
                    panic_tx.clone(),
                )
            })
            .collect();
        Self {
            threads,
            job_tx,
            job_rx,
            job_cvar,
            panic_rx,
            panic_tx,
            is_active,
        }
    }

    pub fn submit<F>(&mut self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        while let Ok(idx) = self.panic_rx.try_recv() {
            let handle = start_worker(
                self.job_rx.clone(),
                self.job_cvar.clone(),
                self.is_active.clone(),
                idx,
                self.panic_tx.clone(),
            );
            let handle = std::mem::replace(&mut self.threads[idx], handle);
            let Err(e) = handle.join() else {
                unreachable!();
            };
            eprintln!("Worker #{idx} panicked: {e:?}");
        }
        self.job_tx
            .send(Box::new(job))
            .unwrap_or_else(|_| unreachable!());
        self.job_cvar.notify_one();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.is_active.store(false, atomic::Ordering::Relaxed);
        for handle in self.threads.drain(..) {
            let _ = handle.join();
        }
    }
}

fn worker_next_job(job_rx: &Mutex<Receiver<Box<Job>>>, job_cvar: &Condvar) -> Box<Job> {
    let job_rx = job_rx.lock().unwrap_or_else(|_| unreachable!());
    if let Ok(job) = job_rx.try_recv() {
        job
    } else {
        let job_rx = job_cvar.wait(job_rx).unwrap_or_else(|_| unreachable!());
        job_rx.recv().unwrap_or_else(|_| unreachable!())
    }
}

fn start_worker(
    job_rx: Arc<Mutex<Receiver<Box<Job>>>>,
    job_cvar: Arc<Condvar>,
    is_active: Arc<AtomicBool>,
    panic_idx: usize,
    panic_tx: Sender<usize>,
) -> JoinHandle<()> {
    use std::panic::{AssertUnwindSafe, catch_unwind};
    thread::spawn(move || {
        while is_active.load(atomic::Ordering::Relaxed) {
            let job = worker_next_job(&job_rx, &job_cvar);
            if catch_unwind(AssertUnwindSafe(job)).is_err() {
                panic_tx.send(panic_idx).unwrap_or_else(|_| unreachable!());
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_pool() {
        let n = 1_000_000;
        let mut pool = ThreadPool::default();
        let (tx, rx) = mpsc::channel();
        for i in 1..=n {
            let tx = tx.clone();
            pool.submit(move || {
                thread::sleep(std::time::Duration::from_millis(10));
                tx.send(i * 2).unwrap();
            });
            pool.submit(move || {
                thread::sleep(std::time::Duration::from_millis(10));
            });
        }
        drop(tx);
        let result_pool = rx.iter().sum::<u64>();
        let result_check = (1..=n).map(|x| x * 2).sum::<u64>();
        assert_eq!(result_check, result_pool);
    }
}
