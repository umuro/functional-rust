// impl Trait in return position in Rust

// Return opaque iterator (concrete type: Map<Range<i32>, fn>)
fn make_range(start: i32, end: i32) -> impl Iterator<Item = i32> {
    start..end
}

// Return opaque closure
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

fn make_multiplier(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x * n
}

// Chain returns
fn evens_in_range(start: i32, end: i32) -> impl Iterator<Item = i32> {
    (start..end).filter(|x| x % 2 == 0)
}

fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(f(x))
}

// Return impl Trait from different branches requires boxing (or enum)
fn make_iterator(use_odds: bool) -> Box<dyn Iterator<Item = i32>> {
    if use_odds {
        Box::new((1..10).filter(|x| x % 2 != 0))
    } else {
        Box::new((1..10).filter(|x| x % 2 == 0))
    }
}

fn main() {
    let v: Vec<i32> = make_range(1, 6).collect();
    println!("Range: {:?}", v);

    let add5 = make_adder(5);
    println!("add5(10) = {}", add5(10));

    let times3 = make_multiplier(3);
    println!("apply_twice times3 2 = {}", apply_twice(times3, 2));

    let evens: Vec<i32> = evens_in_range(1, 11).collect();
    println!("Evens in 1..11: {:?}", evens);

    let odds: Vec<i32> = make_iterator(true).collect();
    println!("Odds: {:?}", odds);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_range() {
        let v: Vec<i32> = make_range(0, 3).collect();
        assert_eq!(v, vec![0, 1, 2]);
    }

    #[test]
    fn test_make_adder() {
        let f = make_adder(7);
        assert_eq!(f(3), 10);
    }

    #[test]
    fn test_apply_twice() {
        let f = make_multiplier(2);
        assert_eq!(apply_twice(f, 3), 12); // 3*2=6, 6*2=12
    }
}
