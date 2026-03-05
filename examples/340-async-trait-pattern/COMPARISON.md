# OCaml vs Rust: Async Trait Pattern

## Trait Definition

**OCaml:**
```ocaml
module type ASYNC_STORE = sig
  type t
  val get : t -> string -> string option Lwt.t
  val set : t -> string -> string -> unit Lwt.t
end
```

**Rust:**
```rust
type AsyncResult<T, E> = Pin<Box<dyn Future<Output = Result<T, E>> + Send>>;

trait AsyncStore: Send + Sync {
    fn get(&self, key: &str) -> AsyncResult<Option<String>, String>;
    fn set(&self, key: String, val: String) -> AsyncResult<(), String>;
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Polymorphism | First-class modules | `dyn Trait` |
| Boxing | Implicit (Lwt.t) | Explicit `Box::pin` |
| Object safety | N/A (modules) | Requires boxed return |
| Crate helper | N/A | `#[async_trait]` |
