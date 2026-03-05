// Fragment specifiers in macro_rules!
use std::fmt;

// expr: captures any expression
macro_rules! dbg_expr {
    ($e:expr) => {
        { let val = $e; println!("  {} = {:?}", stringify!($e), val); val }
    };
}

// ident: captures an identifier (variable name, function name, etc.)
macro_rules! make_getter {
    ($field:ident : $ty:ty) => {
        fn $field(&self) -> &$ty { &self.$field }
    };
}

// ty: captures a type
macro_rules! make_default_fn {
    ($name:ident -> $ret:ty) => {
        fn $name() -> $ret { Default::default() }
    };
}

// pat: captures a pattern
macro_rules! matches_pat {
    ($val:expr, $pat:pat) => {
        matches!($val, $pat)  // delegates to std matches! macro
    };
}

// literal: captures a literal value
macro_rules! repeat_str {
    ($s:literal, $n:literal) => {
        $s.repeat($n)
    };
}

// block: captures a block expression
macro_rules! time_block {
    ($name:literal, $block:block) => {
        {
            let t = std::time::Instant::now();
            let result = $block;
            println!("Block '{}' took {:?}", $name, t.elapsed());
            result
        }
    };
}

// stmt: captures statements
macro_rules! with_logging {
    ($($stmt:stmt;)*) => {
        {
            println!("--- begin ---");
            $($stmt;)*
            println!("--- end ---");
        }
    };
}

struct Person { name: String, age: u32 }

impl Person {
    fn new(name: &str, age: u32) -> Self { Person { name: name.to_string(), age } }
    make_getter!(name: String);
    make_getter!(age: u32);
}

make_default_fn!(default_string -> String);
make_default_fn!(default_i32 -> i32);

fn main() {
    println!("=== expr ===");
    let x = dbg_expr!(2 + 3 * 4);
    dbg_expr!(x > 10);

    println!("\n=== ident/ty getters ===");
    let p = Person::new("Alice", 30);
    println!("name={}, age={}", p.name(), p.age());

    println!("\n=== ty default ===");
    println!("default_string: {:?}", default_string());
    println!("default_i32: {}", default_i32());

    println!("\n=== pat ===");
    let opt: Option<i32> = Some(42);
    println!("is Some: {}", matches_pat!(opt, Some(_)));
    println!("is None: {}", matches_pat!(opt, None));

    println!("\n=== literal ===");
    println!("{}", repeat_str!("ab", 3));

    println!("\n=== block ===");
    let sum = time_block!("sum", { (1..=1000i64).sum::<i64>() });
    println!("sum = {}", sum);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_dbg_expr() {
        let v = dbg_expr!(1 + 1);
        assert_eq!(v, 2);
    }

    #[test]
    fn test_repeat_str() {
        assert_eq!(repeat_str!("xy", 3), "xyxyxy");
    }
}
