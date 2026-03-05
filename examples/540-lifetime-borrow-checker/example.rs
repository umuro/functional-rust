//! # 540. Borrow Checker Internals
//! Understanding why the borrow checker's rules exist and how to work with them.

/// Rule 1: Cannot have &mut while & exists
fn rule_one_demo() {
    let mut v = vec![1, 2, 3];

    let r1 = &v;     // shared borrow
    let r2 = &v;     // another shared borrow — OK!
    println!("r1: {:?}, r2: {:?}", r1, r2);
    // r1 and r2 last used above — borrows end here (NLL)

    v.push(4);       // mutable borrow — OK now (r1, r2 no longer live)
    println!("after push: {:?}", v);
}

/// Rule 2: Cannot have two &mut at the same time
fn rule_two_demo() {
    let mut x = 5;
    let r1 = &mut x;  // exclusive mutable borrow
    *r1 += 1;         // use r1
    drop(r1);         // r1 ends here
    let r2 = &mut x;  // new mutable borrow — OK (r1 ended)
    *r2 *= 2;
    drop(r2);
    println!("x = {}", x); // 12: (5+1)*2
}

/// Rule 3: Cannot use moved value
fn rule_three_demo() {
    let s1 = String::from("hello");
    let s2 = s1;          // s1 MOVED to s2
    // println!("{}", s1); // ERROR: s1 was moved
    println!("s2: {}", s2);

    // Clone to keep both:
    let s3 = String::from("world");
    let s4 = s3.clone();  // s3 still valid
    println!("{} and {}", s3, s4);
}

/// Understanding the "borrow graph"
fn borrow_graph_demo() {
    let mut data = vec![1, 2, 3, 4, 5];

    // Reads are non-exclusive: any number allowed
    let sum: i32 = data.iter().sum();
    let count = data.len();
    println!("sum={}, count={} (multiple shared borrows)", sum, count);
    // Borrows ended above

    // Write is exclusive: only one at a time
    data[0] *= 10;
    data.retain(|&x| x > 5);
    println!("filtered: {:?}", data);
}

/// Patterns to work around borrow checker
fn workaround_patterns() {
    let mut v = vec![1, 2, 3, 4, 5];

    // Pattern 1: Clone to read while also mutating
    let first = v[0]; // Copy type — no borrow needed
    v.push(first * 10);
    println!("v: {:?}", v);

    // Pattern 2: Use index instead of reference
    let len = v.len();
    for i in 0..len {
        v[i] *= 2; // indexed access avoids conflicting borrows
    }
    println!("doubled: {:?}", v);

    // Pattern 3: Split the borrow
    let (left, right) = v.split_at_mut(3);
    left[0] = 999;
    right[0] = 888;
    println!("split-mutated: {:?}", v);
}

fn main() {
    println!("=== Rule 1: Shared borrows are compatible ===");
    rule_one_demo();

    println!("\n=== Rule 2: Exclusive &mut — only one at a time ===");
    rule_two_demo();

    println!("\n=== Rule 3: Move semantics ===");
    rule_three_demo();

    println!("\n=== Borrow graph analysis ===");
    borrow_graph_demo();

    println!("\n=== Workaround patterns ===");
    workaround_patterns();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_borrows() {
        let v = vec![1, 2, 3];
        let a = &v;
        let b = &v;
        assert_eq!(a.len(), b.len());
    }

    #[test]
    fn test_sequential_mut_borrows() {
        let mut x = 1;
        { let r = &mut x; *r = 2; }
        { let r = &mut x; *r = 3; }
        assert_eq!(x, 3);
    }

    #[test]
    fn test_split_borrow() {
        let mut v = [1, 2, 3, 4];
        let (l, r) = v.split_at_mut(2);
        l[0] = 10;
        r[0] = 30;
        assert_eq!(v, [10, 2, 30, 4]);
    }
}
