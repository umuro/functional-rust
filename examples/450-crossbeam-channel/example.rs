// 450. Crossbeam channels — std implementation
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

fn main() {
    // Bounded: SyncSender blocks when full
    let (tx, rx) = mpsc::sync_channel::<u32>(3);
    let p = thread::spawn(move || {
        for i in 1..=6 { tx.send(i).unwrap(); println!("sent {}", i); }
    });
    thread::sleep(Duration::from_millis(20));
    for _ in 0..6 { println!("recv {}", rx.recv().unwrap()); }
    p.join().unwrap();

    // Multi-consumer: wrap Receiver in Arc<Mutex>
    let (tx, rx) = mpsc::channel::<u32>();
    let rx = Arc::new(Mutex::new(rx));
    let cs: Vec<_> = (0..3).map(|id| {
        let rx = Arc::clone(&rx);
        thread::spawn(move || loop {
            match rx.lock().unwrap().recv() {
                Ok(v)  => println!("consumer {} got {}", id, v),
                Err(_) => break,
            }
        })
    }).collect();
    for i in 0..9 { tx.send(i).unwrap(); }
    drop(tx);
    for c in cs { c.join().unwrap(); }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_sync_bounded() {
        let (tx,rx) = mpsc::sync_channel::<u32>(2);
        tx.send(1).unwrap(); tx.send(2).unwrap();
        assert!(tx.try_send(3).is_err()); // full
        assert_eq!(rx.recv().unwrap(), 1);
        assert!(tx.try_send(3).is_ok());
    }
    #[test] fn test_multi_consumer() {
        let (tx,rx)=mpsc::channel::<u32>(); let rx=Arc::new(Mutex::new(rx));
        let res=Arc::new(Mutex::new(vec![]));
        let cs:Vec<_>=(0..2).map(|_|{let rx=Arc::clone(&rx);let r=Arc::clone(&res); thread::spawn(move || loop { match rx.lock().unwrap().recv(){Ok(v)=>{r.lock().unwrap().push(v);}Err(_)=>break;} })}).collect();
        for i in 0..6u32 { tx.send(i).unwrap(); } drop(tx);
        for c in cs { c.join().unwrap(); }
        let mut v=res.lock().unwrap().clone(); v.sort(); assert_eq!(v,vec![0,1,2,3,4,5]);
    }
}
