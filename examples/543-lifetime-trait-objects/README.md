📖 **[View on hightechmind.io →](https://hightechmind.io/rust/543-lifetime-trait-objects)**

---

# Lifetimes in dyn Trait
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Trait objects (`dyn Trait`) are Rust's mechanism for runtime polymorphism — a single `Box<dyn Renderer>` can hold any type implementing `Renderer`. But trait objects carry an implicit lifetime bound: `Box<dyn Renderer>` is shorthand for `Box<dyn Renderer + 'static>`, meaning the underlying type must contain no non-static references. When you need a trait object that borrows from an external scope, you must write `Box<dyn Renderer + 'a>` explicitly. This is critical for middleware stacks, plugin systems, and any architecture that stores trait objects.

## Learning Outcomes

- Why `Box<dyn Trait>` is `Box<dyn Trait + 'static>` by default
- How to create `Box<dyn Trait + 'a>` for trait objects borrowing from a scope
- How `BorrowingRenderer<'a>` implementing `Renderer` can be stored as `Box<dyn Renderer + 'a>`
- How lifetime-annotated trait objects work in function signatures and structs
- Where this pattern appears: plugin systems, middleware, egui/druid widget trees

## Rust Application

`HtmlRenderer` owns its `template: String` — it is `'static` and fits in `Box<dyn Renderer>`. `BorrowingRenderer<'a>` holds `content: &'a str` — it is not `'static` and must be stored as `Box<dyn Renderer + 'a>`. `store_renderer(r: Box<dyn Renderer>) -> Box<dyn Renderer>` works with static renderers. Functions accepting borrowed renderers must write `Box<dyn Renderer + 'a>` and propagate the `'a` lifetime through their own signature.

Key patterns:
- `Box<dyn Trait>` — implicit `+ 'static` bound on the contained type
- `Box<dyn Trait + 'a>` — explicit lifetime bound for borrowing trait objects
- `dyn Renderer + 'a` in function parameters and struct fields

## OCaml Approach

OCaml achieves runtime polymorphism through first-class modules or abstract types. There are no lifetime constraints on module values — the GC ensures all referenced data remains valid:

```ocaml
module type Renderer = sig
  val render : unit -> string
end
let store_renderer (module R : Renderer) = (module R : Renderer)
```

Any module satisfying `Renderer` can be stored regardless of what data it references.

## Key Differences

1. **Implicit 'static**: Rust's `Box<dyn Trait>` silently requires `'static` — a common source of confusion for beginners; OCaml has no implicit constraint on stored modules or closures.
2. **Lifetime propagation**: When a Rust trait object borrows from a scope, that `'a` propagates through every type that stores or passes the object; OCaml has no propagation.
3. **Plugin systems**: Rust plugins stored as `Box<dyn Plugin>` must be `'static` or carefully parameterized; OCaml plugins have no such restriction.
4. **Error messages**: Missing lifetime on `Box<dyn Trait + 'a>` gives cryptic "does not live long enough" errors; the fix is always to add `+ 'a` to the trait object type.

## Exercises

1. **Scoped renderer**: Write a function `fn render_with<'a>(content: &'a str, renderer: &dyn Renderer) -> String` that uses a borrowed trait object — verify it compiles without a `+ 'a` bound on the renderer since the renderer doesn't capture `content`.
2. **Vec of renderers**: Create `Vec<Box<dyn Renderer>>` (static) and add multiple renderer types — then try creating `Vec<Box<dyn Renderer + '_>>` containing a `BorrowingRenderer` and observe the lifetime constraint.
3. **Trait object field**: Implement `struct Screen<'a> { components: Vec<Box<dyn Renderer + 'a>> }` with a `render_all` method that collects all renders into a `Vec<String>`.
