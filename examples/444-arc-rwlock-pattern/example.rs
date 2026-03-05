// 444. Arc<RwLock<T>> read-write sharing
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    let cfg: Arc<RwLock<HashMap<&str,&str>>> = Arc::new(RwLock::new({
        let mut m = HashMap::new(); m.insert("host","localhost"); m.insert("port","8080"); m
    }));

    // Many readers simultaneously
    let readers: Vec<_> = (0..4).map(|id| {
        let c = Arc::clone(&cfg);
        thread::spawn(move || {
            for _ in 0..3 {
                let g = c.read().unwrap();  // shared read — no blocking between readers
                let _ = g.get("host");
                drop(g);
                thread::sleep(Duration::from_millis(5));
            }
            println!("Reader {} done", id);
        })
    }).collect();

    // One writer
    let writer = { let c = Arc::clone(&cfg); thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));
        c.write().unwrap().insert("host", "example.com");  // exclusive
        println!("Writer updated");
    })};

    for r in readers { r.join().unwrap(); }
    writer.join().unwrap();
    println!("host = {}", cfg.read().unwrap().get("host").unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_concurrent_reads() {
        let d = Arc::new(RwLock::new(vec![1,2,3]));
        let hs: Vec<_>=(0..4).map(|_|{ let d=Arc::clone(&d); thread::spawn(move || d.read().unwrap().iter().sum::<i32>()) }).collect();
        for h in hs { assert_eq!(h.join().unwrap(), 6); }
    }
    #[test] fn test_write() { let d=RwLock::new(0u32); *d.write().unwrap()=42; assert_eq!(*d.read().unwrap(),42); }
}
