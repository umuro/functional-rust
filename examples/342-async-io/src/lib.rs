// 342: Async I/O Concepts
// Polling vs blocking, simulated with threads and channels

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// Approach 1: Blocking I/O
fn blocking_read() -> String {
    thread::sleep(Duration::from_millis(10));
    "data from blocking read".to_string()
}

// Approach 2: Threaded I/O with channels
fn parallel_reads() -> Vec<String> {
    let (tx1, rx) = mpsc::channel();
    let tx2 = tx1.clone();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));
        tx1.send("result1".to_string()).unwrap();
    });

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));
        tx2.send("result2".to_string()).unwrap();
    });

    let mut results = Vec::new();
    for _ in 0..2 {
        results.push(rx.recv().unwrap());
    }
    results
}

// Approach 3: Polling simulation
enum PollResult<T> {
    Ready(T),
    Pending,
}

fn simulate_poll(counter: &mut u32) -> PollResult<&'static str> {
    if *counter >= 3 {
        *counter = 0;
        PollResult::Ready("done")
    } else {
        *counter += 1;
        PollResult::Pending
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blocking() {
        assert_eq!(blocking_read(), "data from blocking read");
    }

    #[test]
    fn test_parallel() {
        let results = parallel_reads();
        assert_eq!(results.len(), 2);
        assert!(results.contains(&"result1".to_string()));
        assert!(results.contains(&"result2".to_string()));
    }

    #[test]
    fn test_poll() {
        let mut counter = 0;
        assert!(matches!(simulate_poll(&mut counter), PollResult::Pending));
        assert!(matches!(simulate_poll(&mut counter), PollResult::Pending));
        assert!(matches!(simulate_poll(&mut counter), PollResult::Pending));
        assert!(matches!(simulate_poll(&mut counter), PollResult::Ready("done")));
    }
}
