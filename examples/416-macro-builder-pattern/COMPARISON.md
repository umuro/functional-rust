# OCaml vs Rust: Macro Builder Pattern

## The Pattern

Builders solve the "many optional parameters" problem:
- Required fields must be set
- Optional fields have defaults
- Method chaining for ergonomics

---

## Rust Macro Approach

```rust
macro_rules! builder_setters {
    ($($field:ident : $ty:ty),*) => {
        $(
            pub fn $field(mut self, val: $ty) -> Self {
                self.$field = Some(val);
                self
            }
        )*
    };
}

struct RequestBuilder {
    url: Option<String>,
    method: Option<String>,
}

impl RequestBuilder {
    builder_setters!(url: String, method: String);

    pub fn build(self) -> Result<Request, Error> { ... }
}
```

---

## OCaml Approach

```ocaml
(* Using labeled arguments with defaults *)
let make_request
    ~url
    ?(method_="GET")
    ?(timeout=5000)
    () =
  { url; method_; timeout }

let req = make_request ~url:"http://..." ~timeout:10000 ()
```

---

## 5 Takeaways

1. **Macros eliminate setter boilerplate.**
   One line per field instead of five.

2. **OCaml's labeled args are simpler.**
   `?field=default` achieves similar ergonomics.

3. **Rust builders catch missing required fields.**
   `build()` returns `Result` if validation fails.

4. **Method chaining is idiomatic in Rust.**
   `.field(value).other(value).build()`

5. **Macros can generate entire builders.**
   Advanced macros create both struct and builder.
