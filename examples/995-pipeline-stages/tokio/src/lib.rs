// 995: N-Stage Streaming Pipeline — Tokio version
// Each stage is a tokio task + mpsc channel

use tokio::sync::mpsc;

/// Map stage: applies f to each item
fn map_stage<T, U, F>(mut rx: mpsc::Receiver<T>, f: F) -> mpsc::Receiver<U>
where
    T: Send + 'static,
    U: Send + 'static,
    F: Fn(T) -> U + Send + 'static,
{
    let (tx, out) = mpsc::channel(32);
    tokio::spawn(async move {
        while let Some(item) = rx.recv().await {
            if tx.send(f(item)).await.is_err() { break; }
        }
    });
    out
}

/// Filter stage: only forward items where pred is true
fn filter_stage<T, F>(mut rx: mpsc::Receiver<T>, pred: F) -> mpsc::Receiver<T>
where
    T: Send + 'static,
    F: Fn(&T) -> bool + Send + 'static,
{
    let (tx, out) = mpsc::channel(32);
    tokio::spawn(async move {
        while let Some(item) = rx.recv().await {
            if pred(&item) {
                if tx.send(item).await.is_err() { break; }
            }
        }
    });
    out
}

/// Flat-map stage: one item → multiple outputs
fn flat_map_stage<T, U, F>(mut rx: mpsc::Receiver<T>, f: F) -> mpsc::Receiver<U>
where
    T: Send + 'static,
    U: Send + 'static,
    F: Fn(T) -> Vec<U> + Send + 'static,
{
    let (tx, out) = mpsc::channel(32);
    tokio::spawn(async move {
        while let Some(item) = rx.recv().await {
            for v in f(item) {
                if tx.send(v).await.is_err() { return; }
            }
        }
    });
    out
}

/// Build a multi-stage pipeline
async fn pipeline_even_squares() -> Vec<String> {
    let (tx, rx) = mpsc::channel::<i32>(32);

    let rx1 = map_stage(rx, |x| x * x);
    let rx2 = filter_stage(rx1, |x| x % 2 == 0);
    let mut rx3 = map_stage(rx2, |x: i32| x.to_string());

    tokio::spawn(async move {
        for i in 1..=10 { tx.send(i).await.unwrap(); }
    });

    let mut results = Vec::new();
    while let Some(v) = rx3.recv().await {
        results.push(v);
    }
    results
}

/// Word count pipeline
async fn word_count_pipeline(text: &str) -> usize {
    let stop_words = vec!["the", "a", "an", "is", "in", "of", "to"];
    let words: Vec<String> = text.split_whitespace()
        .map(|w| w.to_lowercase())
        .collect();

    let (tx, rx) = mpsc::channel::<String>(32);
    let rx1 = filter_stage(rx, move |w: &String| !stop_words.contains(&w.as_str()));
    let rx2 = filter_stage(rx1, |w: &String| !w.is_empty());
    let mut rx3 = map_stage(rx2, |_: String| 1usize);

    tokio::spawn(async move {
        for w in words { tx.send(w).await.unwrap(); }
    });

    let mut count = 0;
    while let Some(v) = rx3.recv().await { count += v; }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pipeline_even_squares() {
        let results = pipeline_even_squares().await;
        assert_eq!(results, vec!["4", "16", "36", "64", "100"]);
    }

    #[tokio::test]
    async fn test_map_stage() {
        let (tx, rx) = mpsc::channel::<i32>(32);
        let mut out = map_stage(rx, |x| x * 2);
        tokio::spawn(async move {
            for i in [1, 2, 3] { tx.send(i).await.unwrap(); }
        });
        let mut results = Vec::new();
        while let Some(v) = out.recv().await { results.push(v); }
        assert_eq!(results, vec![2, 4, 6]);
    }

    #[tokio::test]
    async fn test_filter_stage() {
        let (tx, rx) = mpsc::channel::<i32>(32);
        let mut out = filter_stage(rx, |x| x % 2 == 0);
        tokio::spawn(async move {
            for i in 1..=6 { tx.send(i).await.unwrap(); }
        });
        let mut results = Vec::new();
        while let Some(v) = out.recv().await { results.push(v); }
        assert_eq!(results, vec![2, 4, 6]);
    }

    #[tokio::test]
    async fn test_flat_map_stage() {
        let (tx, rx) = mpsc::channel::<i32>(32);
        let mut out = flat_map_stage(rx, |x| vec![x, x * 10]);
        tokio::spawn(async move {
            for i in [1, 2, 3] { tx.send(i).await.unwrap(); }
        });
        let mut results = Vec::new();
        while let Some(v) = out.recv().await { results.push(v); }
        assert_eq!(results, vec![1, 10, 2, 20, 3, 30]);
    }

    #[tokio::test]
    async fn test_word_count_pipeline() {
        let count = word_count_pipeline("the quick brown fox jumps over the lazy dog").await;
        assert!(count > 0 && count < 9);
    }
}
