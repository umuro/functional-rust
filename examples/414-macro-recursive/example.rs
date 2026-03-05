// Recursive macro patterns in Rust

// Reverse a list at compile time using recursion + accumulator
macro_rules! reverse_list {
    // Base case: nothing left, return accumulator
    (@acc [$($acc:expr),*] ) => {
        [$($acc),*]
    };
    // Recursive case: move first element to front of accumulator
    (@acc [$($acc:expr),*] $head:expr $(, $tail:expr)*) => {
        reverse_list!(@acc [$head $(, $acc)*] $($tail),*)
    };
    // Public entry: start with empty accumulator
    ($($x:expr),* $(,)?) => {
        reverse_list!(@acc [] $($x),*)
    };
}

// Count elements recursively
macro_rules! count {
    () => { 0usize };
    ($head:expr $(, $tail:expr)*) => {
        1 + count!($($tail),*)
    };
}

// Build a tuple from a list (up to 4 for simplicity)
macro_rules! tuple_from {
    ($a:expr, $b:expr) => { ($a, $b) };
    ($a:expr, $b:expr, $c:expr) => { ($a, $b, $c) };
    ($a:expr, $b:expr, $c:expr, $d:expr) => { ($a, $b, $c, $d) };
}

// Recursive OR matcher
macro_rules! one_of {
    ($val:expr, $first:expr) => { $val == $first };
    ($val:expr, $first:expr $(, $rest:expr)+) => {
        $val == $first || one_of!($val $(, $rest)+)
    };
}

// Recursive string concatenation
macro_rules! concat_with {
    ($sep:expr; $a:expr) => { $a.to_string() };
    ($sep:expr; $a:expr $(, $rest:expr)+) => {
        format!("{}{}{}", $a, $sep, concat_with!($sep; $($rest),+))
    };
}

fn main() {
    let rev = reverse_list![1, 2, 3, 4, 5];
    println!("reverse [1,2,3,4,5] = {:?}", rev);

    println!("count(1,2,3) = {}", count!(1, 2, 3));
    println!("count() = {}", count!());
    println!("count(a,b,c,d,e) = {}", count!('a', 'b', 'c', 'd', 'e'));

    let t = tuple_from!(1, "hello", 3.14);
    println!("tuple: {:?}", t);

    let x = 5;
    println!("x in {{1,3,5,7}}: {}", one_of!(x, 1, 3, 5, 7));
    println!("x in {{2,4,6}}: {}", one_of!(x, 2, 4, 6));

    println!("{}", concat_with!(", "; "one", "two", "three", "four"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_count() {
        assert_eq!(count!(), 0);
        assert_eq!(count!(a), 1);
        assert_eq!(count!(a, b, c), 3);
    }

    #[test]
    fn test_one_of() {
        assert!(one_of!(3, 1, 2, 3, 4));
        assert!(!one_of!(5, 1, 2, 3));
    }
}
