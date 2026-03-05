// Example 895: Move Semantics — Rust Ownership Transfer
//
// In Rust, values have a single owner. When you pass a value to a function,
// ownership transfers (moves) and the original binding becomes invalid.
// This is how Rust prevents use-after-free at compile time — no GC needed.

// Approach 1: Move with String (heap-allocated, non-Copy)
// Takes ownership of s — caller cannot use s after this call
pub fn consume_string(s: String) -> usize {
    s.len()
}

// Approach 2: Borrow instead of move — caller retains ownership
pub fn borrow_string(s: &str) -> usize {
    s.len()
}

// Approach 3: Move with a struct (non-Copy by default)
#[derive(Debug, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

// Consumes person — ownership transfers into the function
pub fn greet(p: Person) -> String {
    format!("Hello, {} (age {})!", p.name, p.age)
}

// Borrows person — caller retains ownership
pub fn greet_ref(p: &Person) -> String {
    format!("Hello, {} (age {})!", p.name, p.age)
}

// Approach 4: Returning ownership — transfer back to caller
pub fn make_greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

// Approach 5: Move in a closure context
// The closure captures `prefix` by move (it's a String)
pub fn make_prefixer(prefix: String) -> impl Fn(&str) -> String {
    move |s| format!("{}: {}", prefix, s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consume_string_transfers_ownership() {
        let greeting = String::from("Hello, ownership!");
        let len = consume_string(greeting);
        // `greeting` is moved — we cannot use it here anymore.
        // The compiler would reject: consume_string(greeting) a second time.
        assert_eq!(len, 17);
    }

    #[test]
    fn test_borrow_does_not_move() {
        let greeting = String::from("Hello, ownership!");
        // Borrow once
        let len1 = borrow_string(&greeting);
        // greeting still valid — borrow returned ownership implicitly
        let len2 = borrow_string(&greeting);
        assert_eq!(len1, len2);
        assert_eq!(len1, 17);
        // greeting is still usable here
        assert_eq!(greeting, "Hello, ownership!");
    }

    #[test]
    fn test_struct_move_consumes_value() {
        let alice = Person {
            name: String::from("Alice"),
            age: 30,
        };
        let msg = greet(alice);
        // `alice` is moved into greet — no longer accessible here
        assert_eq!(msg, "Hello, Alice (age 30)!");
    }

    #[test]
    fn test_struct_borrow_retains_ownership() {
        let bob = Person {
            name: String::from("Bob"),
            age: 25,
        };
        let msg1 = greet_ref(&bob);
        let msg2 = greet_ref(&bob); // bob still alive
        assert_eq!(msg1, msg2);
        assert_eq!(bob.name, "Bob"); // bob is still usable
    }

    #[test]
    fn test_return_transfers_ownership_to_caller() {
        let msg = make_greeting("World");
        // Caller owns `msg` now
        assert_eq!(msg, "Hello, World!");
    }

    #[test]
    fn test_closure_captures_by_move() {
        let prefix = String::from("LOG");
        let prefixer = make_prefixer(prefix);
        // `prefix` is moved into the closure — no longer usable here
        assert_eq!(prefixer("info message"), "LOG: info message");
        assert_eq!(prefixer("error message"), "LOG: error message");
    }

    #[test]
    fn test_copy_types_do_not_move() {
        // i32 implements Copy — assignment copies, not moves
        let x: i32 = 42;
        let y = x; // copy, not move
        assert_eq!(x, 42); // x still valid
        assert_eq!(y, 42);
    }
}
