#![allow(clippy::all)]
// 101: Move Semantics
// Ownership transfer — after move, original is invalid

// Approach 1: Move with String (heap-allocated)
fn take_ownership(s: String) -> usize {
    s.len() // s is consumed here
}

fn demonstrate_move() {
    let s = String::from("hello");
    let len = take_ownership(s);
    // println!("{}", s); // ERROR: s has been moved!
    assert_eq!(len, 5);
}

// Approach 2: Copy types don't move
fn demonstrate_copy() {
    let x = 42;
    let y = x; // copy, not move — x is still valid
    assert_eq!(x, 42);
    assert_eq!(y, 42);
}

// Approach 3: Move in collections
fn demonstrate_vec_move() {
    let v1 = vec![1, 2, 3];
    let v2 = v1; // v1 is moved to v2
                 // println!("{:?}", v1); // ERROR: v1 has been moved
    assert_eq!(v2, vec![1, 2, 3]);
}

// Return value transfers ownership back
fn create_string() -> String {
    let s = String::from("created");
    s // ownership transferred to caller
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_ownership() {
        let s = String::from("hello");
        assert_eq!(take_ownership(s), 5);
    }

    #[test]
    fn test_copy_types() {
        let x = 42;
        let y = x;
        assert_eq!(x + y, 84);
    }

    #[test]
    fn test_vec_move() {
        let v1 = vec![1, 2, 3];
        let v2 = v1;
        assert_eq!(v2.len(), 3);
    }

    #[test]
    fn test_return_ownership() {
        let s = create_string();
        assert_eq!(s, "created");
    }
}
