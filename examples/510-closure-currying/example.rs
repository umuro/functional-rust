//! # 510. Currying Pattern in Rust
//! Explicit currying via nested closures returning closures.

/// Curried add: add(x)(y) = x + y
fn add(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

/// Curried multiply: mul(x)(y) = x * y
fn mul(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x * y
}

/// Three-argument curried clamp: clamp(lo)(hi)(x)
/// Uses Box<dyn Fn> because nested impl Fn -> impl Fn isn't allowed
fn clamp(lo: i32) -> Box<dyn Fn(i32) -> Box<dyn Fn(i32) -> i32>> {
    Box::new(move |hi| Box::new(move |x| x.max(lo).min(hi)))
}

/// Convert a 2-arg uncurried function to curried form
/// Uses Box<dyn Fn> for the outer return to allow nested return type
fn curry<A: Copy + 'static, B: Copy + 'static, C: 'static, F>(f: F) -> Box<dyn Fn(A) -> Box<dyn Fn(B) -> C>>
where
    F: Fn(A, B) -> C + Copy + 'static,
{
    Box::new(move |a| Box::new(move |b| f(a, b)))
}

/// Convert a curried function back to uncurried
fn uncurry<A, B, C, F, G>(f: F) -> impl Fn(A, B) -> C
where
    F: Fn(A) -> G,
    G: Fn(B) -> C,
{
    move |a, b| f(a)(b)
}

fn main() {
    // Natural use of curried functions
    let add5 = add(5);
    let times3 = mul(3);
    println!("add5(10) = {}", add5(10));
    println!("times3(7) = {}", times3(7));

    // Chained call: curried style
    println!("add(3)(4) = {}", add(3)(4));
    println!("mul(6)(7) = {}", mul(6)(7));

    // Three-arg curried: clamp(0)(100)(x)
    let clamp_0_100 = clamp(0)(100);
    println!("clamp(0)(100)(150) = {}", clamp_0_100(150));
    println!("clamp(0)(100)(-5) = {}", clamp_0_100(-5));
    println!("clamp(0)(100)(42) = {}", clamp_0_100(42));

    // curry/uncurry conversion
    let uncurried_add = |x: i32, y: i32| x + y;
    let curried_add = curry(uncurried_add);
    let add7 = curried_add(7);
    println!("add7(3) = {}", add7(3));

    let back_to_uncurried = uncurry(add);
    println!("uncurried add(3, 4) = {}", back_to_uncurried(3, 4));

    // Point-free map with curried add
    let numbers = [1, 2, 3, 4, 5];
    let add10 = add(10);
    let result: Vec<i32> = numbers.iter().map(|&x| add10(x)).collect();
    println!("map add(10) [1..5] = {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curried_add() {
        assert_eq!(add(5)(3), 8);
        let add10 = add(10);
        assert_eq!(add10(0), 10);
        assert_eq!(add10(5), 15);
    }

    #[test]
    fn test_curried_clamp() {
        let clamp_5_10 = clamp(5)(10);
        assert_eq!(clamp_5_10(7), 7);
        assert_eq!(clamp_5_10(2), 5);
        assert_eq!(clamp_5_10(15), 10);
    }

    #[test]
    fn test_curry() {
        let f = curry(|x: i32, y: i32| x * y);
        assert_eq!(f(6)(7), 42);
    }

    #[test]
    fn test_uncurry() {
        let g = uncurry(add);
        assert_eq!(g(3, 4), 7);
    }
}
