use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn spawn_worker(id: usize, delay_ms: u64) -> thread::JoinHandle<String> {
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(delay_ms));
        format!("worker-{id} done after {delay_ms}ms")
    })
}

fn spawn_with_channel(n: usize) -> Vec<String> {
    let (tx, rx) = mpsc::channel();
    for i in 0..n {
        let tx = tx.clone();
        thread::spawn(move || { thread::sleep(Duration::from_millis((n-i) as u64*5)); tx.send(format!("task-{i}")).unwrap(); });
    }
    drop(tx);
    rx.into_iter().collect()
}

fn main() {
    let handles: Vec<_> = (0..5).map(|i| spawn_worker(i, (5-i) as u64*10)).collect();
    for h in handles { println!("{}", h.join().unwrap()); }
    let mut results = spawn_with_channel(4);
    results.sort();
    println!("Channel: {results:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn worker_string() { let h = spawn_worker(0,1); assert!(h.join().unwrap().contains("worker-0")); }
    #[test] fn channel_all() { assert_eq!(spawn_with_channel(5).len(), 5); }
}
