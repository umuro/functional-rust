#![allow(clippy::all)]
// 1002: Backpressure — Tokio version
// Bounded tokio::sync::mpsc channels provide natural backpressure

use tokio::sync::mpsc;
use std::time::{Duration, Instant};

/// Bounded channel with slow consumer — producer blocks when buffer full
async fn bounded_backpressure() -> (usize, Duration) {
    const BUFFER_SIZE: usize = 3;
    let (tx, mut rx) = mpsc::channel::<i32>(BUFFER_SIZE);

    let start = Instant::now();

    let producer = tokio::spawn(async move {
        for i in 1..=9 {
            tx.send(i).await.unwrap(); // blocks when buffer full
        }
    });

    let consumer = tokio::spawn(async move {
        while let Some(_item) = rx.recv().await {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    });

    producer.await.unwrap();
    consumer.await.unwrap();
    (9, start.elapsed())
}

/// try_send for non-blocking backpressure
async fn try_send_demo() -> (usize, usize) {
    let (tx, mut rx) = mpsc::channel::<i32>(2);

    let mut accepted = 0;
    let mut dropped = 0;

    for i in 1..=10 {
        match tx.try_send(i) {
            Ok(_) => accepted += 1,
            Err(mpsc::error::TrySendError::Full(_)) => dropped += 1,
            Err(mpsc::error::TrySendError::Closed(_)) => break,
        }
    }

    drop(tx);
    let mut drained = Vec::new();
    while let Some(v) = rx.recv().await { drained.push(v); }
    assert_eq!(drained.len(), accepted);
    (accepted, dropped)
}

/// Bounded pipeline with backpressure between stages
async fn bounded_pipeline(items: Vec<i32>) -> Vec<i32> {
    let (tx1, mut rx1) = mpsc::channel::<i32>(2);
    let (tx2, mut rx2) = mpsc::channel::<i32>(2);
    let (tx3, mut rx3) = mpsc::channel::<i32>(2);

    // Stage 1: double
    tokio::spawn(async move {
        while let Some(item) = rx1.recv().await {
            tx2.send(item * 2).await.unwrap();
        }
    });

    // Stage 2: add 1 (slow)
    tokio::spawn(async move {
        while let Some(item) = rx2.recv().await {
            tokio::time::sleep(Duration::from_millis(1)).await;
            tx3.send(item + 1).await.unwrap();
        }
    });

    // Producer
    tokio::spawn(async move {
        for item in items {
            tx1.send(item).await.unwrap();
        }
    });

    let mut results = Vec::new();
    while let Some(v) = rx3.recv().await {
        results.push(v);
    }
    results
}

/// Measure backpressure effect: bounded vs unbounded
async fn measure_backpressure_effect() -> bool {
    // Unbounded (large buffer)
    let (tx_fast, mut rx_fast) = mpsc::channel::<i32>(1000);
    let fast_start = Instant::now();
    let h1 = tokio::spawn(async move {
        for i in 0..20 { tx_fast.send(i).await.unwrap(); }
    });
    h1.await.unwrap();
    let fast_time = fast_start.elapsed();
    while rx_fast.recv().await.is_some() {}

    // Bounded (buffer=1) with slow consumer
    let (tx_bounded, mut rx_bounded) = mpsc::channel::<i32>(1);
    let bounded_start = Instant::now();
    let h2 = tokio::spawn(async move {
        for i in 0..20 { tx_bounded.send(i).await.unwrap(); }
    });
    tokio::spawn(async move {
        while let Some(_) = rx_bounded.recv().await {
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    });
    h2.await.unwrap();
    let bounded_time = bounded_start.elapsed();

    bounded_time > fast_time
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bounded_backpressure_processes_all() {
        let (count, _) = bounded_backpressure().await;
        assert_eq!(count, 9);
    }

    #[tokio::test]
    async fn test_try_send_drops_when_full() {
        let (accepted, dropped) = try_send_demo().await;
        assert_eq!(accepted, 2);
        assert_eq!(dropped, 8);
        assert_eq!(accepted + dropped, 10);
    }

    #[tokio::test]
    async fn test_bounded_pipeline_correctness() {
        let mut results = bounded_pipeline(vec![1, 2, 3]).await;
        results.sort();
        assert_eq!(results, vec![3, 5, 7]); // 1*2+1=3, 2*2+1=5, 3*2+1=7
    }

    #[tokio::test]
    async fn test_zero_buffer_rendezvous() {
        // mpsc channel with capacity 1 (min) — similar to rendezvous
        let (tx, mut rx) = mpsc::channel::<i32>(1);
        let h = tokio::spawn(async move {
            tx.send(42).await.unwrap();
        });
        assert_eq!(rx.recv().await.unwrap(), 42);
        h.await.unwrap();
    }

    #[tokio::test]
    async fn test_backpressure_is_slower() {
        assert!(measure_backpressure_effect().await);
    }

    #[tokio::test]
    async fn test_try_send_error_type() {
        let (tx, _rx) = mpsc::channel::<i32>(1);
        tx.try_send(1).unwrap();
        let err = tx.try_send(2);
        assert!(matches!(err, Err(mpsc::error::TrySendError::Full(_))));
    }
}
