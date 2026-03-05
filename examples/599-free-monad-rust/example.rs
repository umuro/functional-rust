// Free monad: program as data structure
#[derive(Debug)]
enum ConsoleF<A> {
    Print(String, A),
    ReadLine(Box<dyn Fn(String) -> A>),
}

enum Free<A> {
    Pure(A),
    Free(ConsoleF<Box<Free<A>>>),
}

// Smart constructors
fn print_line(s: impl Into<String>) -> Free<()> {
    Free::Free(ConsoleF::Print(s.into(), Box::new(Free::Pure(()))))
}

fn read_line() -> Free<String> {
    Free::Free(ConsoleF::ReadLine(Box::new(|s| Free::Pure(s))))
}

// Simplified program type for clarity
#[derive(Debug)]
enum Prog {
    Done,
    Print(String, Box<Prog>),
    Read(Box<dyn Fn(String) -> Prog>),
}

fn print_prog(s: impl Into<String>, next: Prog) -> Prog {
    Prog::Print(s.into(), Box::new(next))
}

fn read_prog(f: impl Fn(String) -> Prog + 'static) -> Prog {
    Prog::Read(Box::new(f))
}

// Production interpreter: real I/O
fn run_io(prog: Prog) {
    match prog {
        Prog::Done => {}
        Prog::Print(s, next) => { println!("{}", s); run_io(*next); }
        Prog::Read(f) => {
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).ok();
            run_io(f(buf.trim().to_string()));
        }
    }
}

// Test interpreter: pure, no I/O
fn run_test(prog: Prog, mut inputs: Vec<String>) -> Vec<String> {
    let mut outputs = Vec::new();
    let mut current = prog;
    loop {
        current = match current {
            Prog::Done => break,
            Prog::Print(s, next) => { outputs.push(s); *next }
            Prog::Read(f) => {
                let input = inputs.remove(0);
                f(input)
            }
        };
    }
    outputs
}

fn make_program() -> Prog {
    print_prog("What is your name?",
    read_prog(|name|
    print_prog(format!("Hello, {}!", name),
    read_prog(|age_str| {
        let age: u32 = age_str.parse().unwrap_or(0);
        print_prog(format!("In 10 years you'll be {}.", age+10), Prog::Done)
    }))))
}

fn main() {
    let prog = make_program();
    let outputs = run_test(prog, vec!["Alice".into(), "30".into()]);
    for o in &outputs { println!("{}", o); }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_interpreter() {
        let prog = make_program();
        let out = run_test(prog, vec!["Bob".into(), "25".into()]);
        assert!(out[1].contains("Bob"));
        assert!(out[2].contains("35"));
    }
}
