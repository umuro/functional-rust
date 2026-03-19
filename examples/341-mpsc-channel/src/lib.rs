#![allow(clippy::all)]
//! # MPSC Channel
//!
//! Multi-producer, single-consumer channel — the standard way to communicate between threads.

use std::sync::mpsc::{self, Sender};
use std::thread;

/// Creates a fan-in pattern: multiple producers, one consumer.
pub fn fan_in<T: Send + 'static>(producers: Vec<Box<dyn FnOnce(Sender<T>) + Send>>) -> Vec<T> {
    let (tx, rx) = mpsc::channel();

    for producer in producers {
        let tx = tx.clone();
        thread::spawn(move || producer(tx));
    }

    drop(tx); // Important: drop original so rx closes when all producers done
    rx.into_iter().collect()
}

/// Creates a bounded channel that applies backpressure.
pub fn bounded_producer_consumer(capacity: usize, items: Vec<i32>) -> Vec<i32> {
    let (tx, rx) = mpsc::sync_channel::<i32>(capacity);

    let producer = thread::spawn(move || {
        for item in items {
            tx.send(item).unwrap(); // Blocks if buffer full
        }
    });

    let results: Vec<_> = rx.into_iter().collect();
    producer.join().unwrap();
    results
}

/// Demonstrates multiple producers sending to one consumer.
pub fn multi_producer(num_producers: usize, messages_per_producer: usize) -> Vec<String> {
    let (tx, rx) = mpsc::channel();

    for i in 0..num_producers {
        let tx = tx.clone();
        thread::spawn(move || {
            for j in 0..messages_per_producer {
                tx.send(format!("producer-{}-msg-{}", i, j)).unwrap();
            }
        });
    }

    drop(tx);
    rx.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fan_in() {
        let producers: Vec<Box<dyn FnOnce(Sender<i32>) + Send>> = vec![
            Box::new(|tx| {
                tx.send(1).unwrap();
                tx.send(2).unwrap();
            }),
            Box::new(|tx| {
                tx.send(3).unwrap();
            }),
        ];
        let mut results = fan_in(producers);
        results.sort();
        assert_eq!(results, vec![1, 2, 3]);
    }

    #[test]
    fn test_bounded_channel() {
        let results = bounded_producer_consumer(2, vec![1, 2, 3, 4, 5]);
        assert_eq!(results, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_multi_producer() {
        let results = multi_producer(3, 2);
        assert_eq!(results.len(), 6);
    }

    #[test]
    fn test_channel_closes_when_senders_dropped() {
        let (tx, rx) = mpsc::channel::<i32>();
        drop(tx);
        assert!(rx.recv().is_err());
    }

    #[test]
    fn test_sync_channel_blocks() {
        let (tx, rx) = mpsc::sync_channel::<i32>(1);
        tx.send(1).unwrap();
        // Next send would block if we didn't receive
        assert_eq!(rx.recv().unwrap(), 1);
    }
}
