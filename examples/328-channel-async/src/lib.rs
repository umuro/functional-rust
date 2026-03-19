#![allow(clippy::all)]
//! # Async Channels (mpsc)
//!
//! Multi-producer, single-consumer channels let multiple tasks send messages
//! to one receiver — the safe, idiomatic way to communicate between concurrent workers.

use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

/// Creates a producer that sends labeled messages through a channel.
pub fn create_producer(
    tx: Sender<String>,
    label: &'static str,
    count: usize,
    delay_ms: u64,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        for i in 1..=count {
            thread::sleep(Duration::from_millis(delay_ms));
            tx.send(format!("{}-{}", label, i)).unwrap();
        }
    })
}

/// Collects all messages from multiple producers.
pub fn collect_messages(rx: Receiver<String>) -> Vec<String> {
    rx.into_iter().collect()
}

/// Creates a bounded channel simulation with a buffer.
pub struct BoundedChannel<T> {
    tx: Sender<T>,
    rx: Option<Receiver<T>>,
}

impl<T> BoundedChannel<T> {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Self { tx, rx: Some(rx) }
    }

    pub fn sender(&self) -> Sender<T> {
        self.tx.clone()
    }

    pub fn take_receiver(&mut self) -> Option<Receiver<T>> {
        self.rx.take()
    }
}

impl<T> Default for BoundedChannel<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Fan-in pattern: multiple sources, one collector.
pub fn fan_in<T: Send + 'static>(sources: Vec<Box<dyn FnOnce(Sender<T>) + Send>>) -> Vec<T> {
    let (tx, rx) = mpsc::channel();

    for source in sources {
        let tx = tx.clone();
        thread::spawn(move || source(tx));
    }

    drop(tx); // Drop original sender
    rx.into_iter().collect()
}

/// Worker pool pattern: distribute work, collect results.
pub fn worker_pool<T, R, F>(items: Vec<T>, workers: usize, f: F) -> Vec<R>
where
    T: Send + 'static,
    R: Send + 'static,
    F: Fn(T) -> R + Send + Sync + Clone + 'static,
{
    let (work_tx, work_rx) = mpsc::channel::<T>();
    let (result_tx, result_rx) = mpsc::channel::<R>();
    let work_rx = std::sync::Arc::new(std::sync::Mutex::new(work_rx));

    // Spawn workers
    let handles: Vec<_> = (0..workers)
        .map(|_| {
            let work_rx = work_rx.clone();
            let result_tx = result_tx.clone();
            let f = f.clone();
            thread::spawn(move || {
                while let Ok(item) = work_rx.lock().unwrap().recv() {
                    let _ = result_tx.send(f(item));
                }
            })
        })
        .collect();

    // Send work
    for item in items {
        work_tx.send(item).unwrap();
    }
    drop(work_tx);

    // Wait for workers to finish
    drop(result_tx);
    for h in handles {
        h.join().unwrap();
    }

    result_rx.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_producer_sends_all_messages() {
        let (tx, rx) = mpsc::channel();
        let handle = create_producer(tx, "test", 3, 1);
        handle.join().unwrap();
        let msgs: Vec<_> = rx.into_iter().collect();
        assert_eq!(msgs.len(), 3);
    }

    #[test]
    fn test_multiple_producers() {
        let (tx, rx) = mpsc::channel();
        let h1 = create_producer(tx.clone(), "A", 2, 1);
        let h2 = create_producer(tx, "B", 2, 1);
        h1.join().unwrap();
        h2.join().unwrap();
        let msgs: Vec<_> = rx.into_iter().collect();
        assert_eq!(msgs.len(), 4);
    }

    #[test]
    fn test_channel_closes_on_sender_drop() {
        let (tx, rx) = mpsc::channel::<i32>();
        drop(tx);
        assert!(rx.recv().is_err());
    }

    #[test]
    fn test_fan_in() {
        let sources: Vec<Box<dyn FnOnce(Sender<i32>) + Send>> = vec![
            Box::new(|tx| {
                tx.send(1).unwrap();
                tx.send(2).unwrap();
            }),
            Box::new(|tx| {
                tx.send(3).unwrap();
            }),
        ];
        let mut results = fan_in(sources);
        results.sort();
        assert_eq!(results, vec![1, 2, 3]);
    }

    #[test]
    fn test_worker_pool() {
        let items = vec![1, 2, 3, 4, 5];
        let mut results = worker_pool(items, 2, |x| x * 2);
        results.sort();
        assert_eq!(results, vec![2, 4, 6, 8, 10]);
    }
}
