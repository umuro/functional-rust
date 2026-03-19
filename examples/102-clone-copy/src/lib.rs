#![allow(clippy::all)]
// 102: Clone vs Copy
// Copy = implicit bitwise copy (small stack types)
// Clone = explicit deep copy (heap types)

// Copy types: integers, floats, bool, char, tuples of Copy types
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

// Clone-only types: anything with heap allocation
#[derive(Debug, Clone, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

fn demonstrate_copy() {
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = p1; // Copy — p1 still valid
    assert_eq!(p1, p2);
    // Both usable!
    println!("p1: {:?}, p2: {:?}", p1, p2);
}

fn demonstrate_clone() {
    let p1 = Person {
        name: "Alice".into(),
        age: 30,
    };
    let p2 = p1.clone(); // explicit deep copy
                         // p1 is still valid because we cloned
    assert_eq!(p1, p2);
    println!("p1: {:?}", p1);

    let p3 = p1; // move — p1 no longer valid
                 // println!("{:?}", p1); // ERROR!
    println!("p3: {:?}", p3);
}

fn demonstrate_vec_clone() {
    let v1 = vec![1, 2, 3];
    let v2 = v1.clone(); // deep copy
    assert_eq!(v1, v2);
    // v1 still valid because we cloned
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy() {
        let a = 42;
        let b = a; // Copy
        assert_eq!(a, b);
    }

    #[test]
    fn test_point_copy() {
        let p1 = Point { x: 1.0, y: 2.0 };
        let p2 = p1;
        assert_eq!(p1.x, p2.x); // both valid
    }

    #[test]
    fn test_clone() {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        assert_eq!(s1, s2); // both valid
    }

    #[test]
    fn test_vec_clone() {
        let v1 = vec![1, 2, 3];
        let v2 = v1.clone();
        assert_eq!(v1, v2);
    }
}
