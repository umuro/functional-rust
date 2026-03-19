// 996: Timeout Pattern
// Rust: mpsc::recv_timeout — like OCaml's Lwt.pick with sleep

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// --- Approach 1: recv_timeout on a channel ---
fn channel_with_timeout(delay_ms: u64, timeout_ms: u64) -> Result<i32, &'static str> {
    let (tx, rx) = mpsc::channel::<i32>();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(delay_ms));
        tx.send(42).ok(); // may fail if receiver timed out and was dropped
    });

    match rx.recv_timeout(Duration::from_millis(timeout_ms)) {
        Ok(v) => Ok(v),
        Err(mpsc::RecvTimeoutError::Timeout) => Err("timeout"),
        Err(mpsc::RecvTimeoutError::Disconnected) => Err("disconnected"),
    }
}

// --- Approach 2: Run any function with a timeout via thread ---
fn with_timeout<T, F>(timeout: Duration, f: F) -> Option<T>
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    let (tx, rx) = mpsc::channel::<T>();
    thread::spawn(move || {
        let result = f();
        tx.send(result).ok();
    });
    rx.recv_timeout(timeout).ok()
}

// --- Approach 3: First-of-N wins (Lwt.pick analogue) ---
fn race<T: Send + 'static>(
    tasks: Vec<Box<dyn FnOnce() -> T + Send + 'static>>,
    timeout: Duration,
) -> Option<T> {
    let (tx, rx) = mpsc::channel::<T>();

    for task in tasks {
        let tx = tx.clone();
        thread::spawn(move || {
            let result = task();
            tx.send(result).ok(); // first to arrive wins
        });
    }
    drop(tx); // close original sender

    rx.recv_timeout(timeout).ok()
}

// --- Approach 4: Retry with overall deadline ---
fn retry_with_deadline<T, E, F>(
    max_attempts: usize,
    timeout_per_attempt: Duration,
    f: F,
) -> Result<T, &'static str>
where
    T: Send + 'static,
    E: Send + 'static,
    F: Fn() -> Result<T, E> + Send + Sync + Clone + 'static,
{
    for attempt in 0..max_attempts {
        let f = f.clone();
        let result = with_timeout(timeout_per_attempt, move || f());
        match result {
            Some(Ok(v)) => return Ok(v),
            Some(Err(_)) | None => {
                if attempt + 1 < max_attempts {
                    thread::sleep(Duration::from_millis(1 << attempt));
                }
            }
        }
    }
    Err("max attempts exceeded")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recv_before_timeout() {
        let result = channel_with_timeout(10, 500);
        assert_eq!(result, Ok(42));
    }

    #[test]
    fn test_recv_after_timeout() {
        let result = channel_with_timeout(200, 20);
        assert_eq!(result, Err("timeout"));
    }

    #[test]
    fn test_with_timeout_succeeds() {
        let result = with_timeout(Duration::from_millis(500), || {
            thread::sleep(Duration::from_millis(5));
            99i32
        });
        assert_eq!(result, Some(99));
    }

    #[test]
    fn test_with_timeout_expires() {
        let result = with_timeout(Duration::from_millis(5), || {
            thread::sleep(Duration::from_millis(100));
            99i32
        });
        assert_eq!(result, None);
    }

    #[test]
    fn test_race_fastest_wins() {
        let tasks: Vec<Box<dyn FnOnce() -> u32 + Send + 'static>> = vec![
            Box::new(|| {
                thread::sleep(Duration::from_millis(50));
                1
            }),
            Box::new(|| {
                thread::sleep(Duration::from_millis(5));
                2
            }),
            Box::new(|| {
                thread::sleep(Duration::from_millis(30));
                3
            }),
        ];
        let winner = race(tasks, Duration::from_millis(200));
        assert_eq!(winner, Some(2)); // fastest thread wins
    }

    #[test]
    fn test_recv_timeout_error_types() {
        let (_, rx) = mpsc::channel::<i32>();
        // Disconnected immediately (no sender)
        let err = rx.recv_timeout(Duration::from_millis(1));
        assert!(err.is_err());
    }
}
