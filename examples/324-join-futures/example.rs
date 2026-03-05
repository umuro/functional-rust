use std::thread;
use std::time::{Duration, Instant};

fn slow_add(a: i32, b: i32, ms: u64) -> i32 {
    thread::sleep(Duration::from_millis(ms));
    a + b
}

fn join_all<T: Send + 'static>(tasks: Vec<Box<dyn FnOnce()->T+Send>>) -> Vec<T> {
    tasks.into_iter().map(|f| thread::spawn(f))
        .collect::<Vec<_>>().into_iter().map(|h| h.join().unwrap()).collect()
}

fn main() {
    let start = Instant::now();
    let results = join_all(vec![
        Box::new(|| slow_add(1,2,50)),
        Box::new(|| slow_add(3,4,30)),
        Box::new(|| slow_add(5,6,10)),
    ]);
    println!("Results: {results:?}");
    println!("Elapsed: {:.0}ms", start.elapsed().as_secs_f64()*1000.0);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn all_results() {
        let r = join_all(vec![Box::new(||1+1), Box::new(||2+2), Box::new(||3+3)]);
        assert_eq!(r, vec![2,4,6]);
    }
    #[test] fn concurrent_faster() {
        let start = Instant::now();
        join_all(vec![Box::new(||{thread::sleep(Duration::from_millis(30)); 1}), Box::new(||{thread::sleep(Duration::from_millis(30)); 2})]);
        assert!(start.elapsed() < Duration::from_millis(55));
    }
}
