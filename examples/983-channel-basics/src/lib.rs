#![allow(clippy::all)]
// 983: MPSC Channel Basics
// Rust: std::sync::mpsc — Multiple Producer, Single Consumer

use std::sync::mpsc;
use std::thread;

// --- Approach 1: Single producer, single consumer ---
fn single_producer_consumer() -> Vec<i32> {
    let (tx, rx) = mpsc::channel::<i32>();

    let producer = thread::spawn(move || {
        for i in 1..=5 {
            tx.send(i).unwrap();
        }
        // tx drops here — channel closes
    });

    // Collect until channel is closed
    let results: Vec<i32> = rx.iter().collect();
    producer.join().unwrap();
    results
}

// --- Approach 2: Multiple producers (clone the sender) ---
fn multi_producer_consumer() -> Vec<i32> {
    let (tx, rx) = mpsc::channel::<i32>();

    let handles: Vec<_> = (0..3)
        .map(|batch| {
            let tx = tx.clone(); // each producer gets its own sender
            thread::spawn(move || {
                let start = batch * 10 + 1;
                for i in start..=start + 2 {
                    tx.send(i).unwrap();
                }
                // tx drops when thread exits
            })
        })
        .collect();

    drop(tx); // drop original so channel closes when all clones drop

    let mut results: Vec<i32> = rx.iter().collect();
    for h in handles {
        h.join().unwrap();
    }
    results.sort();
    results
}

// --- Approach 3: Producer sends typed messages ---
#[derive(Debug, PartialEq)]
enum WorkItem {
    Task(String),
    Done,
}

fn typed_channel() -> Vec<String> {
    let (tx, rx) = mpsc::channel::<WorkItem>();

    let producer = thread::spawn(move || {
        for name in ["alpha", "beta", "gamma"] {
            tx.send(WorkItem::Task(name.to_string())).unwrap();
        }
        tx.send(WorkItem::Done).unwrap();
    });

    let mut results = Vec::new();
    loop {
        match rx.recv().unwrap() {
            WorkItem::Task(s) => results.push(s),
            WorkItem::Done => break,
        }
    }
    producer.join().unwrap();
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_producer() {
        assert_eq!(single_producer_consumer(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_multi_producer() {
        let results = multi_producer_consumer();
        assert_eq!(results.len(), 9);
        // Contains items from all 3 batches
        assert!(results.contains(&1));
        assert!(results.contains(&11));
        assert!(results.contains(&21));
    }

    #[test]
    fn test_typed_channel() {
        let results = typed_channel();
        assert_eq!(results, vec!["alpha", "beta", "gamma"]);
    }

    #[test]
    fn test_channel_closes_on_drop() {
        let (tx, rx) = mpsc::channel::<i32>();
        drop(tx); // immediately close
        assert!(rx.recv().is_err()); // disconnected
    }

    #[test]
    fn test_recv_blocks_until_send() {
        let (tx, rx) = mpsc::channel();
        let h = thread::spawn(move || {
            thread::sleep(std::time::Duration::from_millis(1));
            tx.send(42).unwrap();
        });
        assert_eq!(rx.recv().unwrap(), 42);
        h.join().unwrap();
    }
}
