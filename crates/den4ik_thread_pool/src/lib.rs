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
    fmt,
    num::NonZeroUsize,
    sync::{
        Arc, Condvar, Mutex, MutexGuard,
        atomic::{self, AtomicBool},
    },
    thread::{self, JoinHandle},
};

const FALLBACK_NUM_OF_THREADS: NonZeroUsize = NonZeroUsize::new(4).unwrap();
type Job = dyn FnOnce() + Send + 'static;

fn mutex_lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|p| {
        m.clear_poison();
        p.into_inner()
    })
}

fn mutex_wait<'a, T>(m: &Mutex<T>, g: MutexGuard<'a, T>, cv: &Condvar) -> MutexGuard<'a, T> {
    cv.wait(g).unwrap_or_else(|p| {
        m.clear_poison();
        p.into_inner()
    })
}

pub struct ThreadPool {
    threads: Mutex<Vec<JoinHandle<()>>>,
    job_queue: Arc<Mutex<VecDeque<Box<Job>>>>,
    job_cvar: Arc<Condvar>,
    panic_queue: Arc<Mutex<VecDeque<usize>>>,
    is_active: Arc<AtomicBool>,
}

impl fmt::Debug for ThreadPool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ThreadPool")
            .field("threads", &mutex_lock(&self.threads).len())
            .field("job_queue", &mutex_lock(&self.job_queue).len())
            .field("job_cvar", &self.job_cvar)
            .field("is_active", &self.is_active)
            .field("panic_queue", &mutex_lock(&self.panic_queue).len())
            .finish()
    }
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
        Self::new(num_of_threads)
    }
}

impl ThreadPool {
    #[must_use]
    pub fn new(num_of_threads: NonZeroUsize) -> Self {
        let job_queue = Arc::new(Mutex::new(VecDeque::new()));
        let job_cvar = Arc::new(Condvar::new());
        let panic_queue = Arc::new(Mutex::new(VecDeque::new()));
        let is_active = Arc::new(AtomicBool::new(true));
        let threads = (0..num_of_threads.into())
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
            threads: Mutex::new(threads),
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
        let mut panic_queue = mutex_lock(&self.panic_queue);
        let mut threads = mutex_lock(&self.threads);
        while let Some(panic_idx) = panic_queue.pop_front() {
            let handle = start_worker(
                self.job_queue.clone(),
                self.job_cvar.clone(),
                self.is_active.clone(),
                self.panic_queue.clone(),
                panic_idx,
            );
            let handle = std::mem::replace(&mut threads[panic_idx], handle);
            let Err(e) = handle.join() else {
                unreachable!();
            };
            eprintln!("Worker #{panic_idx} panicked: {e:?}");
        }
        drop(panic_queue);
        drop(threads);
        mutex_lock(&self.job_queue).push_back(Box::new(job));
        self.job_cvar.notify_one();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.is_active.store(false, atomic::Ordering::Relaxed);
        self.job_cvar.notify_all();
        for handle in mutex_lock(&self.threads).drain(..) {
            let _ = handle.join();
        }
    }
}

fn worker_next_job(
    queue: &Mutex<VecDeque<Box<Job>>>,
    cvar: &Condvar,
    is_active: &AtomicBool,
) -> Option<Box<Job>> {
    let mut guard = mutex_lock(queue);
    loop {
        if let Some(job) = guard.pop_front() {
            return Some(job);
        }
        if !is_active.load(atomic::Ordering::Relaxed) {
            return None;
        }
        guard = mutex_wait(queue, guard, cvar);
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
            mutex_lock(&panic_queue).push_back(panic_idx);
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
        let n = 1_000;
        let pool = ThreadPool::new(2.try_into().unwrap());
        let (tx, rx) = mpsc::channel();
        for i in 1..=n {
            let tx = tx.clone();
            pool.submit(move || {
                let _ = tx.send(i * 2);
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
