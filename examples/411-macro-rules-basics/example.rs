// macro_rules! declarative macros in Rust

// Simple logging macro
macro_rules! log_info {
    ($msg:expr) => {
        println!("[INFO] {}", $msg)
    };
    ($fmt:expr, $($arg:expr),*) => {
        println!(concat!("[INFO] ", $fmt), $($arg),*)
    };
}

// assert with custom message (like assert_eq! but custom)
macro_rules! check_eq {
    ($left:expr, $right:expr) => {
        if $left != $right {
            panic!("check_eq failed: {} != {}", $left, $right);
        }
    };
    ($left:expr, $right:expr, $msg:expr) => {
        if $left != $right {
            panic!("check_eq failed ({}): {} != {}", $msg, $left, $right);
        }
    };
}

// repeat macro
macro_rules! repeat {
    ($n:expr, $body:block) => {
        for _ in 0..$n $body
    };
}

// min/max for any number of args
macro_rules! min_of {
    ($a:expr) => { $a };
    ($a:expr, $($rest:expr),+) => {
        {
            let rest_min = min_of!($($rest),+);
            if $a < rest_min { $a } else { rest_min }
        }
    };
}

macro_rules! max_of {
    ($a:expr) => { $a };
    ($a:expr, $($rest:expr),+) => {
        {
            let rest_max = max_of!($($rest),+);
            if $a > rest_max { $a } else { rest_max }
        }
    };
}

// Map literal macro
macro_rules! map {
    ($($k:expr => $v:expr),* $(,)?) => {
        {
            let mut m = std::collections::HashMap::new();
            $(m.insert($k, $v);)*
            m
        }
    };
}

fn main() {
    log_info!("Application started");
    log_info!("User {} logged in at port {}", "Alice", 8080);

    check_eq!(2 + 2, 4, "basic arithmetic");
    check_eq!("hello".len(), 5);

    print!("Repeating: ");
    repeat!(3, { print!("Ho "); });
    println!();

    println!("min(3,7,1,9) = {}", min_of!(3, 7, 1, 9));
    println!("max(3,7,1,9) = {}", max_of!(3, 7, 1, 9));

    let m = map! {
        "one" => 1,
        "two" => 2,
        "three" => 3,
    };
    println!("Map: {:?}", m);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_min_max() {
        assert_eq!(min_of!(5, 3, 7, 1, 9), 1);
        assert_eq!(max_of!(5, 3, 7, 1, 9), 9);
    }

    #[test]
    fn test_map_macro() {
        let m = map! { "a" => 1, "b" => 2 };
        assert_eq!(m["a"], 1);
        assert_eq!(m["b"], 2);
    }

    #[test]
    #[should_panic]
    fn test_check_eq_fails() {
        check_eq!(1, 2);
    }
}
