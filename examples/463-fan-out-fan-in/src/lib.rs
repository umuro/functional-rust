// 463. Fan-out / fan-in
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

fn fan_map<T,U,F>(items: Vec<T>, n: usize, f: F) -> Vec<U>
where T:Send+'static, U:Send+'static, F:Fn(T)->U+Send+Sync+'static {
    let work = Arc::new(Mutex::new(items.into_iter()));
    let f    = Arc::new(f);
    let (tx,rx) = mpsc::channel::<U>();
    let ws: Vec<_> = (0..n).map(|_| {
        let (w,f,t) = (Arc::clone(&work),Arc::clone(&f),tx.clone());
        thread::spawn(move || loop {
            let item = w.lock().unwrap().next();
            match item { Some(x) => { let _=t.send(f(x)); } None => break }
        })
    }).collect();
    drop(tx);
    for w in ws { w.join().unwrap(); }
    rx.iter().collect()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_fan_map() { let mut r=fan_map((1..=8u32).collect(),4,|x|x*2); r.sort(); assert_eq!(r,vec![2,4,6,8,10,12,14,16]); }
    #[test] fn test_all()    { assert_eq!(fan_map((0..100u32).collect(),8,|x|x).len(), 100); }
}
