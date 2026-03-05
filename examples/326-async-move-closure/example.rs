use std::sync::{Arc, Mutex};
use std::thread;

fn make_greeter(name: String) -> impl Fn() {
    move || println!("Hello, {name}!")
}

fn make_counter(start: i32) -> impl FnMut() -> i32 {
    let mut count = start;
    move || { let v = count; count += 1; v }
}

fn shared_state_demo() -> i32 {
    let shared = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..5).map(|_| {
        let shared = Arc::clone(&shared);
        thread::spawn(move || { *shared.lock().unwrap() += 1; }) // like async move {}
    }).collect();
    for h in handles { h.join().unwrap(); }
    *shared.lock().unwrap()
}

fn main() {
    make_greeter("Alice".into())();
    make_greeter("Bob".into())();
    let mut c = make_counter(10);
    println!("{}", c()); println!("{}", c());
    println!("Shared: {}", shared_state_demo());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn counter_increments() { let mut c = make_counter(0); assert_eq!(c(),0); assert_eq!(c(),1); }
    #[test] fn shared_counts_all() { assert_eq!(shared_state_demo(), 5); }
}
