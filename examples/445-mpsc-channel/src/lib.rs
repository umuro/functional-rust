//! # MPSC Channels — Message Passing Between Threads
//!
//! Send values across threads with `std::sync::mpsc` — multiple producers,
//! one consumer, with automatic shutdown when all senders drop.

use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

/// Approach 1: Multiple producers, single consumer
pub fn multi_producer_single_consumer(num_producers: usize, msgs_per_producer: usize) -> Vec<String> {
    let (tx, rx) = mpsc::channel::<String>();

    let handles: Vec<_> = (0..num_producers)
        .map(|id| {
            let tx = tx.clone();
            thread::spawn(move || {
                for i in 0..msgs_per_producer {
                    tx.send(format!("p{}-msg{}", id, i)).unwrap();
                }
            })
        })
        .collect();

    drop(tx); // Important: drop original sender

    // Collect all messages
    let messages: Vec<String> = rx.iter().collect();

    for h in handles {
        h.join().unwrap();
    }

    messages
}

/// Approach 2: Bounded channel using sync_channel
pub fn bounded_channel_demo(buffer_size: usize, num_msgs: usize) -> Vec<i32> {
    let (tx, rx) = mpsc::sync_channel::<i32>(buffer_size);

    let producer = thread::spawn(move || {
        for i in 0..num_msgs as i32 {
            tx.send(i).unwrap();
        }
    });

    let consumer = thread::spawn(move || {
        let mut results = Vec::new();
        for msg in rx {
            results.push(msg);
        }
        results
    });

    producer.join().unwrap();
    consumer.join().unwrap()
}

/// Approach 3: Non-blocking try_recv and try_iter
pub fn non_blocking_receive(msgs: Vec<i32>) -> Vec<i32> {
    let (tx, rx) = mpsc::channel();

    for msg in msgs {
        tx.send(msg).unwrap();
    }
    drop(tx);

    // Non-blocking collect
    rx.try_iter().collect()
}

/// Approach 4: Timeout-based receive
pub fn receive_with_timeout(timeout_ms: u64) -> Option<i32> {
    let (tx, rx) = mpsc::channel();

    let sender = thread::spawn(move || {
        thread::sleep(Duration::from_millis(timeout_ms * 2));
        let _ = tx.send(42);
    });

    let result = rx.recv_timeout(Duration::from_millis(timeout_ms)).ok();

    sender.join().unwrap();
    result
}

/// Producer-consumer pattern with work items
pub struct WorkQueue<T> {
    sender: Sender<T>,
}

impl<T> WorkQueue<T> {
    pub fn new() -> (Self, Receiver<T>) {
        let (sender, receiver) = mpsc::channel();
        (Self { sender }, receiver)
    }

    pub fn send(&self, item: T) -> Result<(), mpsc::SendError<T>> {
        self.sender.send(item)
    }

    pub fn clone_sender(&self) -> Sender<T> {
        self.sender.clone()
    }
}

impl<T> Default for WorkQueue<T> {
    fn default() -> Self {
        Self::new().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_recv_basic() {
        let (tx, rx) = mpsc::channel();
        tx.send(42u32).unwrap();
        assert_eq!(rx.recv().unwrap(), 42);
    }

    #[test]
    fn test_channel_closed() {
        let (tx, rx) = mpsc::channel::<i32>();
        drop(tx);
        assert!(rx.recv().is_err());
    }

    #[test]
    fn test_multiple_producers() {
        let (tx, rx) = mpsc::channel::<u32>();
        let handles: Vec<_> = (0..4)
            .map(|i| {
                let tx = tx.clone();
                thread::spawn(move || tx.send(i).unwrap())
            })
            .collect();

        drop(tx);

        let mut results: Vec<u32> = rx.iter().collect();
        results.sort();
        assert_eq!(results, vec![0, 1, 2, 3]);

        for h in handles {
            h.join().unwrap();
        }
    }

    #[test]
    fn test_multi_producer_consumer() {
        let messages = multi_producer_single_consumer(3, 5);
        assert_eq!(messages.len(), 15);
    }

    #[test]
    fn test_bounded_channel() {
        let results = bounded_channel_demo(2, 10);
        assert_eq!(results, (0..10).collect::<Vec<i32>>());
    }

    #[test]
    fn test_non_blocking() {
        let input = vec![1, 2, 3, 4, 5];
        let output = non_blocking_receive(input.clone());
        assert_eq!(output, input);
    }

    #[test]
    fn test_try_recv_empty() {
        let (_tx, rx) = mpsc::channel::<i32>();
        assert!(rx.try_recv().is_err());
    }

    #[test]
    fn test_recv_timeout() {
        let result = receive_with_timeout(10);
        assert!(result.is_none()); // Timeout before message arrives
    }

    #[test]
    fn test_work_queue() {
        let (queue, rx) = WorkQueue::<i32>::new();

        queue.send(1).unwrap();
        queue.send(2).unwrap();
        queue.send(3).unwrap();

        let tx2 = queue.clone_sender();
        tx2.send(4).unwrap();

        drop(queue);
        drop(tx2);

        let results: Vec<i32> = rx.iter().collect();
        assert_eq!(results, vec![1, 2, 3, 4]);
    }
}
