//! # Closure as Argument — Higher-Order Functions

pub fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

pub fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(f(x))
}

pub fn compose<F, G>(f: F, g: G) -> impl Fn(i32) -> i32
where
    F: Fn(i32) -> i32,
    G: Fn(i32) -> i32,
{
    move |x| f(g(x))
}

pub fn filter_with<F: Fn(&i32) -> bool>(items: Vec<i32>, predicate: F) -> Vec<i32> {
    items.into_iter().filter(predicate).collect()
}

pub fn map_with<F: Fn(i32) -> i32>(items: Vec<i32>, mapper: F) -> Vec<i32> {
    items.into_iter().map(mapper).collect()
}

pub fn reduce_with<F: Fn(i32, i32) -> i32>(items: Vec<i32>, initial: i32, reducer: F) -> i32 {
    items.into_iter().fold(initial, reducer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply() {
        assert_eq!(apply(|x| x + 1, 41), 42);
    }

    #[test]
    fn test_apply_twice() {
        assert_eq!(apply_twice(|x| x * 2, 5), 20);
    }

    #[test]
    fn test_compose() {
        let f = compose(|x| x + 1, |x| x * 2);
        assert_eq!(f(5), 11); // (5*2)+1
    }

    #[test]
    fn test_filter() {
        let v = filter_with(vec![1, 2, 3, 4, 5], |&x| x % 2 == 0);
        assert_eq!(v, vec![2, 4]);
    }

    #[test]
    fn test_map() {
        let v = map_with(vec![1, 2, 3], |x| x * x);
        assert_eq!(v, vec![1, 4, 9]);
    }

    #[test]
    fn test_reduce() {
        let sum = reduce_with(vec![1, 2, 3, 4], 0, |a, b| a + b);
        assert_eq!(sum, 10);
    }
}
