# Run-Length Encoding — OCaml vs Rust Comparison

## Core Insight

Both languages handle string building through a mutable buffer pattern. OCaml uses `Buffer.t`, Rust uses `String` (which is essentially a growable UTF-8 buffer). The recursive structure maps directly between both languages.

## OCaml Approach

Uses `Buffer.create` for efficient string building, with a recursive `go` function that tracks current character and count. Character access via `s.[i]` is O(1) since OCaml strings are byte arrays. Buffer operations are imperative but wrapped in a functional recursive structure.

## Rust Approach

Uses `String::push` and `String::push_str` for building. Characters must first be collected into `Vec<char>` for indexed access since Rust strings are UTF-8 (variable-width). The iterative version is more idiomatic in Rust than the recursive one due to ownership considerations.

## Comparison Table

| Aspect        | OCaml                         | Rust                               |
|---------------|-------------------------------|-------------------------------------|
| **Memory**    | `Buffer.t` (growable)         | `String` (growable Vec<u8>)         |
| **Null safety** | N/A                        | `unwrap()` on last char             |
| **Errors**    | Index out of bounds           | Panics on OOB, or use `.get()`      |
| **Iteration** | Recursive with index          | For loop over indices or windows    |
| **Strings**   | Byte array (O(1) index)      | UTF-8 (must collect to Vec<char>)   |

## Things Rust Learners Should Notice

1. **Strings aren't indexable** — `s[i]` doesn't work on `&str`; collect to `Vec<char>` first
2. **`String` is a buffer** — `push()` for char, `push_str()` for &str, like OCaml's Buffer
3. **`to_string()` on numbers** — `count.to_string()` allocates; consider `write!` macro for formatting
4. **Roundtrip testing** — `decode(encode(s)) == s` is a property-based testing pattern

## Further Reading

- [String in Rust](https://doc.rust-lang.org/std/string/struct.String.html)
- [Exercism: Run-Length Encoding](https://exercism.org/tracks/rust/exercises/run-length-encoding)
