// 1000: Reactive Stream — Tokio version
// Push-based async streams using tokio::sync::mpsc

use tokio::sync::mpsc;

/// Async Observable using mpsc channels
struct AsyncObservable<T> {
    subscribe_fn: Box<dyn Fn(mpsc::Sender<T>) -> tokio::task::JoinHandle<()> + Send + Sync>,
}

impl<T: Clone + Send + 'static> AsyncObservable<T> {
    fn from_iter(items: Vec<T>) -> Self {
        AsyncObservable {
            subscribe_fn: Box::new(move |tx| {
                let items = items.clone();
                tokio::spawn(async move {
                    for item in items {
                        if tx.send(item).await.is_err() { break; }
                    }
                })
            }),
        }
    }

    fn subscribe(&self, tx: mpsc::Sender<T>) -> tokio::task::JoinHandle<()> {
        (self.subscribe_fn)(tx)
    }
}

/// Collect all items from a channel-based stream
async fn collect_stream<T: Send + 'static>(mut rx: mpsc::Receiver<T>) -> Vec<T> {
    let mut results = Vec::new();
    while let Some(v) = rx.recv().await {
        results.push(v);
    }
    results
}

/// Map operator: transform stream items
async fn stream_map(input: Vec<i32>, f: impl Fn(i32) -> i32 + Send + 'static) -> Vec<i32> {
    let (tx_in, mut rx_in) = mpsc::channel(32);
    let (tx_out, rx_out) = mpsc::channel(32);

    // Producer
    tokio::spawn(async move {
        for item in input { tx_in.send(item).await.ok(); }
    });

    // Map stage
    tokio::spawn(async move {
        while let Some(v) = rx_in.recv().await {
            if tx_out.send(f(v)).await.is_err() { break; }
        }
    });

    collect_stream(rx_out).await
}

/// Filter operator
async fn stream_filter(input: Vec<i32>, pred: impl Fn(&i32) -> bool + Send + 'static) -> Vec<i32> {
    let (tx_in, mut rx_in) = mpsc::channel(32);
    let (tx_out, rx_out) = mpsc::channel(32);

    tokio::spawn(async move {
        for item in input { tx_in.send(item).await.ok(); }
    });

    tokio::spawn(async move {
        while let Some(v) = rx_in.recv().await {
            if pred(&v) {
                if tx_out.send(v).await.is_err() { break; }
            }
        }
    });

    collect_stream(rx_out).await
}

/// Take N items
async fn stream_take(input: Vec<i32>, n: usize) -> Vec<i32> {
    let (tx_in, mut rx_in) = mpsc::channel(32);
    let (tx_out, rx_out) = mpsc::channel(32);

    tokio::spawn(async move {
        for item in input { tx_in.send(item).await.ok(); }
    });

    tokio::spawn(async move {
        let mut count = 0;
        while let Some(v) = rx_in.recv().await {
            if count >= n { break; }
            if tx_out.send(v).await.is_err() { break; }
            count += 1;
        }
    });

    collect_stream(rx_out).await
}

/// Full reactive chain: filter even → map square → take 3
async fn reactive_chain() -> Vec<i32> {
    let input: Vec<i32> = (1..=10).collect();

    let (tx1, mut rx1) = mpsc::channel(32);
    let (tx2, mut rx2) = mpsc::channel(32);
    let (tx3, mut rx3) = mpsc::channel(32);
    let (tx4, rx4) = mpsc::channel(32);

    // Source
    tokio::spawn(async move {
        for i in input { tx1.send(i).await.ok(); }
    });

    // Filter even
    tokio::spawn(async move {
        while let Some(v) = rx1.recv().await {
            if v % 2 == 0 { tx2.send(v).await.ok(); }
        }
    });

    // Map square
    tokio::spawn(async move {
        while let Some(v) = rx2.recv().await {
            tx3.send(v * v).await.ok();
        }
    });

    // Take 3
    tokio::spawn(async move {
        let mut count = 0;
        while let Some(v) = rx3.recv().await {
            if count >= 3 { break; }
            tx4.send(v).await.ok();
            count += 1;
        }
    });

    collect_stream(rx4).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_from_iter() {
        let obs = AsyncObservable::from_iter(vec![1, 2, 3]);
        let (tx, rx) = mpsc::channel(32);
        obs.subscribe(tx).await.unwrap();
        assert_eq!(collect_stream(rx).await, vec![1, 2, 3]);
    }

    #[tokio::test]
    async fn test_stream_map() {
        let result = stream_map(vec![1, 2, 3], |x| x * 2).await;
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[tokio::test]
    async fn test_stream_filter() {
        let result = stream_filter(vec![1, 2, 3, 4, 5], |x| x % 2 == 0).await;
        assert_eq!(result, vec![2, 4]);
    }

    #[tokio::test]
    async fn test_stream_take() {
        let result = stream_take(vec![1, 2, 3, 4, 5], 3).await;
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[tokio::test]
    async fn test_reactive_chain() {
        let result = reactive_chain().await;
        assert_eq!(result, vec![4, 16, 36]);
    }

    #[tokio::test]
    async fn test_empty_stream() {
        let result = stream_map(vec![], |x| x * 2).await;
        assert!(result.is_empty());
    }
}
