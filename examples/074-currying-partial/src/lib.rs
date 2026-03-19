// 074: Currying and Partial Application

// Approach 1: Closures for partial application
fn add(x: i32, y: i32) -> i32 {
    x + y
}
fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

fn make_multiplier(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x * y
}

// Approach 2: Curried function returning closures
fn make_greeting(prefix: &str) -> impl Fn(&str) -> Box<dyn Fn(&str) -> String + '_> + '_ {
    move |name: &str| {
        let owned_prefix = prefix.to_string();
        let owned_name = name.to_string();
        Box::new(move |suffix: &str| format!("{} {}{}", owned_prefix, owned_name, suffix))
    }
}

// Simpler version:
fn greet(prefix: &str, name: &str, suffix: &str) -> String {
    format!("{} {}{}", prefix, name, suffix)
}

// Approach 3: Higher-order + partial application
fn apply_twice(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

fn compose<A, B, C>(f: impl Fn(B) -> C, g: impl Fn(A) -> B) -> impl Fn(A) -> C {
    move |x| f(g(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_application() {
        let add5 = make_adder(5);
        assert_eq!(add5(3), 8);
        assert_eq!(add5(0), 5);

        let double = make_multiplier(2);
        assert_eq!(double(7), 14);

        let triple = make_multiplier(3);
        assert_eq!(triple(4), 12);
    }

    #[test]
    fn test_apply_twice() {
        let add5 = make_adder(5);
        assert_eq!(apply_twice(&add5, 0), 10);
        assert_eq!(apply_twice(&add5, 5), 15);

        let double = make_multiplier(2);
        assert_eq!(apply_twice(&double, 3), 12);
    }

    #[test]
    fn test_compose() {
        let add5_then_double = compose(make_multiplier(2), make_adder(5));
        assert_eq!(add5_then_double(3), 16); // (3+5)*2

        let double_then_add5 = compose(make_adder(5), make_multiplier(2));
        assert_eq!(double_then_add5(3), 11); // 3*2+5
    }

    #[test]
    fn test_greet() {
        assert_eq!(greet("Hello", "World", "!"), "Hello World!");
    }
}
