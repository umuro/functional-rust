// Counting elements at compile time in Rust

// Basic counting macro
macro_rules! count {
    () => { 0usize };
    ($head:tt $($tail:tt)*) => { 1 + count!($($tail)*) };
}

// More efficient: replace each element with 1 using substitution
macro_rules! count_tts {
    ($($tts:tt)*) => {
        <[()]>::len(&[$(replace_with_unit!($tts)),*])
    };
}

macro_rules! replace_with_unit {
    ($anything:tt) => { () };
}

// Count expressions (cleaner for expr lists)
macro_rules! count_exprs {
    () => { 0usize };
    ($e:expr $(, $rest:expr)*) => { 1 + count_exprs!($($rest),*) };
}

// Create a fixed-size array from variadic arguments
macro_rules! fixed_array {
    ($($val:expr),* $(,)?) => {
        {
            const N: usize = count_exprs!($($val),*);
            let arr: [i32; N] = [$($val,)*];
            arr
        }
    };
}

// Compile-time length check
macro_rules! assert_count {
    ($expected:literal; $($items:expr),*) => {
        const _: () = assert!(
            count_exprs!($($items),*) == $expected,
            "Expected item count mismatch!"
        );
    };
}

// Generate static dispatch table with known size
macro_rules! dispatch_table {
    ($($name:ident : $fn:expr),* $(,)?) => {
        {
            const SIZE: usize = count_exprs!($($fn),*);
            let names: [&str; SIZE] = [$(stringify!($name),)*];
            let funcs: [fn(i32) -> i32; SIZE] = [$($fn,)*];
            (names, funcs)
        }
    };
}

fn main() {
    // Basic counting
    println!("count!(a b c d e) = {}", count!(a b c d e));
    println!("count!() = {}", count!());
    println!("count_exprs!(1,2,3) = {}", count_exprs!(1, 2, 3));

    // Fixed array
    let arr = fixed_array![10, 20, 30, 40, 50];
    println!("fixed_array type=[i32; {}]: {:?}", arr.len(), arr);

    // Dispatch table
    let (names, funcs) = dispatch_table!(
        double: |x| x * 2,
        square: |x| x * x,
        negate: |x| -x,
    );
    println!("
Dispatch table ({} entries):", names.len());
    for (name, f) in names.iter().zip(funcs.iter()) {
        println!("  {}(5) = {}", name, f(5));
    }

    // Compile-time assertion
    assert_count!(3; 1, 2, 3);
    println!("Count assertions passed");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_count() {
        assert_eq!(count!(), 0);
        assert_eq!(count!(a), 1);
        assert_eq!(count!(a b c d e), 5);
    }

    #[test]
    fn test_count_exprs() {
        assert_eq!(count_exprs!(), 0);
        assert_eq!(count_exprs!(1, 2, 3), 3);
    }

    #[test]
    fn test_fixed_array() {
        let a = fixed_array![1, 2, 3];
        assert_eq!(a.len(), 3);
        assert_eq!(a[0], 1);
    }
}
