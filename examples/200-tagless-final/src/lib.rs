// Tagless Final: embed DSLs as trait methods — no intermediate AST.
// Multiple interpreters share one program definition.

// =============================================================
// Solution 1: Tagless Final — Generic Associated Types (GATs)
// Direct translation of OCaml's module type with type constructor
// =============================================================

/// The DSL signature — written once, interpreted many ways.
///
/// `Repr<T>` is the "representation" type: what this interpreter
/// produces for a value of type `T`.  For `Eval`, `Repr<T> = T`
/// (the value itself).  For `Pretty`, `Repr<T> = String` (always
/// a string, regardless of `T`).
pub trait Expr {
    type Repr<T>;

    fn int(n: i64) -> Self::Repr<i64>;
    fn bool_val(b: bool) -> Self::Repr<bool>;
    fn add(a: Self::Repr<i64>, b: Self::Repr<i64>) -> Self::Repr<i64>;
    fn mul(a: Self::Repr<i64>, b: Self::Repr<i64>) -> Self::Repr<i64>;
    fn leq(a: Self::Repr<i64>, b: Self::Repr<i64>) -> Self::Repr<bool>;
    fn if_<T>(c: Self::Repr<bool>, t: Self::Repr<T>, e: Self::Repr<T>) -> Self::Repr<T>;
}

// --- Interpreter 1: Evaluate ---
// Repr<T> = T — the representation IS the value.
pub struct Eval;

impl Expr for Eval {
    type Repr<T> = T;

    fn int(n: i64) -> i64 {
        n
    }
    fn bool_val(b: bool) -> bool {
        b
    }
    fn add(a: i64, b: i64) -> i64 {
        a + b
    }
    fn mul(a: i64, b: i64) -> i64 {
        a * b
    }
    fn leq(a: i64, b: i64) -> bool {
        a <= b
    }
    fn if_<T>(c: bool, t: T, e: T) -> T {
        if c {
            t
        } else {
            e
        }
    }
}

// --- Interpreter 2: Pretty-print ---
// Repr<T> = String — always produces a string, ignoring T.
pub struct Pretty;

impl Expr for Pretty {
    type Repr<T> = String;

    fn int(n: i64) -> String {
        n.to_string()
    }
    fn bool_val(b: bool) -> String {
        b.to_string()
    }
    fn add(a: String, b: String) -> String {
        format!("({a} + {b})")
    }
    fn mul(a: String, b: String) -> String {
        format!("({a} * {b})")
    }
    fn leq(a: String, b: String) -> String {
        format!("({a} <= {b})")
    }
    fn if_<T>(c: String, t: String, e: String) -> String {
        format!("(if {c} then {t} else {e})")
    }
}

/// The program — written once, run through any interpreter.
///
/// Encodes: `if (3 + 4) <= (2 * 5) then 42 else 0`
pub fn program<E: Expr>() -> E::Repr<i64> {
    E::if_(
        E::leq(E::add(E::int(3), E::int(4)), E::mul(E::int(2), E::int(5))),
        E::int(42),
        E::int(0),
    )
}

// =============================================================
// Solution 2: Initial Encoding — explicit AST for comparison
// Adding a new interpreter requires modifying eval_ast here.
// Tagless final avoids that: new interpreter = new impl block.
// =============================================================

#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    Int(i64),
    Bool(bool),
    Add(Box<Ast>, Box<Ast>),
    Mul(Box<Ast>, Box<Ast>),
    Leq(Box<Ast>, Box<Ast>),
    If(Box<Ast>, Box<Ast>, Box<Ast>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Bool(bool),
}

/// Evaluate an AST to a `Value`.
pub fn eval_ast(ast: &Ast) -> Value {
    match ast {
        Ast::Int(n) => Value::Int(*n),
        Ast::Bool(b) => Value::Bool(*b),
        Ast::Add(a, b) => match (eval_ast(a), eval_ast(b)) {
            (Value::Int(x), Value::Int(y)) => Value::Int(x + y),
            _ => panic!("type error: add requires integers"),
        },
        Ast::Mul(a, b) => match (eval_ast(a), eval_ast(b)) {
            (Value::Int(x), Value::Int(y)) => Value::Int(x * y),
            _ => panic!("type error: mul requires integers"),
        },
        Ast::Leq(a, b) => match (eval_ast(a), eval_ast(b)) {
            (Value::Int(x), Value::Int(y)) => Value::Bool(x <= y),
            _ => panic!("type error: leq requires integers"),
        },
        Ast::If(c, t, e) => match eval_ast(c) {
            Value::Bool(true) => eval_ast(t),
            Value::Bool(false) => eval_ast(e),
            _ => panic!("type error: if condition must be bool"),
        },
    }
}

/// The same program built with the initial (AST) encoding.
pub fn program_ast() -> Ast {
    Ast::If(
        Box::new(Ast::Leq(
            Box::new(Ast::Add(Box::new(Ast::Int(3)), Box::new(Ast::Int(4)))),
            Box::new(Ast::Mul(Box::new(Ast::Int(2)), Box::new(Ast::Int(5)))),
        )),
        Box::new(Ast::Int(42)),
        Box::new(Ast::Int(0)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Tagless Final ---

    #[test]
    fn test_eval_program() {
        // (3+4)=7, (2*5)=10, 7<=10 → true → result=42
        assert_eq!(program::<Eval>(), 42);
    }

    #[test]
    fn test_pretty_program() {
        assert_eq!(
            program::<Pretty>(),
            "(if ((3 + 4) <= (2 * 5)) then 42 else 0)"
        );
    }

    #[test]
    fn test_eval_false_branch() {
        // (1+1)=2, (0*5)=0, 2<=0 → false → result=0
        let result = Eval::if_(
            Eval::leq(
                Eval::add(Eval::int(1), Eval::int(1)),
                Eval::mul(Eval::int(0), Eval::int(5)),
            ),
            Eval::int(99),
            Eval::int(0),
        );
        assert_eq!(result, 0);
    }

    #[test]
    fn test_pretty_primitives() {
        assert_eq!(Pretty::int(42), "42");
        assert_eq!(Pretty::bool_val(true), "true");
        assert_eq!(Pretty::add("3".to_string(), "4".to_string()), "(3 + 4)");
        assert_eq!(Pretty::leq("7".to_string(), "10".to_string()), "(7 <= 10)");
    }

    #[test]
    fn test_eval_arithmetic() {
        assert_eq!(Eval::add(Eval::int(10), Eval::int(32)), 42);
        assert_eq!(Eval::mul(Eval::int(6), Eval::int(7)), 42);
        assert_eq!(Eval::leq(Eval::int(5), Eval::int(5)), true);
    }

    // --- Initial encoding ---

    #[test]
    fn test_ast_program_matches_tagless() {
        let tagless = program::<Eval>();
        let initial = eval_ast(&program_ast());
        assert_eq!(initial, Value::Int(tagless));
    }

    #[test]
    fn test_ast_false_branch() {
        // leq(4, 3) → false → else branch = 0
        let ast = Ast::If(
            Box::new(Ast::Leq(Box::new(Ast::Int(4)), Box::new(Ast::Int(3)))),
            Box::new(Ast::Int(99)),
            Box::new(Ast::Int(0)),
        );
        assert_eq!(eval_ast(&ast), Value::Int(0));
    }
}
