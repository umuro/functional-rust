// 461. Producer-consumer pattern
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_all_consumed() {
        let (tx, rx) = mpsc::sync_channel::<u32>(4);
        let rx = Arc::new(Mutex::new(rx));
        let ps: Vec<_> = (0..2)
            .map(|id| {
                let tx = tx.clone();
                thread::spawn(move || {
                    for i in 0..5u32 {
                        tx.send(id * 10 + i).unwrap();
                    }
                })
            })
            .collect();
        drop(tx);
        let c = thread::spawn(move || rx.lock().unwrap().iter().count());
        for p in ps {
            p.join().unwrap();
        }
        assert_eq!(c.join().unwrap(), 10);
    }
}
