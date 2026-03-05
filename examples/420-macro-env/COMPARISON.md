# OCaml vs Rust: Compile-Time Environment

## Rust env! and option_env!

```rust
// Required: compile error if missing
const VERSION: &str = env!("CARGO_PKG_VERSION");

// Optional: returns Option<&str>
const API_KEY: Option<&str> = option_env!("API_KEY");

// Common Cargo variables
env!("CARGO_PKG_NAME")       // package name
env!("CARGO_PKG_VERSION")    // version
env!("CARGO_MANIFEST_DIR")   // path to Cargo.toml
env!("CARGO_PKG_AUTHORS")    // authors
```

## OCaml Equivalent

```ocaml
(* No direct compile-time env access *)
(* Use dune substitution or build scripts *)

(* dune file:
   (library
     (name mylib)
     (preprocess (action (run %{bin:ppx_version} %{env:VERSION=0.0.0}))))
*)

(* Or runtime: *)
let version = Sys.getenv_opt "VERSION" |> Option.value ~default:"unknown"
```

## 5 Takeaways

1. **`env!` reads env vars at compile time.**
2. **Fails compilation if variable is missing.**
3. **`option_env!` returns `Option<&str>`.**
4. **Cargo provides many built-in variables.**
5. **OCaml relies on runtime or build scripts.**
