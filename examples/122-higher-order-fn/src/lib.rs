// Example 122: Higher-Order Functions with Lifetime Constraints
//
// When HOFs deal with references, lifetimes must be explicit
// to tell the compiler how long returned references live.

// Approach 1: HOF returning a reference — lifetime ties output to input
// The output &str can't outlive the slice it came from.
pub fn find_first<'a, F>(items: &'a [&'a str], pred: F) -> Option<&'a str>
where
    F: Fn(&str) -> bool,
{
    items.iter().copied().find(|&s| pred(s))
}

// Approach 2: Function composition — owned types, no lifetimes needed
pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(B) -> C,
    G: Fn(A) -> B,
{
    move |x| f(g(x))
}

// Approach 3: Apply — takes a reference, returns a derived reference
// Lifetime 'a ensures: the returned &str lives exactly as long as the input &str.
pub fn apply_to_str<'a, F>(s: &'a str, f: F) -> &'a str
where
    F: Fn(&'a str) -> &'a str,
{
    f(s)
}

// Approach 4: Filter slice by predicate — borrows input, returns subslice refs
// Lifetime elision applies here: the output references live as long as the input slice.
pub fn filter_refs<T, F>(items: &[T], pred: F) -> Vec<&T>
where
    F: Fn(&T) -> bool,
{
    items.iter().filter(|x| pred(x)).collect()
}

// Approach 5: Recursive HOF — apply f n times
pub fn apply_n<T, F>(f: F, n: usize, init: T) -> T
where
    F: Fn(T) -> T,
{
    (0..n).fold(init, |acc, _| f(acc))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first_match() {
        let data = vec!["apple", "banana", "cherry", "date"];
        assert_eq!(find_first(&data, |s| s.len() > 5), Some("banana"));
    }

    #[test]
    fn test_find_first_no_match() {
        let data = vec!["hi", "ok", "no"];
        assert_eq!(find_first(&data, |s| s.len() > 10), None);
    }

    #[test]
    fn test_find_first_empty() {
        let data: Vec<&str> = vec![];
        assert_eq!(find_first(&data, |_| true), None);
    }

    #[test]
    fn test_compose_add_double() {
        let double = |x: i32| x * 2;
        let add1 = |x: i32| x + 1;
        let double_then_add = compose(add1, double);
        assert_eq!(double_then_add(5), 11);
    }

    #[test]
    fn test_compose_string_transform() {
        let trim = |s: String| s.trim().to_string();
        let upper = |s: String| s.to_uppercase();
        let trim_then_upper = compose(upper, trim);
        assert_eq!(trim_then_upper("  hello  ".to_string()), "HELLO");
    }

    #[test]
    fn test_apply_to_str_lifetime() {
        let s = String::from("hello world");
        // The returned slice is tied to the lifetime of `s` — safe.
        let first_word = apply_to_str(&s, |t| t.split_whitespace().next().unwrap_or(""));
        assert_eq!(first_word, "hello");
    }

    #[test]
    fn test_filter_refs_even() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let evens: Vec<&i32> = filter_refs(&nums, |x| *x % 2 == 0);
        assert_eq!(evens, vec![&2, &4, &6]);
    }

    #[test]
    fn test_filter_refs_empty_result() {
        let nums = vec![1, 3, 5];
        let evens: Vec<&i32> = filter_refs(&nums, |x| *x % 2 == 0);
        assert!(evens.is_empty());
    }

    #[test]
    fn test_apply_n_double() {
        // Apply double 3 times: 1 -> 2 -> 4 -> 8
        assert_eq!(apply_n(|x: i32| x * 2, 3, 1), 8);
    }

    #[test]
    fn test_apply_n_zero_times() {
        assert_eq!(apply_n(|x: i32| x + 100, 0, 42), 42);
    }
}
