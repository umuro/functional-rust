#![allow(clippy::all)]
// 983: MPSC Channel Basics — Tokio version
// tokio::sync::mpsc — async channels with backpressure

use tokio::sync::mpsc;

/// Single producer, single consumer
async fn single_producer_consumer() -> Vec<i32> {
    let (tx, mut rx) = mpsc::channel::<i32>(32);

    tokio::spawn(async move {
        for i in 1..=5 {
            tx.send(i).await.unwrap();
        }
        // tx drops here — channel closes
    });

    let mut results = Vec::new();
    while let Some(v) = rx.recv().await {
        results.push(v);
    }
    results
}

/// Multiple producers (clone the sender)
async fn multi_producer_consumer() -> Vec<i32> {
    let (tx, mut rx) = mpsc::channel::<i32>(32);

    for batch in 0..3 {
        let tx = tx.clone();
        tokio::spawn(async move {
            let start = batch * 10 + 1;
            for i in start..=start + 2 {
                tx.send(i).await.unwrap();
            }
        });
    }

    drop(tx); // drop original so channel closes when all clones drop

    let mut results = Vec::new();
    while let Some(v) = rx.recv().await {
        results.push(v);
    }
    results.sort();
    results
}

/// Typed messages
#[derive(Debug, PartialEq)]
enum WorkItem {
    Task(String),
    Done,
}

async fn typed_channel() -> Vec<String> {
    let (tx, mut rx) = mpsc::channel::<WorkItem>(32);

    tokio::spawn(async move {
        for name in ["alpha", "beta", "gamma"] {
            tx.send(WorkItem::Task(name.to_string())).await.unwrap();
        }
        tx.send(WorkItem::Done).await.unwrap();
    });

    let mut results = Vec::new();
    while let Some(item) = rx.recv().await {
        match item {
            WorkItem::Task(s) => results.push(s),
            WorkItem::Done => break,
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_single_producer() {
        assert_eq!(single_producer_consumer().await, vec![1, 2, 3, 4, 5]);
    }

    #[tokio::test]
    async fn test_multi_producer() {
        let results = multi_producer_consumer().await;
        assert_eq!(results.len(), 9);
        assert!(results.contains(&1));
        assert!(results.contains(&11));
        assert!(results.contains(&21));
    }

    #[tokio::test]
    async fn test_typed_channel() {
        let results = typed_channel().await;
        assert_eq!(results, vec!["alpha", "beta", "gamma"]);
    }

    #[tokio::test]
    async fn test_channel_closes_on_drop() {
        let (tx, mut rx) = mpsc::channel::<i32>(1);
        drop(tx);
        assert!(rx.recv().await.is_none()); // closed
    }

    #[tokio::test]
    async fn test_bounded_channel() {
        // tokio mpsc is always bounded — backpressure built in
        let (tx, mut rx) = mpsc::channel::<i32>(2);
        tx.send(1).await.unwrap();
        tx.send(2).await.unwrap();
        // Channel is full — send would block (not tested here)
        assert_eq!(rx.recv().await, Some(1));
        assert_eq!(rx.recv().await, Some(2));
    }
}
