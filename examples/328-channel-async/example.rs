use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn producer(tx: mpsc::Sender<String>, label: &'static str, n: usize, delay_ms: u64) {
    thread::spawn(move || {
        for i in 1..=n {
            thread::sleep(Duration::from_millis(delay_ms));
            tx.send(format!("{label}-{i}")).unwrap();
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel::<String>();
    producer(tx.clone(), "A", 3, 10);
    producer(tx.clone(), "B", 3, 15);
    drop(tx);
    let mut msgs: Vec<String> = rx.into_iter().collect();
    msgs.sort();
    for m in &msgs { println!("Recv: {m}"); }
    println!("Total: {}", msgs.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn collects_all() {
        let (tx, rx) = mpsc::channel::<i32>();
        let tx2 = tx.clone();
        thread::spawn(move || { for i in 0..5 { tx.send(i).unwrap(); } });
        thread::spawn(move || { for i in 5..10 { tx2.send(i).unwrap(); } });
        let mut msgs: Vec<i32> = rx.into_iter().collect();
        msgs.sort();
        assert_eq!(msgs, (0..10).collect::<Vec<_>>());
    }
    #[test] fn closes_on_drop() {
        let (tx, rx) = mpsc::channel::<i32>();
        drop(tx);
        assert!(rx.recv().is_err());
    }
}
