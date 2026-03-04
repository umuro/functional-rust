#[derive(Debug, PartialEq, Eq)]
pub enum Relation {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

// Solution 1: Idiomatic Rust — uses slice windows for contiguous sublist check
pub fn classify_idiomatic<T: PartialEq>(a: &[T], b: &[T]) -> Relation {
    if a == b {
        Relation::Equal
    } else if is_sublist_idiomatic(a, b) {
        Relation::Sublist
    } else if is_sublist_idiomatic(b, a) {
        Relation::Superlist
    } else {
        Relation::Unequal
    }
}

fn is_sublist_idiomatic<T: PartialEq>(sub: &[T], lst: &[T]) -> bool {
    if sub.is_empty() {
        return true;
    }
    lst.windows(sub.len()).any(|w| w == sub)
}

// Solution 2: Functional/recursive — mirrors the OCaml pattern-match style
pub fn classify_recursive<T: PartialEq>(a: &[T], b: &[T]) -> Relation {
    if a == b {
        Relation::Equal
    } else if is_sublist_recursive(a, b) {
        Relation::Sublist
    } else if is_sublist_recursive(b, a) {
        Relation::Superlist
    } else {
        Relation::Unequal
    }
}

fn starts_with<T: PartialEq>(lst: &[T], prefix: &[T]) -> bool {
    match (lst, prefix) {
        (_, []) => true,
        ([], _) => false,
        ([h1, t1 @ ..], [h2, t2 @ ..]) => h1 == h2 && starts_with(t1, t2),
    }
}

fn is_sublist_recursive<T: PartialEq>(sub: &[T], lst: &[T]) -> bool {
    match lst {
        [] => sub.is_empty(),
        [_, rest @ ..] => starts_with(lst, sub) || is_sublist_recursive(sub, rest),
    }
}

fn name(r: &Relation) -> &'static str {
    match r {
        Relation::Equal => "equal",
        Relation::Sublist => "sublist",
        Relation::Superlist => "superlist",
        Relation::Unequal => "unequal",
    }
}

fn main() {
    let cases: &[(&[i32], &[i32])] = &[
        (&[1, 2, 3], &[0, 1, 2, 3, 4]),
        (&[0, 1, 2, 3, 4], &[1, 2, 3]),
        (&[1, 2, 3], &[1, 2, 3]),
        (&[1, 2, 3], &[4, 5, 6]),
        (&[], &[1, 2, 3]),
    ];

    println!("=== Idiomatic (windows) ===");
    for (a, b) in cases {
        println!("{:?} vs {:?} → {}", a, b, name(&classify_idiomatic(a, b)));
    }

    println!("\n=== Recursive (OCaml-style) ===");
    for (a, b) in cases {
        println!("{:?} vs {:?} → {}", a, b, name(&classify_recursive(a, b)));
    }
}

/* Output:
   === Idiomatic (windows) ===
   [1, 2, 3] vs [0, 1, 2, 3, 4] → sublist
   [0, 1, 2, 3, 4] vs [1, 2, 3] → superlist
   [1, 2, 3] vs [1, 2, 3] → equal
   [1, 2, 3] vs [4, 5, 6] → unequal
   [] vs [1, 2, 3] → sublist

   === Recursive (OCaml-style) ===
   [1, 2, 3] vs [0, 1, 2, 3, 4] → sublist
   [0, 1, 2, 3, 4] vs [1, 2, 3] → superlist
   [1, 2, 3] vs [1, 2, 3] → equal
   [1, 2, 3] vs [4, 5, 6] → unequal
   [] vs [1, 2, 3] → sublist
*/
