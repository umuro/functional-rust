//! # Crossbeam Channels — Advanced Channel Patterns
//!
//! Demonstrates bounded channels and multi-consumer patterns
//! that crossbeam-channel provides, implemented with std.

use std::sync::mpsc::{self, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Approach 1: Bounded (sync) channel
///
/// Sender blocks when buffer is full, providing backpressure.
pub fn bounded_channel_demo(capacity: usize, num_messages: usize) -> Vec<u32> {
    let (tx, rx) = mpsc::sync_channel::<u32>(capacity);

    let producer = thread::spawn(move || {
        for i in 0..num_messages as u32 {
            tx.send(i).unwrap();
        }
    });

    let consumer = thread::spawn(move || {
        let mut received = Vec::new();
        for msg in rx {
            received.push(msg);
        }
        received
    });

    producer.join().unwrap();
    consumer.join().unwrap()
}

/// Approach 2: Multi-consumer pattern with Arc<Mutex<Receiver>>
pub fn multi_consumer_demo(num_consumers: usize, num_messages: usize) -> Vec<u32> {
    let (tx, rx) = mpsc::channel::<u32>();
    let rx = Arc::new(Mutex::new(rx));
    let results = Arc::new(Mutex::new(Vec::new()));

    let consumers: Vec<_> = (0..num_consumers)
        .map(|_id| {
            let rx = Arc::clone(&rx);
            let results = Arc::clone(&results);
            thread::spawn(move || loop {
                let msg = rx.lock().unwrap().recv();
                match msg {
                    Ok(v) => results.lock().unwrap().push(v),
                    Err(_) => break,
                }
            })
        })
        .collect();

    // Send messages
    for i in 0..num_messages as u32 {
        tx.send(i).unwrap();
    }
    drop(tx); // Close channel

    for c in consumers {
        c.join().unwrap();
    }

    let mut result = Arc::try_unwrap(results).unwrap().into_inner().unwrap();
    result.sort();
    result
}

/// Approach 3: try_send for non-blocking send
pub fn try_send_demo(capacity: usize) -> (usize, usize) {
    let (tx, rx) = mpsc::sync_channel::<i32>(capacity);
    let mut sent = 0;
    let mut failed = 0;

    for i in 0..capacity as i32 + 5 {
        match tx.try_send(i) {
            Ok(()) => sent += 1,
            Err(_) => failed += 1,
        }
    }

    // Drain to prevent deadlock
    drop(tx);
    for _ in rx {}

    (sent, failed)
}

/// Producer-consumer with timeout
pub fn recv_timeout_demo(timeout_ms: u64) -> Option<i32> {
    let (tx, rx) = mpsc::sync_channel::<i32>(1);

    // Delayed sender
    let _ = thread::spawn(move || {
        thread::sleep(Duration::from_millis(timeout_ms * 2));
        let _ = tx.send(42);
    });

    rx.recv_timeout(Duration::from_millis(timeout_ms)).ok()
}

/// Work distribution pattern
pub fn work_distribution(num_workers: usize, jobs: Vec<i32>) -> Vec<i32> {
    let (tx, rx) = mpsc::sync_channel::<i32>(num_workers * 2);
    let rx = Arc::new(Mutex::new(rx));
    let results = Arc::new(Mutex::new(Vec::new()));

    // Spawn workers
    let workers: Vec<_> = (0..num_workers)
        .map(|_| {
            let rx = Arc::clone(&rx);
            let results = Arc::clone(&results);
            thread::spawn(move || loop {
                match rx.lock().unwrap().recv() {
                    Ok(job) => {
                        let result = job * job; // Process job
                        results.lock().unwrap().push(result);
                    }
                    Err(_) => break,
                }
            })
        })
        .collect();

    // Send jobs
    for job in jobs {
        tx.send(job).unwrap();
    }
    drop(tx);

    // Wait for workers
    for w in workers {
        w.join().unwrap();
    }

    let mut r = Arc::try_unwrap(results).unwrap().into_inner().unwrap();
    r.sort();
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounded_channel() {
        let results = bounded_channel_demo(3, 10);
        assert_eq!(results, (0..10).collect::<Vec<_>>());
    }

    #[test]
    fn test_sync_channel_blocks_when_full() {
        let (tx, rx) = mpsc::sync_channel::<u32>(2);

        tx.send(1).unwrap();
        tx.send(2).unwrap();

        // Buffer full, try_send should fail
        assert!(tx.try_send(3).is_err());

        // After receiving, we can send again
        assert_eq!(rx.recv().unwrap(), 1);
        assert!(tx.try_send(3).is_ok());
    }

    #[test]
    fn test_multi_consumer() {
        let results = multi_consumer_demo(3, 9);
        assert_eq!(results, (0..9).collect::<Vec<_>>());
    }

    #[test]
    fn test_try_send() {
        let (sent, failed) = try_send_demo(3);
        assert_eq!(sent, 3);
        assert_eq!(failed, 5);
    }

    #[test]
    fn test_recv_timeout() {
        let result = recv_timeout_demo(10);
        assert!(result.is_none()); // Timeout before message
    }

    #[test]
    fn test_work_distribution() {
        let jobs: Vec<i32> = (1..=5).collect();
        let results = work_distribution(2, jobs);
        assert_eq!(results, vec![1, 4, 9, 16, 25]);
    }

    #[test]
    fn test_empty_work_distribution() {
        let results = work_distribution(2, vec![]);
        assert!(results.is_empty());
    }
}
