/// Option.iter equivalent: run a side effect only if Some.
/// Idiomatic Rust uses `if let` or `Option::iter().for_each`.
pub fn greet_if_present(name: Option<&str>) -> Vec<String> {
    let mut log = Vec::new();
    // Option::iter() yields 0 or 1 items — mirrors OCaml's Option.iter
    name.iter().for_each(|n| log.push(format!("Hello, {}!", n)));
    log
}

/// Option.iter with `if let` — the most natural Rust idiom for conditional side effects.
pub fn greet_if_present_iflet(name: Option<&str>) -> Vec<String> {
    let mut log = Vec::new();
    if let Some(n) = name {
        log.push(format!("Hello, {}!", n));
    }
    log
}

/// Option.fold equivalent: collapse an Option to a value with a default.
/// `Option::map_or(default, f)` is the direct translation of OCaml's
/// `Option.fold ~none:default ~some:f`.
pub fn greeting(name: Option<&str>) -> String {
    // map_or: if None → first arg; if Some(v) → apply closure to v
    name.map_or_else(
        || "Hello, stranger!".to_owned(),
        |n| format!("Hello, {}!", n),
    )
}

/// Functional/recursive style: explicit pattern match mirrors OCaml's match.
pub fn greeting_match(name: Option<&str>) -> String {
    match name {
        None => "Hello, stranger!".to_owned(),
        Some(n) => format!("Hello, {}!", n),
    }
}

/// Generic fold over Option — mirrors the generalised OCaml Option.fold signature.
/// `Option::fold` is not in std, but we can express it with `map_or`.
pub fn option_fold<T, U>(opt: Option<T>, none: U, some: impl FnOnce(T) -> U) -> U {
    opt.map_or_else(|| none, some)
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- greet_if_present (Option::iter) ---

    #[test]
    fn iter_some_produces_greeting() {
        assert_eq!(greet_if_present(Some("Alice")), vec!["Hello, Alice!"]);
    }

    #[test]
    fn iter_none_produces_nothing() {
        let result: Vec<String> = greet_if_present(None);
        assert!(result.is_empty());
    }

    #[test]
    fn iter_iflet_some() {
        assert_eq!(greet_if_present_iflet(Some("Bob")), vec!["Hello, Bob!"]);
    }

    #[test]
    fn iter_iflet_none_is_empty() {
        assert!(greet_if_present_iflet(None).is_empty());
    }

    // --- greeting (Option::map_or — Option.fold) ---

    #[test]
    fn fold_some_returns_personalised() {
        assert_eq!(greeting(Some("Alice")), "Hello, Alice!");
    }

    #[test]
    fn fold_none_returns_default() {
        assert_eq!(greeting(None), "Hello, stranger!");
    }

    #[test]
    fn fold_match_some() {
        assert_eq!(greeting_match(Some("Carol")), "Hello, Carol!");
    }

    #[test]
    fn fold_match_none() {
        assert_eq!(greeting_match(None), "Hello, stranger!");
    }

    // --- generic option_fold ---

    #[test]
    fn generic_fold_some() {
        assert_eq!(option_fold(Some(42), 0, |x| x * 2), 84);
    }

    #[test]
    fn generic_fold_none_returns_default() {
        assert_eq!(option_fold(None::<i32>, 0, |x| x * 2), 0);
    }

    #[test]
    fn generic_fold_string() {
        let result = option_fold(Some("world"), "nothing".to_owned(), |s| {
            format!("hello {s}")
        });
        assert_eq!(result, "hello world");
    }
}
