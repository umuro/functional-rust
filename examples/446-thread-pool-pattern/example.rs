// 446. Thread pool implementation
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool { workers: Vec<thread::JoinHandle<()>>, tx: Option<mpsc::Sender<Job>> }

impl ThreadPool {
    pub fn new(n: usize) -> Self {
        let (tx, rx) = mpsc::channel::<Job>();
        let rx = Arc::new(Mutex::new(rx));
        let workers = (0..n).map(|id| {
            let rx = Arc::clone(&rx);
            thread::spawn(move || loop {
                match rx.lock().unwrap().recv() {
                    Ok(f)  => { println!("worker {} running job", id); f(); }
                    Err(_) => { println!("worker {} exiting", id); break; }
                }
            })
        }).collect();
        ThreadPool { workers, tx: Some(tx) }
    }
    pub fn execute<F: FnOnce()+Send+'static>(&self, f: F) {
        self.tx.as_ref().unwrap().send(Box::new(f)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.tx.take());
        for w in self.workers.drain(..) { w.join().unwrap(); }
    }
}

fn main() {
    let pool = ThreadPool::new(4);
    let results = Arc::new(Mutex::new(vec![]));
    for i in 0..8 {
        let r = Arc::clone(&results);
        pool.execute(move || r.lock().unwrap().push(i * i));
    }
    drop(pool);
    let mut r = results.lock().unwrap().clone();
    r.sort();
    println!("results: {:?}", r);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize,Ordering};
    #[test] fn test_all_jobs() {
        let count = Arc::new(AtomicUsize::new(0));
        { let p = ThreadPool::new(2);
          for _ in 0..10 { let c=Arc::clone(&count); p.execute(move || { c.fetch_add(1,Ordering::Relaxed); }); }
        }
        assert_eq!(count.load(Ordering::SeqCst), 10);
    }
}
