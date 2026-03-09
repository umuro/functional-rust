/// Solution 1: Idiomatic Rust — iterator `.map().collect()`
/// This is how a Rust developer writes map: zero recursion, zero overhead.
pub fn map_idiomatic<T, U>(list: &[T], f: impl Fn(&T) -> U) -> Vec<U> {
    list.iter().map(f).collect()
}

/// Solution 2: Tail-recursive with accumulator + reverse
/// Mirrors OCaml's `map_tr`: accumulate in reverse, then reverse at the end.
/// Rust doesn't optimize tail calls, but this avoids growing the call stack
/// by using an explicit loop (iterative translation of the tail-recursive pattern).
pub fn map_tr<T, U>(list: &[T], f: impl Fn(&T) -> U) -> Vec<U> {
    let mut acc = Vec::with_capacity(list.len());
    for item in list {
        acc.push(f(item));
    }
    acc
}

/// Solution 3: CPS (Continuation-Passing Style)
/// Mirrors OCaml's `map_cps`: each step wraps the continuation in a closure.
/// In Rust we use `Box<dyn FnOnce>` to heap-allocate the continuation chain.
/// This preserves output order without a final reverse.
pub fn map_cps<T, U: 'static>(list: &[T], f: impl Fn(&T) -> U) -> Vec<U> {
    type Cont<U> = Box<dyn FnOnce(Vec<U>) -> Vec<U>>;

    fn go<T, U: 'static>(slice: &[T], f: &dyn Fn(&T) -> U, k: Cont<U>) -> Vec<U> {
        match slice {
            [] => k(Vec::new()),
            [head, tail @ ..] => {
                let mapped = f(head);
                go(
                    tail,
                    f,
                    Box::new(move |mut rest| {
                        rest.insert(0, mapped);
                        k(rest)
                    }),
                )
            }
        }
    }

    go(list, &f, Box::new(|v| v))
}

/// Solution 4: Recursive (naive) — direct translation of OCaml's `map_naive`
/// Not tail-recursive; will overflow the stack on large inputs.
pub fn map_naive<T, U>(list: &[T], f: &dyn Fn(&T) -> U) -> Vec<U> {
    match list {
        [] => Vec::new(),
        [head, tail @ ..] => {
            let mut result = vec![f(head)];
            result.extend(map_naive(tail, f));
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_list() {
        let empty: &[i32] = &[];
        assert_eq!(map_idiomatic(empty, |x| x * 2), Vec::<i32>::new());
        assert_eq!(map_tr(empty, |x| x * 2), Vec::<i32>::new());
        assert_eq!(map_cps(empty, |x| x * 2), Vec::<i32>::new());
        assert_eq!(map_naive(empty, &|x| x * 2), Vec::<i32>::new());
    }

    #[test]
    fn test_single_element() {
        let single = &[5];
        assert_eq!(map_idiomatic(single, |x| x * 3), vec![15]);
        assert_eq!(map_tr(single, |x| x * 3), vec![15]);
        assert_eq!(map_cps(single, |x| x * 3), vec![15]);
        assert_eq!(map_naive(single, &|x| x * 3), vec![15]);
    }

    #[test]
    fn test_multiple_elements() {
        let nums = &[1, 2, 3, 4, 5];
        let expected = vec![2, 4, 6, 8, 10];
        assert_eq!(map_idiomatic(nums, |x| x * 2), expected);
        assert_eq!(map_tr(nums, |x| x * 2), expected);
        assert_eq!(map_cps(nums, |x| x * 2), expected);
        assert_eq!(map_naive(nums, &|x| x * 2), expected);
    }

    #[test]
    fn test_type_transformation() {
        let nums = &[1, 2, 3];
        let expected = vec!["1", "2", "3"];
        assert_eq!(map_idiomatic(nums, |x| x.to_string()), expected);
        assert_eq!(map_tr(nums, |x| x.to_string()), expected);
        assert_eq!(map_cps(nums, |x| x.to_string()), expected);
        assert_eq!(map_naive(nums, &|x| x.to_string()), expected);
    }

    #[test]
    fn test_preserves_order() {
        let nums = &[10, 20, 30, 40];
        let expected = vec![11, 21, 31, 41];
        assert_eq!(map_idiomatic(nums, |x| x + 1), expected);
        assert_eq!(map_tr(nums, |x| x + 1), expected);
        assert_eq!(map_cps(nums, |x| x + 1), expected);
        assert_eq!(map_naive(nums, &|x| x + 1), expected);
    }

    #[test]
    fn test_string_slices() {
        let words = &["hello", "world"];
        let expected = vec![5, 5];
        assert_eq!(map_idiomatic(words, |s| s.len()), expected);
        assert_eq!(map_tr(words, |s| s.len()), expected);
        assert_eq!(map_cps(words, |s| s.len()), expected);
        assert_eq!(map_naive(words, &|s| s.len()), expected);
    }
}
