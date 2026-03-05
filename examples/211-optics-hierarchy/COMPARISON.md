# OCaml vs Rust: Optics Hierarchy — Iso ⊂ Lens ⊂ Traversal, Iso ⊂ Prism ⊂ Traversal

## Side-by-Side Code

### OCaml

```ocaml
(* The hierarchy as a unified discriminated union *)
type ('s, 'a) optic =
  | Iso_op     of { get : 's -> 'a; reverse_get : 'a -> 's }
  | Lens_op    of { get : 's -> 'a; set : 'a -> 's -> 's }
  | Prism_op   of { preview : 's -> 'a option; review : 'a -> 's }
  | Traversal_op of { over : ('a -> 'a) -> 's -> 's; to_list : 's -> 'a list }

(* Upcast: every Lens is a Traversal *)
let lens_as_traversal { get; set } =
  { over   = (fun f s -> set (f (get s)) s);
    to_list = (fun s  -> [get s]) }

(* Upcast: every Prism is a Traversal *)
let prism_as_traversal { preview; review } =
  { over   = (fun f s -> match preview s with Some a -> review (f a) | None -> s);
    to_list = (fun s  -> match preview s with Some a -> [a]           | None -> []) }

(* Upcast: every Iso is a Lens *)
let iso_as_lens { get; reverse_get } =
  { get; set = (fun a _s -> reverse_get a) }

(* Generic function at the Traversal level — accepts any optic *)
let collect_all trav s = trav.to_list s
```

### Rust (struct-based — mirrors OCaml record approach)

```rust
pub struct Lens<S, A> {
    get_fn: Box<dyn Fn(&S) -> A>,
    set_fn: Box<dyn Fn(A, &S) -> S>,
}

impl<S: Clone + 'static, A: Clone + 'static> Lens<S, A> {
    /// Every Lens is a Traversal.
    pub fn as_traversal(self) -> Traversal<S, A> {
        use std::rc::Rc;
        let get_fn = Rc::new(self.get_fn);
        let get_fn2 = Rc::clone(&get_fn);
        let set_fn = self.set_fn;
        Traversal::new(
            move |f, s| { let a = get_fn(s); set_fn(f(&a), s) },
            move |s| vec![get_fn2(s)],
        )
    }
}
```

### Rust (trait-based — zero-cost compile-time dispatch)

```rust
pub trait OpticBase<S: Clone, A: Clone> {
    fn collect(&self, s: &S) -> Vec<A>;
    fn over_t(&self, f: &dyn Fn(&A) -> A, s: &S) -> S;
}

pub trait LensOptic<S: Clone, A: Clone>: OpticBase<S, A> {
    fn view(&self, s: &S) -> A;
    fn set(&self, a: A, s: &S) -> S;
}

pub trait PrismOptic<S: Clone, A: Clone>: OpticBase<S, A> {
    fn preview(&self, s: &S) -> Option<A>;
    fn review(&self, a: A) -> S;
}

pub trait IsoOptic<S: Clone, A: Clone>: LensOptic<S, A> + PrismOptic<S, A> {
    fn reverse_get(&self, a: A) -> S;
}

// CelsiusIso implements ALL FOUR traits — proving it sits at the top of the hierarchy.
pub struct CelsiusIso;
impl OpticBase<Celsius, f64> for CelsiusIso { /* ... */ }
impl LensOptic<Celsius, f64> for CelsiusIso { /* ... */ }
impl PrismOptic<Celsius, f64> for CelsiusIso { /* ... */ }
impl IsoOptic<Celsius, f64>  for CelsiusIso { /* ... */ }

// Generic code at the base level — accepts Lens, Prism, or Iso equally
fn first_focus<S: Clone, A: Clone>(optic: &dyn OpticBase<S, A>, s: &S) -> Option<A> {
    optic.collect(s).into_iter().next()
}
```

## Type Signatures

| Concept             | OCaml                                                          | Rust                                             |
|---------------------|----------------------------------------------------------------|--------------------------------------------------|
| Traversal           | `{ over : ('a -> 'a) -> 's -> 's; to_list : 's -> 'a list }`  | `struct Traversal<S, A>` with boxed closures     |
| Lens                | `{ get : 's -> 'a; set : 'a -> 's -> 's }`                    | `struct Lens<S, A>` with `get_fn` + `set_fn`     |
| Prism               | `{ preview : 's -> 'a option; review : 'a -> 's }`            | `struct Prism<S, A>` with `preview_fn` + `review_fn` |
| Iso                 | `{ get : 's -> 'a; reverse_get : 'a -> 's }`                  | `struct Iso<S, A>` with `get_fn` + `reverse_get_fn` |
| Lens upcast         | `let lens_as_traversal { get; set } = ...`                    | `fn as_traversal(self) -> Traversal<S, A>`       |
| Iso set (lossless)  | `set = (fun a _s -> reverse_get a)`                           | `move \|a, _\| rev(a)` — `_s` discarded          |
| Prism miss in over  | `\| None -> s`                                                | `None => s.clone()` — `Clone` required           |
| Trait hierarchy     | Module-level type classes (first-class modules)               | Supertrait bounds: `LensOptic: OpticBase`        |
| Generic function    | `let collect_all trav s = trav.to_list s`                     | `fn collect_all(optic: &dyn OpticBase<S,A>,...)`  |

## Key Insights

1. **Sharing closures between two consumers**: In OCaml, record fields are values that can be freely aliased. In Rust, a `Box<dyn Fn>` can only be moved once. When both the `over` closure and the `to_list` closure in `Traversal` need the same `get_fn`, Rust requires `Rc::new(get_fn)` + `Rc::clone` to share ownership — a pattern invisible in OCaml.

2. **Iso `set` discards the original structure**: An Iso's `set(a, _s) = reverse_get(a)` throws away `_s`. This is only safe because the Iso is lossless — `a` carries full information. A regular Lens cannot do this. Rust makes this explicit by marking the parameter `_` (intentionally unused), whereas OCaml uses `_s` by convention.

3. **Trait supertraits vs OCaml module hierarchies**: OCaml expresses the hierarchy through functor composition and first-class module inclusion. Rust uses supertrait bounds (`LensOptic<S,A>: OpticBase<S,A>`), which enforces the hierarchy statically at compile time. A type implementing `IsoOptic` must implement all four traits — the compiler verifies membership in the hierarchy.

4. **Two representation strategies**: Rust offers a choice OCaml doesn't force you to make explicitly. The struct-based approach (`Lens<S,A>`, `Prism<S,A>`) allows runtime composition and returning optics from functions — closer to OCaml's first-class records. The trait-based approach (`LensOptic`, `PrismOptic`) is zero-cost with monomorphisation but requires a named type per optic instance.

5. **`Clone` as an explicit effect**: OCaml's `s` in `None -> s` is structurally shared for free. In Rust's `None => s.clone()`, `Clone` is an explicit operation. Requiring `S: Clone` in `Prism::as_traversal` makes the cost visible at the type level and is absent from the Lens path, which never needs to copy the structure.

## When to Use Each Style

**Use struct-based optics when:** you need to return optics from functions, build them at runtime, or compose them dynamically (e.g., configuring which fields to traverse based on user input).

**Use trait-based optics when:** the optic is statically known, you want zero-cost abstraction via monomorphisation, and you want the type system to enforce hierarchy membership at compile time.
