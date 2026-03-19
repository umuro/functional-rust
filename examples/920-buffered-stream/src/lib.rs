#![allow(clippy::all)]
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

struct Semaphore {
    count: Mutex<usize>,
    cond: Condvar,
}

impl Semaphore {
    fn new(n: usize) -> Arc<Self> {
        Arc::new(Self {
            count: Mutex::new(n),
            cond: Condvar::new(),
        })
    }
    fn acquire(&self) {
        let mut c = self.count.lock().unwrap();
        while *c == 0 {
            c = self.cond.wait(c).unwrap();
        }
        *c -= 1;
    }
    fn release(&self) {
        *self.count.lock().unwrap() += 1;
        self.cond.notify_one();
    }
}

fn buffered_map<T, U, F>(items: Vec<T>, concurrency: usize, f: F) -> Vec<U>
where
    T: Send + 'static,
    U: Send + 'static,
    F: Fn(T) -> U + Send + Sync + 'static,
{
    let sem = Semaphore::new(concurrency);
    let f = Arc::new(f);
    let results: Arc<Mutex<Vec<(usize, U)>>> = Arc::new(Mutex::new(Vec::new()));

    let handles: Vec<_> = items
        .into_iter()
        .enumerate()
        .map(|(i, item)| {
            let sem = Arc::clone(&sem);
            let f = Arc::clone(&f);
            let results = Arc::clone(&results);
            thread::spawn(move || {
                sem.acquire();
                let result = f(item);
                sem.release();
                results.lock().unwrap().push((i, result));
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    let mut res = results.lock().unwrap().drain(..).collect::<Vec<_>>();
    res.sort_by_key(|(i, _)| *i);
    res.into_iter().map(|(_, v)| v).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn buffered_map_all_results() {
        let r = buffered_map(vec![1u64, 2, 3, 4, 5], 2, |x| x * 2);
        assert_eq!(r, vec![2, 4, 6, 8, 10]);
    }
    #[test]
    fn concurrency_1_sequential() {
        let r = buffered_map(vec![1, 2, 3], 1, |x: i32| x + 10);
        assert_eq!(r, vec![11, 12, 13]);
    }
}
