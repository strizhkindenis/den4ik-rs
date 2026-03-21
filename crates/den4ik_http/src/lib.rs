#![deny(
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::pedantic,
    clippy::cargo,
)]

use std::{
    array,
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, SendError, Sender},
    },
    thread::{self, JoinHandle},
    time::Duration,
};

const SLEEP_MS: Duration = Duration::from_millis(10);

struct ThreadPool<F, T, const N: usize>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static
{
    threads: [JoinHandle<()>; N],
    tx: Sender<F>,
}

fn start_worker<F, T>(
    rx: Arc<Mutex<Receiver<F>>>,
) -> JoinHandle<()>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static
{
    thread::spawn(move || {
        use std::sync::{PoisonError, mpsc::TryRecvError};
        loop {
            let Ok(rx) = rx.lock() else {
                break;    
            };
            let Ok(job) = rx.recv() else {
                break;
            };
            drop(rx);
            job();
        }
    })
}

impl<F, T, const N: usize> ThreadPool<F, T, N>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static
{
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        let threads = array::from_fn(|_| {
            start_worker(rx.clone())
        });
        Self { threads, tx }
    }

    pub fn submit(&self, job: F) {
        let _ = self.tx.send(job);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_thread_pool() {
        let n = 10;
        let pool = ThreadPool::<_, _, 4>::new();        
        let (tx, rx) = mpsc::channel();
        for i in 1..=n {
            let tx = tx.clone();
            pool.submit(move || {
                thread::sleep(Duration::from_millis(10));
                tx.send(i * 2).unwrap();
            });
        }
        let result_pool = rx.iter().sum::<u64>();
        let result_check = (1..=n).map(|x| x * 2).sum::<u64>();
        assert_eq!(result_check, result_pool);
    }   
}
