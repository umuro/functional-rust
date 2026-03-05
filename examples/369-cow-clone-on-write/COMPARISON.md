# OCaml vs Rust: Clone-on-Write

## Side-by-Side Comparison

### Conditional Processing

**OCaml:**
```ocaml
let maybe_uppercase s threshold =
  if String.length s > threshold then String.uppercase_ascii s
  else s  (* no copy needed - string is immutable *)
```

**Rust:**
```rust
fn ensure_no_spaces(s: &str) -> Cow<str> {
    if s.contains(' ') {
        Cow::Owned(s.replace(' ', "_"))
    } else {
        Cow::Borrowed(s)
    }
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Strings | Immutable by default | Owned String or borrowed &str |
| Copy-on-write | Implicit (GC handles) | Explicit Cow<T> |
| Return type | Same type | Cow enum |
| Allocation | Hidden by GC | Explicit Borrowed/Owned |

## OCaml's Advantage

In OCaml, strings are immutable, so returning the same string or a new one has the same type. The GC handles memory - no explicit Cow needed.

## Rust's Cow

Rust's ownership model requires distinguishing between:
- `&str` - borrowed string slice
- `String` - owned string

`Cow<str>` bridges these, allowing a function to return either depending on whether modification occurred.

## Performance

| Scenario | OCaml | Rust |
|----------|-------|------|
| No change needed | Return same ref | `Cow::Borrowed` - zero alloc |
| Change needed | Allocate new string | `Cow::Owned` - allocate |
| Memory overhead | GC tracking | Enum discriminant (1 byte) |

## Use Cases

- String sanitization
- Path normalization
- Config value processing
- Any "transform if needed" pattern
