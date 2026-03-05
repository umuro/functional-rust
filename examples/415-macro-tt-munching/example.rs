// Token tree munching technique in Rust

// Parse a simple "key: type = default" DSL using TTM
macro_rules! define_struct {
    // Done: no more fields
    (@fields $name:ident {} -> { $($fields:tt)* }) => {
        #[derive(Debug, Default)]
        struct $name {
            $($fields)*
        }
    };
    // Munch one field: ident: type = default,
    (@fields $name:ident {
        $field:ident : $ty:ty = $default:expr ,
        $($rest:tt)*
    } -> { $($fields:tt)* }) => {
        define_struct!(@fields $name { $($rest)* } -> {
            $($fields)*
            $field: $ty,
        });
    };
    // Entry point
    (struct $name:ident { $($body:tt)* }) => {
        define_struct!(@fields $name { $($body)* } -> {});
    };
}

// TTM for a simple arithmetic DSL (evaluates at compile time)
macro_rules! calc {
    // Base: single number
    ($n:literal) => { $n };
    // Addition
    ($a:literal + $($rest:tt)+) => {
        $a + calc!($($rest)+)
    };
    // Subtraction
    ($a:literal - $($rest:tt)+) => {
        $a - calc!($($rest)+)
    };
    // Multiplication — higher precedence via nesting
    ($a:literal * $b:literal + $($rest:tt)+) => {
        ($a * $b) + calc!($($rest)+)
    };
    ($a:literal * $b:literal) => {
        $a * $b
    };
}

// Munch pairs
macro_rules! process_pairs {
    // Base case: empty
    (@acc $results:tt) => { $results };
    // Munch one pair
    (@acc [$($acc:expr),*] ($a:expr, $b:expr) $(($ra:expr, $rb:expr))*) => {
        process_pairs!(@acc [$($acc,)* $a + $b] $(($ra, $rb))*)
    };
    // Entry
    ($(($a:expr, $b:expr)),* $(,)?) => {
        process_pairs!(@acc [] $(($a, $b))*)
    };
}

define_struct!(struct Config {
    port: u16 = 8080,
    debug: bool = false,
    max_connections: u32 = 100,
});

fn main() {
    let c = Config { port: 9090, ..Default::default() };
    println!("Config: {:?}", c);

    // calc DSL
    let result = calc!(2 + 3 * 4);
    println!("2 + 3 * 4 = {}", result);

    let r2 = calc!(10 - 3 + 2);
    println!("10 - 3 + 2 = {}", r2);

    // process pairs
    let sums = process_pairs!((1, 2), (3, 4), (5, 6));
    println!("Pair sums: {:?}", sums);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calc() {
        assert_eq!(calc!(2 + 3), 5);
        assert_eq!(calc!(10 - 4), 6);
        assert_eq!(calc!(3 * 4), 12);
    }
}
