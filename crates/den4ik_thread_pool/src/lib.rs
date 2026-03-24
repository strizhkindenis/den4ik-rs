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
    collections::VecDeque,
    num::NonZeroUsize,
    sync::{
        Arc, Condvar, Mutex,
        atomic::{self, AtomicBool},
    },
    thread::{self, JoinHandle},
};

const FALLBACK_NUM_OF_THREADS: NonZeroUsize = NonZeroUsize::new(4).unwrap();
type Job = dyn FnOnce() + Send + 'static;

pub struct ThreadPool {
    threads: Vec<JoinHandle<()>>,
    job_queue: Arc<Mutex<VecDeque<Box<Job>>>>,
    job_cvar: Arc<Condvar>,
    panic_queue: Arc<Mutex<VecDeque<usize>>>,
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
        let job_queue = Arc::new(Mutex::new(VecDeque::with_capacity(num_of_threads)));
        let job_cvar = Arc::new(Condvar::new());
        let panic_queue = Arc::new(Mutex::new(VecDeque::with_capacity(num_of_threads)));
        let is_active = Arc::new(AtomicBool::new(true));
        let threads = (0..num_of_threads)
            .map(|panic_idx| {
                start_worker(
                    job_queue.clone(),
                    job_cvar.clone(),
                    is_active.clone(),
                    panic_queue.clone(),
                    panic_idx,
                )
            })
            .collect();
        Self {
            threads,
            job_queue,
            job_cvar,
            panic_queue,
            is_active,
        }
    }

    pub fn submit<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let mut panic_queue = self.panic_queue.lock().unwrap_or_else(|_| unreachable!());
        while let Some(panic_idx) = panic_queue.pop_front() {
            let handle = start_worker(
                self.job_queue.clone(),
                self.job_cvar.clone(),
                self.is_active.clone(),
                self.panic_queue.clone(),
                panic_idx,
            );
            let thread = unsafe { &mut *(&raw const self.threads[panic_idx]).cast_mut() };
            let handle = std::mem::replace(thread, handle);
            let Err(e) = handle.join() else {
                unreachable!();
            };
            eprintln!("Worker #{panic_idx} panicked: {e:?}");
        }
        drop(panic_queue);
        self.job_queue
            .lock()
            .unwrap_or_else(|_| unreachable!())
            .push_back(Box::new(job));
        self.job_cvar.notify_one();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.is_active.store(false, atomic::Ordering::Relaxed);
        self.job_cvar.notify_all();
        for handle in self.threads.drain(..) {
            let _ = handle.join();
        }
    }
}

fn worker_next_job(
    queue: &Mutex<VecDeque<Box<Job>>>,
    cvar: &Condvar,
    is_active: &AtomicBool,
) -> Option<Box<Job>> {
    let mut queue = queue.lock().unwrap_or_else(|_| unreachable!());
    loop {
        if let Some(job) = queue.pop_front() {
            return Some(job);
        }
        if !is_active.load(atomic::Ordering::Relaxed) {
            return None;
        }
        queue = cvar.wait(queue).unwrap_or_else(|_| unreachable!());
    }
}

fn start_worker(
    job_queue: Arc<Mutex<VecDeque<Box<Job>>>>,
    job_cvar: Arc<Condvar>,
    is_active: Arc<AtomicBool>,
    panic_queue: Arc<Mutex<VecDeque<usize>>>,
    panic_idx: usize,
) -> JoinHandle<()> {
    use std::panic;
    thread::spawn(move || {
        while let Some(job) = worker_next_job(&job_queue, &job_cvar, &is_active) {
            let Err(payload) = panic::catch_unwind(panic::AssertUnwindSafe(job)) else {
                continue;
            };
            panic_queue
                .lock()
                .unwrap_or_else(|_| unreachable!())
                .push_back(panic_idx);
            panic::resume_unwind(payload);
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

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
