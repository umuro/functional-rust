//! # 539. Non-Lexical Lifetimes (NLL)
//! Modern borrow checker: borrows end at last use, not end of block.

fn nll_basic() {
    let mut v = vec![1, 2, 3, 4, 5];

    // NLL: borrow of v ends after this line (last use of 'first')
    let first = v[0]; // shared borrow of v
    // Pre-NLL: error — v still "borrowed" until end of block
    // With NLL: borrow ended at line above — mutation OK here!
    v.push(6);
    println!("first: {}, v: {:?}", first, v);
}

fn nll_conditional() {
    let mut data = vec![10, 20, 5, 30, 15];

    // NLL: borrow in condition ends before the mutation
    let max_val = data.iter().max().copied().unwrap_or(0);
    // Pre-NLL might complain; with NLL this is fine:
    if max_val > 15 {
        data.push(max_val * 2); // mutation after borrow ended
    }
    println!("data after conditional mutation: {:?}", data);
}

fn nll_return_in_branch() -> String {
    let mut s = String::from("hello");

    // Get a reference — if it's long enough, return early
    let r = s.as_str(); // borrow starts
    if r.len() > 3 {
        return r.to_uppercase(); // borrow used here, then ends
    }
    // Borrow ended (via return or end of if block)
    s.push_str(" world"); // mutation OK with NLL
    s
}

fn nll_loop() {
    let mut map = std::collections::HashMap::new();
    map.insert("key", 0i32);

    // NLL enables this pattern in loops
    for _ in 0..5 {
        // Borrow to read
        let current = *map.get("key").unwrap();
        // Borrow ended — can mutate now
        map.insert("key", current + 1);
    }
    println!("Counter: {}", map["key"]);
}

fn nll_two_phase_borrows() {
    let mut v = vec![1, 2, 3];

    // Two-phase borrows: &mut borrow starts but "activates" only at the call
    // This enables: v.push(v.len() as i32) in newer Rust
    let len = v.len() as i32; // shared borrow — fast
    v.push(len);              // mutable borrow
    println!("v with len appended: {:?}", v);
}

fn main() {
    println!("=== NLL: borrow ends at last use ===");
    nll_basic();

    println!("\n=== NLL: conditional borrow ===");
    nll_conditional();

    println!("\n=== NLL: early return ===");
    println!("{}", nll_return_in_branch());

    println!("\n=== NLL: loop borrow ===");
    nll_loop();

    println!("\n=== NLL: two-phase borrows ===");
    nll_two_phase_borrows();

    // Example that would fail even with NLL (actual conflict):
    let mut v = vec![1, 2, 3];
    let r = &v[0]; // borrow of v[0]
    // v.push(4); // This would error — r is still live!
    println!("r = {}", r); // r used here — borrow ends
    v.push(4); // NOW mutation is fine (r no longer live)
    println!("v: {:?}", v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nll_basic() {
        let mut v = vec![1, 2, 3];
        let first = v[0];
        v.push(4); // NLL allows this
        assert_eq!(first, 1);
        assert_eq!(v.last(), Some(&4));
    }

    #[test]
    fn test_nll_loop() {
        let mut map = std::collections::HashMap::new();
        map.insert("x", 0i32);
        for _ in 0..3 {
            let v = *map.get("x").unwrap();
            map.insert("x", v + 1);
        }
        assert_eq!(map["x"], 3);
    }

    #[test]
    fn test_nll_return() {
        let result = nll_return_in_branch();
        assert!(!result.is_empty());
    }
}
