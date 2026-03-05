use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
enum TimeoutError { Elapsed, TaskFailed(String) }

impl std::fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Elapsed => write!(f, "operation timed out"),
            Self::TaskFailed(e) => write!(f, "task failed: {e}"),
        }
    }
}

fn with_timeout<T: Send + 'static>(timeout: Duration, f: impl FnOnce()->Result<T,String>+Send+'static) -> Result<T, TimeoutError> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || { let _ = tx.send(f()); });
    match rx.recv_timeout(timeout) {
        Ok(Ok(v)) => Ok(v),
        Ok(Err(e)) => Err(TimeoutError::TaskFailed(e)),
        Err(mpsc::RecvTimeoutError::Timeout) => Err(TimeoutError::Elapsed),
        Err(mpsc::RecvTimeoutError::Disconnected) => Err(TimeoutError::TaskFailed("disconnected".into())),
    }
}

fn slow(delay_ms: u64, val: i32) -> Result<i32, String> {
    thread::sleep(Duration::from_millis(delay_ms)); Ok(val)
}

fn main() {
    println!("Fast: {:?}", with_timeout(Duration::from_millis(100), || slow(20, 42)));
    println!("Slow: {:?}", with_timeout(Duration::from_millis(30), || slow(200, 0)));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn succeeds() { assert_eq!(with_timeout(Duration::from_millis(200), || slow(10,42)).unwrap(), 42); }
    #[test] fn times_out() { assert!(matches!(with_timeout(Duration::from_millis(10), || slow(500,0)), Err(TimeoutError::Elapsed))); }
}
