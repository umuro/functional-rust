// 445. Multi-producer single-consumer channels
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel::<String>();

    // Multiple producers — clone the sender
    let handles: Vec<_> = (0..3).map(|id| {
        let tx = tx.clone();
        thread::spawn(move || {
            for i in 1..=5 { tx.send(format!("p{}-msg{}", id, i)).unwrap(); }
        })
    }).collect();
    drop(tx); // drop original — channel closes when all clones drop

    // Consumer: for-loop exits when channel closes
    for msg in rx { println!("got: {}", msg); }
    for h in handles { h.join().unwrap(); }

    // try_iter — non-blocking drain
    let (tx2, rx2) = mpsc::channel::<u32>();
    for i in 0..5 { tx2.send(i).unwrap(); }
    drop(tx2);
    let v: Vec<u32> = rx2.try_iter().collect();
    println!("drained: {:?}", v);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_send_recv() { let (t,r)=mpsc::channel(); t.send(42u32).unwrap(); assert_eq!(r.recv().unwrap(),42); }
    #[test] fn test_closed()    { let (t,r)=mpsc::channel::<i32>(); drop(t); assert!(r.recv().is_err()); }
    #[test] fn test_multi()     {
        let (t,r)=mpsc::channel::<u32>();
        let hs:Vec<_>=(0..4).map(|i|{let t=t.clone(); thread::spawn(move || t.send(i).unwrap())}).collect();
        drop(t); let mut v:Vec<u32>=r.iter().collect(); v.sort(); assert_eq!(v,vec![0,1,2,3]);
        for h in hs { h.join().unwrap(); }
    }
}
