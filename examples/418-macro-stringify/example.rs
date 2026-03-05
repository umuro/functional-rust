// stringify! and concat! in Rust

// stringify! captures the token text as a string literal
macro_rules! show_expr {
    ($e:expr) => {
        println!("{} = {}", stringify!($e), $e)
    };
}

// Field names from identifiers
macro_rules! field_name {
    ($field:ident) => { stringify!($field) };
}

// Generate test names with concat!
macro_rules! test_case {
    ($name:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            assert_eq!($input, $expected,
                concat!("Test '", stringify!($name), "' failed"));
        }
    };
}

// Concatenate at compile time
const VERSION: &str = concat!(
    env!("CARGO_PKG_NAME", "unknown"),
    " v",
    "1.0.0",
    " (",
    file!(),
    ")"
);

// Build SQL-like strings at compile time
macro_rules! select {
    ($table:ident . $($col:ident),+) => {
        concat!(
            "SELECT ",
            concat_fields!($($col),+),
            " FROM ",
            stringify!($table)
        )
    };
}

macro_rules! concat_fields {
    ($f:ident) => { stringify!($f) };
    ($f:ident, $($rest:ident),+) => {
        concat!(stringify!($f), ", ", concat_fields!($($rest),+))
    };
}

// Location macro
macro_rules! here {
    () => {
        concat!(file!(), ":", line!(), ":", column!())
    };
}

fn main() {
    // show_expr
    show_expr!(2 + 3 * 4);
    show_expr!("hello".len());
    show_expr!(vec![1,2,3].len());

    // field names
    println!("Field name: {}", field_name!(user_id));
    println!("Field name: {}", field_name!(created_at));

    // compile-time version string
    println!("Version: {}", VERSION);

    // SQL DSL
    let sql = select!(users.id, name, email);
    println!("SQL: {}", sql);

    // Location
    println!("Called at: {}", here!());

    // stringify vs evaluate
    let x = 42;
    println!("stringify!(x) = {}", stringify!(x));       // "x", not "42"
    println!("stringify!(1+1) = {}", stringify!(1 + 1)); // "1 + 1", not "2"

    // concat! at compile time
    const HELLO_WORLD: &str = concat!("Hello", ", ", "World", "!");
    println!("{}", HELLO_WORLD);
}

test_case!(addition_works, 2 + 2, 4);
test_case!(string_len, "rust".len(), 4);

#[cfg(test)]
mod tests {
    #[test]
    fn test_stringify() {
        assert_eq!(stringify!(my_var), "my_var");
        assert_eq!(stringify!(1 + 1), "1 + 1");
    }

    #[test]
    fn test_concat() {
        const S: &str = concat!("foo", "bar", "baz");
        assert_eq!(S, "foobarbaz");
    }
}
