# OCaml vs Rust: Practical Lens — Deeply Nested Config Update

## The Problem

Real configs are deeply nested. Without lenses, updating `App → Server → DB → Pool → max_size`
requires manually rebuilding every ancestor: clone the pool with the new field, clone the db
with the new pool, clone the server with the new db, clone the app with the new server.
Four levels of boilerplate for one field change. Lenses eliminate this.

## Side-by-Side Code

### OCaml — lens type and composition

```ocaml
type ('s, 'a) lens = {
  get : 's -> 'a;
  set : 'a -> 's -> 's;
}

let compose outer inner = {
  get = (fun s -> inner.get (outer.get s));
  set = (fun b s -> outer.set (inner.set b (outer.get s)) s);
}

let over l f s = l.set (f (l.get s)) s
```

### Rust (idiomatic) — lens struct with Rc-shared closures

```rust
type GetFn<S, A> = Rc<dyn Fn(&S) -> A>;
type SetFn<S, A> = Rc<dyn Fn(A, &S) -> S>;

pub struct Lens<S, A> {
    get: GetFn<S, A>,
    set: SetFn<S, A>,
}

impl<S: 'static, A: 'static> Lens<S, A> {
    pub fn compose<B: 'static>(self, inner: Lens<A, B>) -> Lens<S, B>
    where A: Clone {
        let outer_get = Rc::clone(&self.get);
        let outer_get2 = Rc::clone(&self.get);
        let outer_set = Rc::clone(&self.set);
        let inner_get = Rc::clone(&inner.get);
        let inner_set = Rc::clone(&inner.set);
        Lens {
            get: Rc::new(move |s| inner_get(&outer_get(s))),
            set: Rc::new(move |b, s| {
                let a = outer_get2(s);
                outer_set(inner_set(b, &a), s)
            }),
        }
    }
}
```

### Rust (functional) — atomic lenses composed to deep focus

```rust
// Four atomic lenses composed into one App → u32 lens (pool max_size, 4 levels deep)
pub fn app_pool_max_size() -> Lens<AppConfig, u32> {
    app_server()
        .compose(server_db())
        .compose(db_pool())
        .compose(pool_max_size())
}

// Using it: no manual clone chain
pub fn configure_for_production(config: &AppConfig) -> AppConfig {
    let c = app_pool_max_size().over(|n| n * 2, config);
    let c = app_ssl_enabled().set(true, &c);
    app_debug().set(false, &c)
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Lens type | `('s, 'a) lens = { get: 's -> 'a; set: 'a -> 's -> 's }` | `struct Lens<S, A> { get: Rc<dyn Fn(&S)->A>, set: Rc<dyn Fn(A,&S)->S> }` |
| Composition | `compose : ('s,'a) lens -> ('a,'b) lens -> ('s,'b) lens` | `fn compose<B>(self, inner: Lens<A,B>) -> Lens<S,B>` |
| Over | `over : ('s,'a) lens -> ('a->'a) -> 's -> 's` | `fn over(&self, f: impl FnOnce(A)->A, s: &S) -> S` |
| Immutability | structural, algebraic types | owned values + `..struct.clone()` spread |

## Key Insights

1. **Record update syntax**: OCaml's `{ s with field = v }` is mirrored by Rust's `Struct { field: v, ..s.clone() }`. Both express "copy everything except this field", making lens setters concise at every level.

2. **Closure sharing**: OCaml closures are garbage-collected so the `compose` function can freely capture `outer.get` in both `get` and `set`. Rust requires `Rc::clone` to give each composed closure its own reference-counted handle to the same underlying function — no actual data is copied.

3. **Composition is chaining**: OCaml uses `let ( |>> ) = compose` so lenses chain with `app_server |>> server_db |>> db_pool |>> pool_max_size`. Rust uses method-chaining `.compose()` — syntactically different, semantically identical.

4. **Boilerplate vs. correctness**: Without lenses, a six-setting production configurator touches 24+ lines of nested clone code. Each line is an opportunity for a silent field-name typo. With lenses, each setting is one `set` or `over` call regardless of nesting depth.

5. **Lens laws hold**: Both implementations satisfy the three lens laws — `get(set(a,s))=a`, `set(get(s),s)=s`, `set(b,set(a,s))=set(b,s)` — because the setter rebuilds the full ancestor chain using the actual getter value, not a stale copy.

## When to Use Each Style

**Use atomic lenses** when a field is updated in multiple places — define it once, reuse everywhere.  
**Use composed lenses** when transformations target fields at different nesting depths and you want the call site to read as a single focused operation.  
**Use `over` instead of `set`** when the new value depends on the old one (e.g., doubling pool size), keeping the transformation co-located with the lens rather than split across a `view` + `set`.
