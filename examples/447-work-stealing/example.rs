// 447. Work-stealing pattern
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

type Queue = Arc<Mutex<VecDeque<u32>>>;

fn worker(id: usize, own: Queue, others: Vec<Queue>) {
    loop {
        // Try own front first
        if let Some(j) = own.lock().unwrap().pop_front() {
            println!("worker {} owns job {}", id, j);
            thread::sleep(Duration::from_millis(1));
            continue;
        }
        // Try stealing from back of others
        let mut stole = false;
        for q in &others {
            if let Ok(mut g) = q.try_lock() {
                if let Some(j) = g.pop_back() {
                    println!("worker {} STOLE job {}", id, j);
                    drop(g);
                    thread::sleep(Duration::from_millis(1));
                    stole = true; break;
                }
            }
        }
        if !stole { break; }
    }
}

fn main() {
    let qs: Vec<Queue> = (0..4).map(|_| Arc::new(Mutex::new(VecDeque::new()))).collect();
    // Load all jobs into first worker
    for j in 0..20u32 { qs[0].lock().unwrap().push_back(j); }

    let handles: Vec<_> = (0..4usize).map(|i| {
        let own = Arc::clone(&qs[i]);
        let others: Vec<_> = qs.iter().enumerate().filter(|&(j,_)| j!=i).map(|(_,q)| Arc::clone(q)).collect();
        thread::spawn(move || worker(i, own, others))
    }).collect();
    for h in handles { h.join().unwrap(); }
    println!("all jobs done");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_steal_from_back() {
        let q: Queue = Arc::new(Mutex::new(VecDeque::from([1u32,2,3,4,5])));
        assert_eq!(q.lock().unwrap().pop_back(),  Some(5)); // steal
        assert_eq!(q.lock().unwrap().pop_front(), Some(1)); // own
    }
}
