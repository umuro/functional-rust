use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn race<T: Send + 'static>(tasks: Vec<(Box<dyn FnOnce()->T+Send>, &'static str)>) -> (&'static str, T) {
    let (tx, rx) = mpsc::channel();
    for (f, label) in tasks {
        let tx = tx.clone();
        thread::spawn(move || { let _ = tx.send((label, f())); });
    }
    rx.recv().unwrap()
}

fn with_timeout<T: Send + 'static>(f: Box<dyn FnOnce()->T+Send>, ms: u64) -> Option<T> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || { let _ = tx.send(f()); });
    rx.recv_timeout(Duration::from_millis(ms)).ok()
}

fn main() {
    let tasks: Vec<(Box<dyn FnOnce()->i32+Send>, &'static str)> = vec![
        (Box::new(|| {thread::sleep(Duration::from_millis(50)); 1}), "slow"),
        (Box::new(|| {thread::sleep(Duration::from_millis(10)); 2}), "fast"),
    ];
    let (winner, val) = race(tasks);
    println!("Winner: {winner} = {val}");
    println!("Timeout ok: {:?}", with_timeout(Box::new(|| {thread::sleep(Duration::from_millis(5)); 42}), 100));
    println!("Timeout fail: {:?}", with_timeout(Box::new(|| {thread::sleep(Duration::from_millis(200)); 0}), 50));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn fastest_wins() {
        let tasks: Vec<(Box<dyn FnOnce()->i32+Send>, &'static str)> = vec![
            (Box::new(||{thread::sleep(Duration::from_millis(50));1}), "slow"),
            (Box::new(||{thread::sleep(Duration::from_millis(5));2}), "fast"),
        ];
        let (_, v) = race(tasks);
        assert_eq!(v, 2);
    }
    #[test] fn timeout_succeeds() { assert_eq!(with_timeout(Box::new(||{thread::sleep(Duration::from_millis(5));99}), 200), Some(99)); }
}
