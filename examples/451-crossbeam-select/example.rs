// 451. select! for multiple channels — std poll
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx1, rx1) = mpsc::channel::<&str>();
    let (tx2, rx2) = mpsc::channel::<u32>();
    let (stop_tx, stop_rx) = mpsc::channel::<()>();

    thread::spawn(move || { for w in ["hello","world","rust"] { thread::sleep(Duration::from_millis(15)); let _=tx1.send(w); } });
    thread::spawn(move || { for i in 0..4u32 { thread::sleep(Duration::from_millis(20)); let _=tx2.send(i*10); } });
    thread::spawn(move || { thread::sleep(Duration::from_millis(120)); let _=stop_tx.send(()); });

    loop {
        if stop_rx.try_recv().is_ok() { println!("stop"); break; }
        match rx1.try_recv() { Ok(s) => { println!("ch1: {}", s); continue; } Err(mpsc::TryRecvError::Disconnected)=>break, _=>{} }
        match rx2.try_recv() { Ok(n) => { println!("ch2: {}", n); continue; } Err(mpsc::TryRecvError::Disconnected)=>break, _=>{} }
        thread::sleep(Duration::from_millis(2));
    }

    // recv_timeout — single-channel wait with timeout
    let (_tx3, rx3) = mpsc::channel::<i32>();
    match rx3.recv_timeout(Duration::from_millis(10)) {
        Ok(v)  => println!("got {}", v),
        Err(e) => println!("recv_timeout: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_try_recv() { let (t,r)=mpsc::channel::<u32>(); assert!(r.try_recv().is_err()); t.send(1).unwrap(); assert_eq!(r.try_recv().unwrap(),1); }
    #[test] fn test_recv_timeout() { let (_t,r)=mpsc::channel::<u32>(); assert!(r.recv_timeout(Duration::from_millis(5)).is_err()); }
}
