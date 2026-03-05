// 441. std::thread spawn and join
use std::thread;
use std::time::Duration;

fn main() {
    // Spawn threads, collect handles
    let handles: Vec<_> = (0..4u32).map(|i| {
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(i as u64 * 5));
            println!("Thread {}: {}^2 = {}", i, i, i*i);
            i * i
        })
    }).collect();

    // Join and collect results
    let results: Vec<u32> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    println!("Results: {:?}", results);
    println!("Sum: {}", results.iter().sum::<u32>());

    // Panic recovery
    let h = thread::spawn(|| -> i32 { panic!("boom") });
    match h.join() {
        Ok(v)  => println!("Got {}", v),
        Err(_) => println!("Thread panicked — caught safely"),
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    #[test] fn test_join_result() { assert_eq!(thread::spawn(|| 42u32).join().unwrap(), 42); }
    #[test] fn test_many()       { let hs: Vec<_>=(0..8).map(|i| thread::spawn(move || i*2u32)).collect(); let r:Vec<_>=hs.into_iter().map(|h| h.join().unwrap()).collect(); assert_eq!(r,vec![0,2,4,6,8,10,12,14]); }
    #[test] fn test_panic()      { assert!(thread::spawn(|| panic!("x")).join().is_err()); }
}
