📖 **[View on hightechmind.io →](https://hightechmind.io/rust/213-record-update)**

---

# Practical Lens — Deeply Nested Config Update
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

This example returns to the motivating problem from the lens introduction (example 201) and solves it completely with the lens toolkit. A three-level nested `AppConfig` requires updating the database host. Without lenses, this is six lines of destructuring and reconstruction. With composed lenses, it is one line: `over(db_host_lens, |_| new_host, config)`. This demonstrates the practical value of the lens abstraction for real-world code.

## Learning Outcomes

- Apply lens composition to solve the nested update problem concretely
- Implement field lenses for each level of nesting and compose them
- See how `over` with a composed lens updates deeply nested fields in one call
- Understand how this pattern scales to arbitrary nesting depth

## Rust Application

Field lenses: `server_lens: Lens<AppConfig, ServerConfig>`, `db_lens: Lens<ServerConfig, DbConfig>`, `host_lens: Lens<DbConfig, String>`. Composition: `app_db_host = server_lens.compose(db_lens.compose(host_lens))`. Update: `over(&app_db_host, |_| "new-host".to_string(), config)`. The `Rc`-wrapped functions allow sharing the lens after composition. Each `set` clones only the changed path, sharing the rest — efficient for immutable structures.

## OCaml Approach

OCaml's approach with `ppx_lens`:
```ocaml
(* ppx_lens auto-generates these: *)
let server_lens = AppConfig.server
let db_lens = ServerConfig.db
let host_lens = DbConfig.host
let app_db_host = server_lens |> compose db_lens |> compose host_lens
let new_config = over app_db_host (fun _ -> "new-host") config
```
With automatic lens generation, the entire update reduces to composition and `over`. OCaml's `ppx_lens` reduces boilerplate to near zero.

## Key Differences

1. **Boilerplate**: Rust's lens definition requires explicit `get`/`set` closures; OCaml's `ppx_lens` or Rust's `#[derive(Lens)]` eliminate this.
2. **Sharing vs. copying**: Rust's `Rc`-based `set` shares the unchanged subtree; under GC, OCaml achieves the same sharing automatically.
3. **Ergonomics at scale**: With 5+ levels of nesting, composed lenses win decisively over manual reconstruction; the verbosity cost is paid once at lens definition.
4. **Production use**: Redux's `Lens.set`, Haskell's `over`, and Scala's Monocle all use this exact pattern for state management in large applications.

## Exercises

1. Add a fourth level: `AppConfig.server.db.primary.host` and compose a four-lens chain.
2. Implement `update_port(config, new_port)` using composed lenses without touching the database host.
3. Write a function that takes a list of lens-update pairs `(lens, new_value)` and applies all of them to the config.
