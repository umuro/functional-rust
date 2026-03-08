// === Solution 1: Naive recursive map ===
// Direct translation of OCaml's `let rec map f = function | [] -> [] | h::t -> f h :: map f t`
// Not tail-recursive: the cons (`f h ::`) happens AFTER the recursive call returns.
// Will stack-overflow on large inputs — same problem as OCaml's naive version.
pub fn map_naive<T, U, F: Fn(&T) -> U>(f: &F, list: &[T]) -> Vec<U> {
    match list {
        [] => vec![],
        [head, tail @ ..] => {
            let mut result = vec![f(head)];
            result.extend(map_naive(f, tail));
            result
        }
    }
}

// === Solution 2: Accumulator-based map (loop) ===
// In OCaml, this is tail-recursive: `let rec go acc = function | [] -> List.rev acc | h::t -> go (f h :: acc) t`
// OCaml cons prepends (O(1)) building the list in reverse, then List.rev fixes order.
// Rust has no TCO guarantee, so the idiomatic translation is a loop.
// Vec::push appends (O(1) amortized) and builds in forward order — no reverse needed.
pub fn map_acc<T, U, F: Fn(&T) -> U>(f: &F, list: &[T]) -> Vec<U> {
    let mut acc = Vec::with_capacity(list.len());
    for item in list {
        acc.push(f(item));
    }
    acc
}

// === Solution 3: CPS (Continuation-Passing Style) ===
// OCaml: `let rec go k = function | [] -> k [] | h::t -> go (fun rest -> k (f h :: rest)) t`
// Each step wraps the previous continuation, building a chain of closures.
// In OCaml this is tail-recursive (go is the last call). In Rust we build the
// continuation chain with a loop (since no TCO), then apply it.
// Note: applying the chain still uses O(n) stack frames from nested closures.
pub fn map_cps<T, U, F: Fn(&T) -> U>(f: &F, list: &[T]) -> Vec<U> {
    // Identity continuation — base case returns its argument unchanged
    let mut cont: Box<dyn FnOnce(Vec<U>) -> Vec<U>> = Box::new(|v| v);

    // Build the chain in reverse so the first element's continuation is outermost.
    // Each layer pushes its mapped value, then delegates to the previous continuation.
    for item in list.iter().rev() {
        let mapped = f(item);
        let prev = cont;
        cont = Box::new(move |mut rest: Vec<U>| {
            rest.push(mapped);
            prev(rest)
        });
    }

    cont(Vec::new())
}

// === Solution 4: Idiomatic Rust ===
// How a Rust developer actually writes map — iterator adaptor chain.
// No recursion, no manual accumulation. The compiler optimizes this into a tight loop.
pub fn map_idiomatic<T, U, F: Fn(&T) -> U>(f: F, list: &[T]) -> Vec<U> {
    list.iter().map(f).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- map_naive ---
    #[test]
    fn naive_empty() {
        assert_eq!(map_naive(&|x: &i32| x * 2, &[]), Vec::<i32>::new());
    }

    #[test]
    fn naive_single() {
        assert_eq!(map_naive(&|x: &i32| x + 1, &[5]), vec![6]);
    }

    #[test]
    fn naive_multiple() {
        assert_eq!(map_naive(&|x: &i32| x * 2, &[1, 2, 3, 4]), vec![2, 4, 6, 8]);
    }

    #[test]
    fn naive_strings() {
        let words = ["hello", "world"];
        assert_eq!(
            map_naive(&|s: &&str| s.to_uppercase(), &words),
            vec!["HELLO", "WORLD"]
        );
    }

    // --- map_acc ---
    #[test]
    fn acc_empty() {
        assert_eq!(map_acc(&|x: &i32| x * 2, &[]), Vec::<i32>::new());
    }

    #[test]
    fn acc_single() {
        assert_eq!(map_acc(&|x: &i32| x + 1, &[5]), vec![6]);
    }

    #[test]
    fn acc_multiple() {
        assert_eq!(map_acc(&|x: &i32| x * 2, &[1, 2, 3, 4]), vec![2, 4, 6, 8]);
    }

    #[test]
    fn acc_preserves_order() {
        assert_eq!(map_acc(&|x: &i32| *x, &[10, 20, 30]), vec![10, 20, 30]);
    }

    #[test]
    fn acc_large_input() {
        let big: Vec<i32> = (0..1_000_000).collect();
        let result = map_acc(&|x: &i32| x * 2, &big);
        assert_eq!(result.len(), 1_000_000);
        assert_eq!(result[0], 0);
        assert_eq!(result[999_999], 1_999_998);
    }

    // --- map_cps ---
    #[test]
    fn cps_empty() {
        assert_eq!(map_cps(&|x: &i32| x * 2, &[]), Vec::<i32>::new());
    }

    #[test]
    fn cps_single() {
        assert_eq!(map_cps(&|x: &i32| x + 1, &[5]), vec![6]);
    }

    #[test]
    fn cps_multiple() {
        assert_eq!(map_cps(&|x: &i32| x * 2, &[1, 2, 3, 4]), vec![2, 4, 6, 8]);
    }

    #[test]
    fn cps_preserves_order() {
        assert_eq!(map_cps(&|x: &i32| *x, &[10, 20, 30]), vec![10, 20, 30]);
    }

    // --- map_idiomatic ---
    #[test]
    fn idiomatic_empty() {
        assert_eq!(map_idiomatic(|x: &i32| x * 2, &[]), Vec::<i32>::new());
    }

    #[test]
    fn idiomatic_single() {
        assert_eq!(map_idiomatic(|x: &i32| x + 1, &[5]), vec![6]);
    }

    #[test]
    fn idiomatic_multiple() {
        assert_eq!(
            map_idiomatic(|x: &i32| x * 2, &[1, 2, 3, 4]),
            vec![2, 4, 6, 8]
        );
    }

    #[test]
    fn idiomatic_type_transform() {
        assert_eq!(
            map_idiomatic(|x: &i32| x.to_string(), &[1, 2, 3]),
            vec!["1", "2", "3"]
        );
    }

    #[test]
    fn idiomatic_large_input() {
        let big: Vec<i32> = (0..1_000_000).collect();
        let result = map_idiomatic(|x: &i32| x * 2, &big);
        assert_eq!(result.len(), 1_000_000);
        assert_eq!(result[0], 0);
        assert_eq!(result[999_999], 1_999_998);
    }

    // --- Cross-implementation consistency ---
    #[test]
    fn all_implementations_agree() {
        let input = [1, 2, 3, 4, 5];
        let f = |x: &i32| x * x;
        let expected = vec![1, 4, 9, 16, 25];
        assert_eq!(map_naive(&f, &input), expected);
        assert_eq!(map_acc(&f, &input), expected);
        assert_eq!(map_cps(&f, &input), expected);
        assert_eq!(map_idiomatic(f, &input), expected);
    }
}
