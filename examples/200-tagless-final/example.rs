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

fn main() {
    let result = program::<Eval>();
    println!("eval:   {result}");

    let printed = program::<Pretty>();
    println!("pretty: {printed}");

    // Demonstrate individual interpreter operations
    println!("\nDirect operations:");
    println!("Eval::add(3, 4) = {}", Eval::add(Eval::int(3), Eval::int(4)));
    println!("Pretty::add(3, 4) = {}", Pretty::add(Pretty::int(3), Pretty::int(4)));
    println!("Eval::bool_val(true) = {}", Eval::bool_val(true));
    println!("Pretty::bool_val(true) = {}", Pretty::bool_val(true));
}

/* Output:
   eval:   42
   pretty: (if ((3 + 4) <= (2 * 5)) then 42 else 0)

   Direct operations:
   Eval::add(3, 4) = 7
   Pretty::add(3, 4) = (3 + 4)
   Eval::bool_val(true) = true
   Pretty::bool_val(true) = true
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_program() {
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
}
