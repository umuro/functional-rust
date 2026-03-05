//! # Crossbeam Select — Multiplexing Multiple Channels
//!
//! Wait on multiple channels simultaneously, receiving from whichever
//! is ready first. Implemented here with std polling to show the concept.

use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::thread;
use std::time::Duration;

/// Approach 1: Poll-based select with try_recv
pub fn poll_select<T, U>(
    rx1: &Receiver<T>,
    rx2: &Receiver<U>,
    timeout: Duration,
) -> SelectResult<T, U> {
    let start = std::time::Instant::now();
    let poll_interval = Duration::from_millis(1);

    loop {
        // Try first channel
        match rx1.try_recv() {
            Ok(v) => return SelectResult::First(v),
            Err(TryRecvError::Disconnected) => return SelectResult::Closed,
            Err(TryRecvError::Empty) => {}
        }

        // Try second channel
        match rx2.try_recv() {
            Ok(v) => return SelectResult::Second(v),
            Err(TryRecvError::Disconnected) => return SelectResult::Closed,
            Err(TryRecvError::Empty) => {}
        }

        // Check timeout
        if start.elapsed() >= timeout {
            return SelectResult::Timeout;
        }

        thread::sleep(poll_interval);
    }
}

#[derive(Debug, PartialEq)]
pub enum SelectResult<T, U> {
    First(T),
    Second(U),
    Timeout,
    Closed,
}

/// Approach 2: Priority select (prefer first channel)
pub fn priority_select<T: Clone>(
    primary: &Receiver<T>,
    secondary: &Receiver<T>,
) -> Option<(T, bool)> {
    // Always try primary first
    if let Ok(v) = primary.try_recv() {
        return Some((v, true));
    }

    if let Ok(v) = secondary.try_recv() {
        return Some((v, false));
    }

    None
}

/// Approach 3: Drain all available messages
pub fn drain_all<T>(receivers: &[Receiver<T>]) -> Vec<T> {
    let mut results = Vec::new();

    for rx in receivers {
        while let Ok(v) = rx.try_recv() {
            results.push(v);
        }
    }

    results
}

/// Approach 4: Select with stop signal
pub fn select_with_stop<T>(
    data_rx: Receiver<T>,
    stop_rx: Receiver<()>,
    timeout: Duration,
) -> Vec<T>
where
    T: Send + 'static,
{
    let mut results = Vec::new();
    let start = std::time::Instant::now();

    loop {
        // Check stop signal
        if stop_rx.try_recv().is_ok() {
            break;
        }

        // Check data
        if let Ok(v) = data_rx.try_recv() {
            results.push(v);
            continue;
        }

        // Check timeout
        if start.elapsed() >= timeout {
            break;
        }

        thread::sleep(Duration::from_millis(1));
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poll_select_first() {
        let (tx1, rx1) = mpsc::channel::<i32>();
        let (_tx2, rx2) = mpsc::channel::<String>();

        tx1.send(42).unwrap();

        let result = poll_select(&rx1, &rx2, Duration::from_millis(100));
        assert_eq!(result, SelectResult::First(42));
    }

    #[test]
    fn test_poll_select_second() {
        let (_tx1, rx1) = mpsc::channel::<i32>();
        let (tx2, rx2) = mpsc::channel::<String>();

        tx2.send("hello".into()).unwrap();

        let result = poll_select(&rx1, &rx2, Duration::from_millis(100));
        assert_eq!(result, SelectResult::Second("hello".into()));
    }

    #[test]
    fn test_poll_select_timeout() {
        let (_tx1, rx1) = mpsc::channel::<i32>();
        let (_tx2, rx2) = mpsc::channel::<String>();

        let result = poll_select(&rx1, &rx2, Duration::from_millis(10));
        assert_eq!(result, SelectResult::Timeout);
    }

    #[test]
    fn test_try_recv_empty() {
        let (_tx, rx) = mpsc::channel::<u32>();
        assert!(rx.try_recv().is_err());
    }

    #[test]
    fn test_try_recv_available() {
        let (tx, rx) = mpsc::channel::<u32>();
        tx.send(1).unwrap();
        assert_eq!(rx.try_recv().unwrap(), 1);
    }

    #[test]
    fn test_priority_select() {
        let (tx1, rx1) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();

        tx1.send(1).unwrap();
        tx2.send(2).unwrap();

        // Primary has priority
        let result = priority_select(&rx1, &rx2);
        assert_eq!(result, Some((1, true)));

        // Now secondary
        let result = priority_select(&rx1, &rx2);
        assert_eq!(result, Some((2, false)));
    }

    #[test]
    fn test_drain_all() {
        let (tx1, rx1) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();

        tx1.send(1).unwrap();
        tx1.send(2).unwrap();
        tx2.send(3).unwrap();

        let results = drain_all(&[rx1, rx2]);
        assert_eq!(results, vec![1, 2, 3]);
    }

    #[test]
    fn test_select_with_stop() {
        let (data_tx, data_rx) = mpsc::channel();
        let (stop_tx, stop_rx) = mpsc::channel();

        data_tx.send(1).unwrap();
        data_tx.send(2).unwrap();
        stop_tx.send(()).unwrap();

        let results = select_with_stop(data_rx, stop_rx, Duration::from_secs(1));
        assert!(results.len() <= 2);
    }
}
