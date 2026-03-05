//! # 519. Closure Type Inference
//! How Rust infers closure types and when annotations are needed.

fn apply<F, T, U>(f: F, x: T) -> U where F: Fn(T) -> U { f(x) }

fn main() {
    // Basic inference — no annotations needed
    let double = |x| x * 2;       // inferred: i32 -> i32 from first use
    let square = |x: i32| x * x;  // explicit input, inferred return
    let greet  = |name: &str| format!("Hello, {}!", name); // explicit

    println!("double(5) = {}", double(5));
    println!("square(4) = {}", square(4));
    println!("{}", greet("Rust"));

    // Type fixed by first use — closure is monomorphic
    let add = |x, y| x + y;
    let _a = add(1i32, 2i32); // fixes T = i32
    // let _b = add(1.0f64, 2.0f64); // ERROR: already fixed as i32

    // Inferred from context (iterator adapter)
    let nums = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = nums.iter().map(|&x| x * 2).collect();
    println!("doubled: {:?}", doubled);

    // Annotation needed for return type coercion
    let parse = |s: &str| -> i64 { s.parse().unwrap_or(0) };
    println!("parse(\"42\") = {}", parse("42"));

    // Multiple closures with same signature, each a distinct type
    let f = |x: i32| x + 1;
    let g = |x: i32| x + 1; // same signature, but different type!
    // Vec<fn(i32)->i32> would work, Vec<decltype(f)> doesn't exist
    let _ = (f, g);

    // Use generic function to accept any Fn
    println!("apply(double, 7) = {}", apply(double, 7));

    // Complex inference chain
    let words = vec!["hello", "world", "rust"];
    let lengths: Vec<_> = words.iter().map(|w| w.len()).collect();
    println!("lengths: {:?}", lengths);

    // Return type inferred through if-else
    let clamp = |x: i32, lo: i32, hi: i32| if x < lo { lo } else if x > hi { hi } else { x };
    println!("clamp(150, 0, 100) = {}", clamp(150, 0, 100));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inferred_arithmetic() {
        let triple = |x| x * 3;
        assert_eq!(triple(7i32), 21);
    }

    #[test]
    fn test_inferred_string() {
        let upper = |s: &str| s.to_uppercase();
        assert_eq!(upper("hello"), "HELLO");
    }

    #[test]
    fn test_inferred_from_context() {
        let v: Vec<i32> = vec![1, 2, 3];
        // type of x is inferred from Vec<i32>
        let doubled: Vec<i32> = v.iter().map(|&x| x * 2).collect();
        assert_eq!(doubled, [2, 4, 6]);
    }

    #[test]
    fn test_apply_infers() {
        let result = apply(|x: i32| x.to_string(), 42);
        assert_eq!(result, "42");
    }
}
