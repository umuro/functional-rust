// Example 185: Console DSL with Free Monad
// Print, ReadLine, Exit operations as a domain-specific language

// === Approach 1: Console DSL enum ===

enum Console<A> {
    Pure(A),
    Print(String, Box<dyn FnOnce() -> Console<A>>),
    ReadLine(Box<dyn FnOnce(String) -> Console<A>>),
    Exit(i32),
}

fn pure<A>(a: A) -> Console<A> {
    Console::Pure(a)
}

fn print_line(msg: &str) -> Console<()> {
    let msg = msg.to_string();
    Console::Print(msg, Box::new(|| Console::Pure(())))
}

fn read_line_dsl() -> Console<String> {
    Console::ReadLine(Box::new(|s| Console::Pure(s)))
}

fn exit_prog<A>(code: i32) -> Console<A> {
    Console::Exit(code)
}

fn bind<A: 'static, B: 'static>(
    ma: Console<A>,
    f: impl FnOnce(A) -> Console<B> + 'static,
) -> Console<B> {
    match ma {
        Console::Pure(a) => f(a),
        Console::Print(msg, k) => Console::Print(msg, Box::new(move || bind(k(), f))),
        Console::ReadLine(k) => Console::ReadLine(Box::new(move |s| bind(k(s), f))),
        Console::Exit(code) => Console::Exit(code),
    }
}

// === Approach 2: Menu program ===

fn menu_program() -> Console<String> {
    bind(print_line("=== Menu ==="), move |()| {
        bind(print_line("1. Greet"), move |()| {
            bind(print_line("2. Exit"), move |()| {
                bind(print_line("Choose: "), move |()| {
                    bind(read_line_dsl(), move |choice: String| {
                        match choice.as_str() {
                            "1" => bind(print_line("Enter name: "), move |()| {
                                bind(read_line_dsl(), move |name: String| {
                                    bind(print_line(&format!("Hello, {}!", name)), move |()| {
                                        pure(format!("greeted {}", name))
                                    })
                                })
                            }),
                            "2" => exit_prog(0),
                            _ => bind(print_line("Invalid choice"), |()| pure("error".to_string())),
                        }
                    })
                })
            })
        })
    })
}

// === Approach 3: Pure test interpreter ===

#[derive(Debug, PartialEq)]
enum ProgramResult<A> {
    Ok(A),
    Exited(i32),
}

fn interpret_pure(inputs: &[&str], prog: Console<String>) -> (Vec<String>, ProgramResult<String>) {
    let mut outputs = Vec::new();
    let mut idx = 0;
    let mut current = prog;

    loop {
        match current {
            Console::Pure(a) => return (outputs, ProgramResult::Ok(a)),
            Console::Print(msg, k) => {
                outputs.push(msg);
                current = k();
            }
            Console::ReadLine(k) => {
                let input = inputs.get(idx).unwrap_or(&"").to_string();
                idx += 1;
                current = k(input);
            }
            Console::Exit(code) => return (outputs, ProgramResult::Exited(code)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_path() {
        let (out, result) = interpret_pure(&["1", "Alice"], menu_program());
        assert_eq!(
            out,
            vec![
                "=== Menu ===",
                "1. Greet",
                "2. Exit",
                "Choose: ",
                "Enter name: ",
                "Hello, Alice!"
            ]
        );
        assert_eq!(result, ProgramResult::Ok("greeted Alice".to_string()));
    }

    #[test]
    fn test_exit_path() {
        let (out, result) = interpret_pure(&["2"], menu_program());
        assert_eq!(out.len(), 4);
        assert_eq!(result, ProgramResult::Exited(0));
    }

    #[test]
    fn test_invalid_path() {
        let (out, result) = interpret_pure(&["x"], menu_program());
        assert!(out.contains(&"Invalid choice".to_string()));
        assert_eq!(result, ProgramResult::Ok("error".to_string()));
    }

    #[test]
    fn test_simple_print() {
        let prog = bind(print_line("hi"), |()| pure("done".to_string()));
        let (out, result) = interpret_pure(&[], prog);
        assert_eq!(out, vec!["hi"]);
        assert_eq!(result, ProgramResult::Ok("done".to_string()));
    }

    #[test]
    fn test_simple_read() {
        let prog = bind(read_line_dsl(), |s: String| pure(s));
        let (_, result) = interpret_pure(&["test"], prog);
        assert_eq!(result, ProgramResult::Ok("test".to_string()));
    }
}
