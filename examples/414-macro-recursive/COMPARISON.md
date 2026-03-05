# OCaml vs Rust: Recursive Macros

## Side-by-Side Code

### OCaml — Recursive functions
```ocaml
let rec count = function
  | [] -> 0
  | _ :: t -> 1 + count t

let rec reverse_acc acc = function
  | [] -> acc
  | h :: t -> reverse_acc (h :: acc) t

let reverse lst = reverse_acc [] lst

let () =
  Printf.printf "count: %d\n" (count [1;2;3;4;5]);
  Printf.printf "reverse: %s\n" 
    (String.concat "," (List.map string_of_int (reverse [1;2;3])))
```

### Rust — Recursive macros
```rust
macro_rules! count {
    () => { 0 };
    ($head:expr $(, $tail:expr)*) => {
        1 + count!($($tail),*)
    };
}

macro_rules! reverse_list {
    (@acc [$($acc:expr),*]) => { [$($acc),*] };
    (@acc [$($acc:expr),*] $head:expr $(, $tail:expr)*) => {
        reverse_list!(@acc [$head $(, $acc)*] $($tail),*)
    };
    ($($x:expr),*) => { reverse_list!(@acc [] $($x),*) };
}

fn main() {
    println!("count: {}", count!(1, 2, 3, 4, 5));
    let rev = reverse_list![1, 2, 3];
}
```

---

## Comparison Table

| Aspect | OCaml Functions | Rust Macros |
|--------|-----------------|-------------|
| When runs | Runtime | Compile time |
| Recursion limit | Stack (TCO helps) | Expansion limit (~128) |
| Accumulator | Function parameter | Internal `@acc` pattern |
| Type checking | Static, once | Per expansion site |

---

## The Accumulator Pattern

For compile-time list reversal:

```rust
macro_rules! reverse_list {
    // Internal rules (start with @)
    (@acc [$($acc:expr),*]) => {
        [$($acc),*]  // Base: return accumulated
    };
    (@acc [$($acc:expr),*] $head:expr $(, $tail:expr)*) => {
        // Move head to front of accumulator, recurse with tail
        reverse_list!(@acc [$head $(, $acc)*] $($tail),*)
    };
    // Public entry point
    ($($x:expr),*) => {
        reverse_list!(@acc [] $($x),*)  // Start with empty acc
    };
}
```

The `@acc` prefix marks internal rules that users shouldn't call directly.

---

## 5 Takeaways

1. **Macros recurse at compile time, not runtime.**
   The result is fully expanded before execution.

2. **Use `@internal` patterns for helper rules.**
   Keeps the public API clean.

3. **Accumulator pattern works in macros too.**
   Same technique as tail-recursive functions.

4. **Rust has a recursion limit (~128).**
   Very deep macro recursion will fail to compile.

5. **Each expansion is type-checked independently.**
   Errors show up at call sites, not macro definition.
