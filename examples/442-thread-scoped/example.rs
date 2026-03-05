// 442. Scoped threads with thread::scope
use std::thread;

fn parallel_sum(data: &[i64]) -> i64 {
    let (left, right) = data.split_at(data.len() / 2);
    let mut ls = 0i64; let mut rs = 0i64;
    thread::scope(|s| {
        let t1 = s.spawn(|| left.iter().sum::<i64>());
        let t2 = s.spawn(|| right.iter().sum::<i64>());
        ls = t1.join().unwrap();
        rs = t2.join().unwrap();
    });
    ls + rs
}

fn main() {
    let data: Vec<i64> = (1..=100).collect();
    println!("Sum = {} (expected 5050)", parallel_sum(&data));

    // Borrow a local string without Arc
    let message = String::from("hello from stack");
    thread::scope(|s| {
        s.spawn(|| println!("Thread sees: {}", message));
        s.spawn(|| println!("Thread 2 len: {}", message.len()));
    });
}

#[cfg(test)]
mod tests {
    use super::*; use std::thread;
    #[test] fn test_sum()   { let d: Vec<i64>=(1..=100).collect(); assert_eq!(parallel_sum(&d), 5050); }
    #[test] fn test_borrow(){ let s = String::from("hi"); thread::scope(|sc| { sc.spawn(|| assert_eq!(s.len(),2)); }); }
}
