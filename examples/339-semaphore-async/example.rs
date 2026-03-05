use std::sync::{Arc,Mutex,Condvar};
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicUsize,Ordering};

struct Semaphore { count: Mutex<usize>, cond: Condvar }

impl Semaphore {
    fn new(n: usize) -> Arc<Self> { Arc::new(Self{count:Mutex::new(n),cond:Condvar::new()}) }
    fn acquire(&self) { let mut c=self.count.lock().unwrap(); while *c==0{c=self.cond.wait(c).unwrap();} *c-=1; }
    fn release(&self) { *self.count.lock().unwrap()+=1; self.cond.notify_one(); }
}

struct Permit<'a>(&'a Semaphore);
impl<'a> Drop for Permit<'a> { fn drop(&mut self) { self.0.release(); } }
impl Semaphore { fn permit(&self) -> Permit<'_> { self.acquire(); Permit(self) } }

fn main() {
    let sem = Semaphore::new(3);
    let active = Arc::new(AtomicUsize::new(0));
    let peak = Arc::new(AtomicUsize::new(0));
    let handles: Vec<_> = (0..8).map(|i| {
        let sem=Arc::clone(&sem); let a=Arc::clone(&active); let p=Arc::clone(&peak);
        thread::spawn(move || {
            let _permit = sem.permit();
            let cur = a.fetch_add(1,Ordering::SeqCst)+1;
            p.fetch_max(cur,Ordering::SeqCst);
            println!("Worker {i} running (active={cur})");
            thread::sleep(Duration::from_millis(10));
            a.fetch_sub(1,Ordering::SeqCst);
        })
    }).collect();
    for h in handles { h.join().unwrap(); }
    println!("Peak concurrency: {}", peak.load(Ordering::SeqCst));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn limits_concurrency() {
        let sem = Semaphore::new(2);
        let active = Arc::new(AtomicUsize::new(0));
        let peak = Arc::new(AtomicUsize::new(0));
        let handles: Vec<_> = (0..6).map(|_| {
            let sem=Arc::clone(&sem); let a=Arc::clone(&active); let p=Arc::clone(&peak);
            thread::spawn(move || {
                let _perm = sem.permit();
                let c = a.fetch_add(1,Ordering::SeqCst)+1;
                p.fetch_max(c,Ordering::SeqCst);
                thread::sleep(Duration::from_millis(5));
                a.fetch_sub(1,Ordering::SeqCst);
            })
        }).collect();
        for h in handles { h.join().unwrap(); }
        assert!(peak.load(Ordering::SeqCst)<=2);
    }
}
