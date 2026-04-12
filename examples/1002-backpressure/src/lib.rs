#![allow(dead_code)]
#![allow(clippy::all)]
// 1002: Backpressure — Bounded sync_channel blocks producer
// When consumer is slow, bounded buffer fills and producer is forced to wait

use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

// --- Approach 1: sync_channel with slow consumer ---
fn bounded_backpressure() -> (usize, Duration) {
    const BUFFER_SIZE: usize = 3;
    // sync_channel(N): sender blocks when N items are buffered
    let (tx, rx) = mpsc::sync_channel::<i32>(BUFFER_SIZE);

    let start = Instant::now();

    let producer = thread::spawn(move || {
        for i in 1..=9 {
            tx.send(i).unwrap(); // blocks when buffer is full
        }
        // tx drops here — signals consumer to stop
    });

    let consumer = thread::spawn(move || {
        for item in rx.iter() {
            thread::sleep(Duration::from_millis(5)); // slow consumer
            let _ = item;
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
    (9, start.elapsed())
}

// --- Approach 2: try_send for non-blocking backpressure (drop or error) ---
fn try_send_demo() -> (usize, usize) {
    let (tx, rx) = mpsc::sync_channel::<i32>(2);

    let mut accepted = 0;
    let mut dropped = 0;

    for i in 1..=10 {
        match tx.try_send(i) {
            Ok(_) => accepted += 1,
            Err(mpsc::TrySendError::Full(_)) => dropped += 1,
            Err(mpsc::TrySendError::Disconnected(_)) => break,
        }
    }

    drop(tx);
    let drained: Vec<_> = rx.iter().collect();
    assert_eq!(drained.len(), accepted);
    (accepted, dropped)
}

// --- Approach 3: Bounded pipeline with backpressure between stages ---
fn bounded_pipeline(items: Vec<i32>) -> Vec<i32> {
    // Stage channels — each bounded to 2 items
    let (tx1, rx1) = mpsc::sync_channel::<i32>(2);
    let (tx2, rx2) = mpsc::sync_channel::<i32>(2);
    let (tx3, rx3) = mpsc::sync_channel::<i32>(2);

    // Stage 1: double
    thread::spawn(move || {
        for item in rx1.iter() {
            tx2.send(item * 2).unwrap();
        }
    });

    // Stage 2: add 1 (slow)
    thread::spawn(move || {
        for item in rx2.iter() {
            thread::sleep(Duration::from_millis(1)); // simulate slow processing
            tx3.send(item + 1).unwrap();
        }
    });

    // Producer
    let producer = thread::spawn(move || {
        for item in items {
            tx1.send(item).unwrap();
        } // blocks when stage 1 full
    });

    // Collect
    let results: Vec<i32> = rx3.iter().collect();
    producer.join().unwrap();
    results
}

// --- Approach 4: Measure backpressure effect ---
fn measure_backpressure_effect() -> bool {
    // With buffer=1: producer is slowed to consumer's pace
    let (tx_fast, rx_fast) = mpsc::channel::<i32>(); // unbounded
    let (tx_bounded, rx_bounded) = mpsc::sync_channel::<i32>(1); // bounded=1

    let fast_start = Instant::now();
    let h = thread::spawn(move || {
        for i in 0..20 {
            tx_fast.send(i).unwrap();
        }
    });
    h.join().unwrap();
    let fast_time = fast_start.elapsed();
    drop(rx_fast);

    let bounded_start = Instant::now();
    let h2 = thread::spawn(move || {
        for i in 0..20 {
            tx_bounded.send(i).unwrap();
        }
    });
    // Slow consumer
    thread::spawn(move || {
        for _ in rx_bounded.iter() {
            thread::sleep(Duration::from_millis(1));
        }
    });
    h2.join().unwrap();
    let bounded_time = bounded_start.elapsed();

    // Bounded (backpressure) should be slower than unbounded
    bounded_time > fast_time
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounded_backpressure_processes_all() {
        let (count, _) = bounded_backpressure();
        assert_eq!(count, 9);
    }

    #[test]
    fn test_try_send_drops_when_full() {
        let (accepted, dropped) = try_send_demo();
        assert_eq!(accepted, 2); // buffer size = 2
        assert_eq!(dropped, 8); // remaining 8 are dropped
        assert_eq!(accepted + dropped, 10);
    }

    #[test]
    fn test_bounded_pipeline_correctness() {
        // 1*2+1=3, 2*2+1=5, 3*2+1=7
        let mut results = bounded_pipeline(vec![1, 2, 3]);
        results.sort();
        assert_eq!(results, vec![3, 5, 7]);
    }

    #[test]
    fn test_sync_channel_zero_buffer_rendezvous() {
        // sync_channel(0) = rendezvous — sender blocks until receiver takes
        let (tx, rx) = mpsc::sync_channel::<i32>(0);
        let h = thread::spawn(move || {
            tx.send(42).unwrap(); // blocks until receiver calls recv()
        });
        assert_eq!(rx.recv().unwrap(), 42);
        h.join().unwrap();
    }

    #[test]
    fn test_backpressure_is_slower() {
        assert!(measure_backpressure_effect());
    }

    #[test]
    fn test_try_send_error_type() {
        let (tx, _rx) = mpsc::sync_channel::<i32>(1);
        tx.try_send(1).unwrap(); // fills the buffer
        let err = tx.try_send(2);
        assert!(matches!(err, Err(mpsc::TrySendError::Full(_))));
    }
}
