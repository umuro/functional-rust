//! Borrow Checker Internals
//!
//! Understanding why the borrow checker's rules exist.

/// Rule: Cannot have &mut while & exists.
pub fn rule_shared_vs_mutable() -> Vec<i32> {
    let mut v = vec![1, 2, 3];
    let r1 = &v;
    let r2 = &v; // multiple shared OK
    let _ = (r1.len(), r2.len()); // use borrows
                                  // r1, r2 end here (NLL)
    v.push(4); // mutable borrow OK now
    v
}

/// Rule: Only one &mut at a time.
pub fn rule_exclusive_mutable(v: &mut Vec<i32>) {
    v.push(1);
    v.push(2);
    // Each push is sequential, not simultaneous
}

/// Demonstrates reborrowing.
pub fn reborrow_demo(v: &mut Vec<i32>) {
    let len = v.len(); // temporary shared borrow
    v.push(len as i32); // back to mutable
}

/// Working with owned vs borrowed.
pub fn ownership_rules() {
    let s = String::from("hello");
    let r = &s; // borrow
    assert_eq!(r, "hello");
    // s still owned here
    drop(s); // explicit drop
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_vs_mutable() {
        let v = rule_shared_vs_mutable();
        assert_eq!(v, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_exclusive_mutable() {
        let mut v = vec![];
        rule_exclusive_mutable(&mut v);
        assert_eq!(v, vec![1, 2]);
    }

    #[test]
    fn test_reborrow() {
        let mut v = vec![1, 2, 3];
        reborrow_demo(&mut v);
        assert_eq!(v, vec![1, 2, 3, 3]);
    }
}
