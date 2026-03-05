//! # 546. Reborrowing Patterns
//! Creating sub-borrows from existing borrows.

fn read_value(r: &i32) -> i32 { *r }
fn increment(r: &mut i32) { *r += 1; }

/// Demonstrate implicit reborrow: &mut T -> &T
fn implicit_reborrow() {
    let mut x = 42;
    let r = &mut x;

    // &mut T coerces to &T automatically (implicit shared reborrow)
    let val = read_value(r); // r reborrowed as &i32 for this call
    // r still valid after — reborrow ended at end of read_value call
    println!("read_value via reborrow: {}", val);
    *r += 1; // r still usable
    println!("after increment: {}", *r);
}

/// Explicit reborrow
fn explicit_reborrow() {
    let mut data = vec![1, 2, 3];
    let r = &mut data;

    // Explicit shared reborrow — can read while r still in scope
    {
        let shared: &Vec<i32> = &*r; // explicit reborrow as shared
        println!("shared reborrow: {:?}", shared);
        // shared ends here
    }

    // r still valid — push more data
    r.push(4);
    println!("after push via r: {:?}", data);
}

/// Reborrow through function argument
fn reborrow_in_fn() {
    let mut s = String::from("hello");
    let r = &mut s;

    // Pass reborrow to function — r still usable after
    fn print_str(s: &str) { println!("{}", s); }
    print_str(r); // implicit reborrow: &mut String -> &str via Deref
    r.push_str(" world"); // r still valid
    println!("after push: {}", *r);
}

/// Sequential reborrows — using &mut multiple times
fn sequential_reborrows() {
    let mut v = vec![3, 1, 4, 1, 5];

    let r = &mut v;
    r.sort();        // mutable reborrow
    r.dedup();       // another mutable reborrow
    r.push(9);       // yet another

    println!("sequential ops: {:?}", v);
}

/// Reborrow to split a mutable ref
fn split_reborrow() {
    let mut pair = (10i32, 20i32);
    let r = &mut pair;

    // Reborrow parts independently
    let first = &mut r.0;
    *first *= 2;
    // Can't borrow r.1 while r.0 is mutably borrowed:
    drop(first);
    let second = &mut r.1;
    *second *= 3;
    drop(second);

    println!("pair: {:?}", pair);
}

fn main() {
    println!("=== Implicit reborrow ===");
    implicit_reborrow();

    println!("\n=== Explicit reborrow ===");
    explicit_reborrow();

    println!("\n=== Reborrow in function call ===");
    reborrow_in_fn();

    println!("\n=== Sequential reborrows ===");
    sequential_reborrows();

    println!("\n=== Split reborrow ===");
    split_reborrow();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_implicit_reborrow() {
        let mut x = 5;
        let r = &mut x;
        let _ = read_value(r); // implicit reborrow
        *r = 10; // r still valid
        assert_eq!(x, 10);
    }

    #[test]
    fn test_sequential_reborrows() {
        let mut v = vec![3, 1, 2];
        let r = &mut v;
        r.sort();
        r.push(4);
        assert_eq!(v, [1, 2, 3, 4]);
    }
}
