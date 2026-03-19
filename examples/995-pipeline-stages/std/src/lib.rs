#![allow(clippy::all)]
// 995: N-Stage Streaming Pipeline
// Each stage is a thread + channel — filter/map/transform stages

use std::sync::mpsc;
use std::thread;

// --- Map stage: applies f to each item ---
fn map_stage<T, U, F>(rx: mpsc::Receiver<T>, f: F) -> mpsc::Receiver<U>
where
    T: Send + 'static,
    U: Send + 'static,
    F: Fn(T) -> U + Send + 'static,
{
    let (tx, out) = mpsc::channel();
    thread::spawn(move || {
        for item in rx.iter() {
            tx.send(f(item)).unwrap();
        }
    });
    out
}

// --- Filter stage: only forward items where pred is true ---
fn filter_stage<T, F>(rx: mpsc::Receiver<T>, pred: F) -> mpsc::Receiver<T>
where
    T: Send + 'static,
    F: Fn(&T) -> bool + Send + 'static,
{
    let (tx, out) = mpsc::channel();
    thread::spawn(move || {
        for item in rx.iter() {
            if pred(&item) {
                tx.send(item).unwrap();
            }
        }
    });
    out
}

// --- Flat-map stage: one item → multiple outputs ---
fn flat_map_stage<T, U, F>(rx: mpsc::Receiver<T>, f: F) -> mpsc::Receiver<U>
where
    T: Send + 'static,
    U: Send + 'static,
    F: Fn(T) -> Vec<U> + Send + 'static,
{
    let (tx, out) = mpsc::channel();
    thread::spawn(move || {
        for item in rx.iter() {
            for v in f(item) {
                tx.send(v).unwrap();
            }
        }
    });
    out
}

// --- Build a multi-stage pipeline ---
fn pipeline_even_squares() -> Vec<String> {
    let (tx, rx) = mpsc::channel::<i32>();

    // Stage 1: square
    let rx1 = map_stage(rx, |x| x * x);
    // Stage 2: keep even
    let rx2 = filter_stage(rx1, |x| x % 2 == 0);
    // Stage 3: to string
    let rx3 = map_stage(rx2, |x: i32| x.to_string());

    // Producer
    let h = thread::spawn(move || {
        for i in 1..=10 { tx.send(i).unwrap(); }
    });

    let results: Vec<String> = rx3.iter().collect();
    h.join().unwrap();
    results
}

// --- More complex: tokenize → filter stop words → count ---
fn word_count_pipeline(text: &str) -> usize {
    let stop_words = vec!["the", "a", "an", "is", "in", "of", "to"];
    let words: Vec<String> = text.split_whitespace()
        .map(|w| w.to_lowercase())
        .collect();

    let (tx, rx) = mpsc::channel::<String>();

    // Stage 1: emit each word
    let rx1 = filter_stage(rx, move |w: &String| !stop_words.contains(&w.as_str()));
    // Stage 2: remove empty
    let rx2 = filter_stage(rx1, |w: &String| !w.is_empty());
    // Stage 3: get length (to count)
    let rx3 = map_stage(rx2, |_: String| 1usize);

    let h = thread::spawn(move || {
        for w in words { tx.send(w).unwrap(); }
    });

    let count: usize = rx3.iter().sum();
    h.join().unwrap();
    count
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_even_squares() {
        let results = pipeline_even_squares();
        // Squares of 1..10: 1,4,9,16,25,36,49,64,81,100
        // Even: 4,16,36,64,100
        assert_eq!(results, vec!["4", "16", "36", "64", "100"]);
    }

    #[test]
    fn test_map_stage() {
        let (tx, rx) = mpsc::channel::<i32>();
        let out = map_stage(rx, |x| x * 2);
        for i in [1, 2, 3] { tx.send(i).unwrap(); }
        drop(tx);
        let results: Vec<i32> = out.iter().collect();
        assert_eq!(results, vec![2, 4, 6]);
    }

    #[test]
    fn test_filter_stage() {
        let (tx, rx) = mpsc::channel::<i32>();
        let out = filter_stage(rx, |x| x % 2 == 0);
        for i in 1..=6 { tx.send(i).unwrap(); }
        drop(tx);
        let results: Vec<i32> = out.iter().collect();
        assert_eq!(results, vec![2, 4, 6]);
    }

    #[test]
    fn test_flat_map_stage() {
        let (tx, rx) = mpsc::channel::<i32>();
        let out = flat_map_stage(rx, |x| vec![x, x * 10]);
        for i in [1, 2, 3] { tx.send(i).unwrap(); }
        drop(tx);
        let results: Vec<i32> = out.iter().collect();
        assert_eq!(results, vec![1, 10, 2, 20, 3, 30]);
    }

    #[test]
    fn test_word_count_pipeline() {
        let count = word_count_pipeline("the quick brown fox jumps over the lazy dog");
        // 9 words - stop words: the(x2), over -> 6 content words
        assert!(count > 0 && count < 9);
    }
}
