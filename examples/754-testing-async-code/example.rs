/// 754: Testing Async Code — std::thread as async analog
///
/// In real async Rust, use #[tokio::test]:
/// ```ignore
/// #[tokio::test]
/// async fn test_my_handler() {
///     let result = my_async_fn().await;
///     assert_eq!(result, expected);
/// }
/// ```
///
/// This example shows the structural pattern using threads.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// ── "Async" service modelled with threads ─────────────────────────────────────

#[derive(Debug)]
pub struct Request {
    pub id:   u64,
    pub body: String,
}

#[derive(Debug, PartialEq)]
pub struct Response {
    pub id:     u64,
    pub result: String,
}

/// A worker that processes requests — simulates an async handler.
pub struct Worker {
    tx:     mpsc::Sender<Request>,
    rx:     mpsc::Receiver<Response>,
    handle: thread::JoinHandle<()>,
}

impl Worker {
    pub fn start() -> Self {
        let (req_tx, req_rx) = mpsc::channel::<Request>();
        let (res_tx, res_rx) = mpsc::channel::<Response>();

        let handle = thread::spawn(move || {
            while let Ok(req) = req_rx.recv() {
                // Simulate async processing
                thread::sleep(Duration::from_millis(1));
                let result = format!("processed:{}", req.body.to_uppercase());
                let _ = res_tx.send(Response { id: req.id, result });
            }
        });

        Worker { tx: req_tx, rx: res_rx, handle }
    }

    pub fn send(&self, req: Request) {
        self.tx.send(req).expect("worker channel closed");
    }

    pub fn recv_timeout(&self, timeout: Duration) -> Option<Response> {
        self.rx.recv_timeout(timeout).ok()
    }

    pub fn shutdown(self) {
        drop(self.tx);  // close channel → worker exits loop
        self.handle.join().expect("worker panicked");
    }
}

// ── Rate limiter with background cleanup (another async analog) ───────────────

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub struct RateLimiter {
    counts: Arc<Mutex<HashMap<String, u32>>>,
    limit:  u32,
}

impl RateLimiter {
    pub fn new(limit: u32) -> Self {
        let counts = Arc::new(Mutex::new(HashMap::new()));
        let counts2 = Arc::clone(&counts);

        // Background cleaner — simulates async periodic task
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(100));
                let mut guard = counts2.lock().unwrap();
                if guard.is_empty() { break; }
                guard.clear();
            }
        });

        RateLimiter { counts, limit }
    }

    pub fn allow(&self, key: &str) -> bool {
        let mut guard = self.counts.lock().unwrap();
        let count = guard.entry(key.to_owned()).or_insert(0);
        if *count < self.limit {
            *count += 1;
            true
        } else {
            false
        }
    }
}

fn main() {
    let worker = Worker::start();

    for i in 0..5u64 {
        worker.send(Request { id: i, body: format!("msg-{}", i) });
    }

    let timeout = Duration::from_secs(2);
    for _ in 0..5 {
        if let Some(resp) = worker.recv_timeout(timeout) {
            println!("Response {}: {}", resp.id, resp.result);
        }
    }

    worker.shutdown();
    println!("Worker shutdown cleanly.");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn timeout() -> Duration { Duration::from_secs(2) }

    #[test]
    fn worker_processes_single_request() {
        let worker = Worker::start();
        worker.send(Request { id: 1, body: "hello".into() });
        let resp = worker.recv_timeout(timeout()).expect("timed out");
        assert_eq!(resp.id, 1);
        assert_eq!(resp.result, "processed:HELLO");
        worker.shutdown();
    }

    #[test]
    fn worker_processes_multiple_requests() {
        let worker = Worker::start();
        let n = 10u64;
        for i in 0..n {
            worker.send(Request { id: i, body: format!("item-{}", i) });
        }
        let mut ids: Vec<u64> = (0..n)
            .filter_map(|_| worker.recv_timeout(timeout()))
            .map(|r| r.id)
            .collect();
        ids.sort();
        assert_eq!(ids.len(), n as usize);
        worker.shutdown();
    }

    #[test]
    fn worker_result_contains_uppercase_body() {
        let worker = Worker::start();
        worker.send(Request { id: 99, body: "rust".into() });
        let resp = worker.recv_timeout(timeout()).unwrap();
        assert!(resp.result.contains("RUST"), "got: {}", resp.result);
        worker.shutdown();
    }

    #[test]
    fn rate_limiter_allows_up_to_limit() {
        let rl = RateLimiter::new(3);
        assert!(rl.allow("user:1"));
        assert!(rl.allow("user:1"));
        assert!(rl.allow("user:1"));
        assert!(!rl.allow("user:1"));  // 4th rejected
    }

    #[test]
    fn rate_limiter_independent_per_key() {
        let rl = RateLimiter::new(2);
        assert!(rl.allow("a"));
        assert!(rl.allow("a"));
        assert!(!rl.allow("a"));       // a exhausted
        assert!(rl.allow("b"));        // b independent
        assert!(rl.allow("b"));
    }
}
