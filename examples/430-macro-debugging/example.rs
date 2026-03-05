// Debugging macros with trace_macros! and cargo expand

// trace_macros!(true); // Nightly only — prints each expansion step
// trace_macros!(false); // Turn off

// A macro with a bug we'll debug
macro_rules! buggy_example {
    // Missing: what if list is empty?
    ($head:expr, $($tail:expr),+) => {
        $head + buggy_example!($($tail),+)
    };
    ($only:expr) => { $only }; // base case
}

// Debugging technique 1: add compile_error! to print matched arm
macro_rules! debug_which_arm {
    () => { compile_error!("matched empty arm") };
    ($single:expr) => { $single };
    ($a:expr, $b:expr) => { $a + $b };
    ($($x:expr),+) => { "more than 2" };
}

// Debugging technique 2: stringify! to see what's being matched
macro_rules! trace_input {
    ($($x:tt)*) => {
        {
            println!("Macro received: {}", stringify!($($x)*));
        }
    };
}

// Debugging technique 3: step through expansion manually
// Imagine: my_macro!(1, 2, 3)
// Step 1: matches ($head:expr, $($tail:expr),+) with head=1, tail=2,3
// Step 2: expands to 1 + my_macro!(2, 3)
// Step 3: matches ($head:expr, $($tail:expr),+) with head=2, tail=3
// Step 4: expands to 1 + (2 + my_macro!(3))
// Step 5: matches ($only:expr) with only=3
// Result: 1 + (2 + 3) = 6

// Well-designed macro for comparison
macro_rules! sum {
    () => { 0 };
    ($head:expr $(, $tail:expr)*) => {
        $head + sum!($($tail),*)
    };
}

// Macro that validates input count
macro_rules! exactly_two {
    ($a:expr, $b:expr) => { ($a, $b) };
    ($($other:tt)*) => {
        compile_error!(concat!(
            "exactly_two! requires exactly 2 args, got: ",
            stringify!($($other)*)
        ))
    };
}

fn main() {
    // Working macro
    println!("buggy_example!(1, 2, 3) = {}", buggy_example!(1, 2, 3));

    // trace_input shows what was passed
    trace_input!(hello world 42);
    trace_input!(1 + 2, "foo");

    // sum macro
    println!("sum!() = {}", sum!());
    println!("sum!(5) = {}", sum!(5));
    println!("sum!(1, 2, 3, 4) = {}", sum!(1, 2, 3, 4));

    // exactly_two
    let (a, b) = exactly_two!(10, 20);
    println!("exactly_two: {}, {}", a, b);

    // Cargo expand command (not runtime, but shown for reference):
    println!("
Debugging tools:");
    println!("  cargo install cargo-expand");
    println!("  cargo expand -- 2>&1 | head -100");
    println!("  RUSTFLAGS='-Z trace-macros' cargo build (nightly)");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sum() {
        assert_eq!(sum!(), 0);
        assert_eq!(sum!(5), 5);
        assert_eq!(sum!(1, 2, 3), 6);
        assert_eq!(sum!(1, 2, 3, 4, 5), 15);
    }

    #[test]
    fn test_buggy_becomes_correct() {
        assert_eq!(buggy_example!(10, 20, 30), 60);
    }
}
