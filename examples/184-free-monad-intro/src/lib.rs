#![allow(clippy::all)]
// Example 184: Introduction to Free Monads
// Separate program description from interpretation

// === Approach 1: Free monad as enum ===

enum Console<A> {
    Pure(A),
    Print(String, Box<Console<A>>),
    GetLine(Box<dyn FnOnce(String) -> Console<A>>),
}

impl<A> Console<A> {
    fn pure(a: A) -> Self {
        Console::Pure(a)
    }
}

fn console_print(msg: &str) -> Console<()> {
    Console::Print(msg.to_string(), Box::new(Console::Pure(())))
}

fn console_get_line() -> Console<String> {
    Console::GetLine(Box::new(Console::Pure))
}

// bind (flatmap) — chain free monad computations
fn bind<A: 'static, B: 'static>(
    ma: Console<A>,
    f: impl FnOnce(A) -> Console<B> + 'static,
) -> Console<B> {
    match ma {
        Console::Pure(a) => f(a),
        Console::Print(msg, next) => Console::Print(msg, Box::new(bind(*next, f))),
        Console::GetLine(k) => Console::GetLine(Box::new(move |s| bind(k(s), f))),
    }
}

// === Approach 2: Pure interpreter (collects output, feeds input) ===

fn interpret_pure(inputs: &[&str], prog: Console<String>) -> (Vec<String>, String) {
    let mut outputs = Vec::new();
    let mut input_idx = 0;
    let mut current = prog;

    loop {
        match current {
            Console::Pure(a) => return (outputs, a),
            Console::Print(msg, next) => {
                outputs.push(msg);
                current = *next;
            }
            Console::GetLine(k) => {
                let input = inputs.get(input_idx).unwrap_or(&"");
                input_idx += 1;
                current = k(input.to_string());
            }
        }
    }
}

// === Approach 3: Builder-style DSL with macro ===

fn greet_program() -> Console<String> {
    bind(console_print("What is your name?"), move |()| {
        bind(console_get_line(), move |name: String| {
            bind(console_print(&format!("Hello, {}!", name)), move |()| {
                Console::pure(name)
            })
        })
    })
}

// Alternate: sequence of operations without result threading
fn log_program(messages: Vec<String>) -> Console<()> {
    let mut prog = Console::pure(());
    // Build in reverse
    for msg in messages.into_iter().rev() {
        let next = prog;
        prog = Console::Print(msg, Box::new(next));
    }
    prog
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pure() {
        let (outputs, result) = interpret_pure(&[], Console::pure("hello".to_string()));
        assert!(outputs.is_empty());
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_greet_alice() {
        let (outputs, result) = interpret_pure(&["Alice"], greet_program());
        assert_eq!(outputs, vec!["What is your name?", "Hello, Alice!"]);
        assert_eq!(result, "Alice");
    }

    #[test]
    fn test_greet_bob() {
        let (outputs, result) = interpret_pure(&["Bob"], greet_program());
        assert_eq!(outputs, vec!["What is your name?", "Hello, Bob!"]);
        assert_eq!(result, "Bob");
    }

    #[test]
    fn test_print_only() {
        let prog = bind(console_print("hi"), |()| Console::pure("done".to_string()));
        let (outputs, result) = interpret_pure(&[], prog);
        assert_eq!(outputs, vec!["hi"]);
        assert_eq!(result, "done");
    }

    #[test]
    fn test_get_line() {
        let prog = bind(console_get_line(), |s: String| Console::pure(s));
        let (_, result) = interpret_pure(&["test"], prog);
        assert_eq!(result, "test");
    }
}
