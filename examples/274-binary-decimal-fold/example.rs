/// Solution 1: Idiomatic Rust — fold over chars, propagate errors
pub fn binary_to_decimal(s: &str) -> Result<u64, String> {
    s.chars().try_fold(0u64, |acc, c| match c {
        '0' => Ok(acc * 2),
        '1' => Ok(acc * 2 + 1),
        _ => Err(format!("invalid binary digit: {c}")),
    })
}

/// Solution 2: Recursive — mirrors the OCaml `go` helper
pub fn decimal_to_binary(n: u64) -> String {
    if n == 0 {
        return "0".to_string();
    }
    fn go(n: u64, acc: String) -> String {
        if n == 0 {
            acc
        } else {
            go(n / 2, format!("{}{}", n % 2, acc))
        }
    }
    go(n, String::new())
}

/// Solution 3: Idiomatic — build binary string with iterators (no recursion)
pub fn decimal_to_binary_iter(n: u64) -> String {
    if n == 0 {
        return "0".to_string();
    }
    let mut bits = Vec::new();
    let mut x = n;
    while x > 0 {
        bits.push((x % 2) as u8);
        x /= 2;
    }
    bits.iter().rev().map(|b| b.to_string()).collect()
}

fn main() {
    println!("binary_to_decimal(\"1010\")   = {:?}", binary_to_decimal("1010"));
    println!("binary_to_decimal(\"11111\")  = {:?}", binary_to_decimal("11111"));
    println!("binary_to_decimal(\"1012\")   = {:?}", binary_to_decimal("1012"));
    println!("decimal_to_binary(10)        = {}", decimal_to_binary(10));
    println!("decimal_to_binary(0)         = {}", decimal_to_binary(0));
    println!("decimal_to_binary_iter(42)   = {}", decimal_to_binary_iter(42));

    // Roundtrip check
    for s in ["1010", "11111", "101010"] {
        let d = binary_to_decimal(s).unwrap();
        println!("roundtrip({s}) → {d} → {}", decimal_to_binary(d));
    }
}

/* Output:
   binary_to_decimal("1010")   = Ok(10)
   binary_to_decimal("11111")  = Ok(31)
   binary_to_decimal("1012")   = Err("invalid binary digit: 2")
   decimal_to_binary(10)        = 1010
   decimal_to_binary(0)         = 0
   decimal_to_binary_iter(42)   = 101010
   roundtrip(1010) → 10 → 1010
   roundtrip(11111) → 31 → 11111
   roundtrip(101010) → 42 → 101010
*/
