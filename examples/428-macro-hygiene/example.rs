// Macro hygiene rules in Rust

// Hygienic: the 'tmp' inside this macro won't conflict with caller's 'tmp'
macro_rules! swap {
    ($a:expr, $b:expr) => {
        {
            let tmp = $a;  // 'tmp' is in the macro's hygiene context
            $a = $b;
            $b = tmp;
        }
    };
}

// Demonstrate hygiene: caller has a variable 'tmp' — it's safe!
fn demonstrate_hygiene() {
    let tmp = "I am the caller's tmp"; // this is the CALLER's tmp

    let mut x = 1;
    let mut y = 2;
    swap!(x, y); // The macro's internal 'tmp' doesn't affect caller's 'tmp'

    println!("After swap: x={}, y={}", x, y);
    println!("Caller's tmp is still: {}", tmp); // unchanged!
}

// Hygiene with ident fragments: identifiers from caller ARE in caller's context
macro_rules! make_counter {
    ($name:ident) => {
        let mut $name = 0u32; // $name is in CALLER's context (from caller)
        // 'count' inside would be hygienic, but $name comes from caller
    };
}

// Non-hygienic case with proc macros (shown conceptually):
// In proc macros, you can choose span: call_site() or def_site()
// call_site() = caller's context (like $ident in macro_rules!)
// def_site() = macro's context (hygienic)

macro_rules! log_and_double {
    ($x:expr) => {
        {
            // 'result' here is hygienic — won't clash with caller's 'result'
            let result = $x * 2;
            println!("log_and_double({}) = {}", stringify!($x), result);
            result
        }
    };
}

macro_rules! with_temp {
    ($val:expr, |$v:ident| $body:expr) => {
        {
            let $v = $val; // $v comes from caller — in caller's scope
            $body
        }
    };
}

fn main() {
    demonstrate_hygiene();

    println!("
=== make_counter ===");
    make_counter!(hits); // introduces 'hits' in caller's scope
    hits += 1;
    hits += 1;
    println!("hits: {}", hits);

    println!("
=== log_and_double ===");
    let result = 42; // caller has 'result' — macro's 'result' is different!
    let doubled = log_and_double!(21);
    println!("caller's result: {}, doubled: {}", result, doubled);

    println!("
=== with_temp ===");
    let output = with_temp!(vec![1,2,3], |v| v.len());
    println!("output: {}", output);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_swap_hygiene() {
        let tmp = "safe"; // same name as macro internal
        let mut a = 1;
        let mut b = 2;
        swap!(a, b);
        assert_eq!(a, 2);
        assert_eq!(b, 1);
        assert_eq!(tmp, "safe"); // unaffected
    }

    #[test]
    fn test_log_and_double() {
        let result = 100; // same name as macro internal
        let doubled = log_and_double!(7);
        assert_eq!(doubled, 14);
        assert_eq!(result, 100); // unaffected
    }
}
