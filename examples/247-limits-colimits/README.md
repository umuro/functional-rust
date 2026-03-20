[![Functional Rust](https://img.shields.io/badge/functional--rust-examples-blue)](https://hightechmind.io)

# Limits and Colimits
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Limits and colimits are the categorical generalizations of "greatest lower bounds" and "least upper bounds." Products, terminal objects, equalizers, and pullbacks are all limits; coproducts, initial objects, coequalizers, and pushouts are all colimits. In type theory and functional programming, they correspond to familiar constructs: product types (limits), sum types (colimits), function spaces, and more. This example instantiates categorical limits and colimits as Rust types and shows the universal property that characterizes each.

## Learning Outcomes

- Identify products, coproducts, equalizers, and pullbacks as categorical limits/colimits
- Implement the universal property (unique morphism) for products and coproducts in Rust
- See how `(A, B)` is the limit of the diagram `A ← • → B`
- See how `Either<A, B>` is the colimit of the same diagram
- Compare categorical constructions with OCaml's type-theoretic encodings

## Rust Application

Universal properties encoded as Rust functions:

```rust
// LIMITS (greatest lower bounds of diagrams)

// Product: universal property
// For any type C with projections fst: C->A, snd: C->B,
// there exists a unique f: C -> (A,B) making the diagram commute.
fn product_universal<C, A: Clone, B: Clone>(
    fst: impl Fn(&C) -> A,
    snd: impl Fn(&C) -> B,
) -> impl Fn(C) -> (A, B) {
    move |c| (fst(&c), snd(&c))
}

// Terminal object: unique morphism to ()
fn terminal<A>(_: A) -> () { () }

// Equalizer: the subset where two functions agree
struct Equalizer<A> {
    elements: Vec<A>,
}

impl<A: Clone + PartialEq> Equalizer<A> {
    fn new(source: Vec<A>, f: impl Fn(&A) -> i32, g: impl Fn(&A) -> i32) -> Self {
        Equalizer {
            elements: source.into_iter().filter(|a| f(a) == g(a)).collect(),
        }
    }
}

// Pullback: pairs (a, b) where f(a) = g(b)
struct Pullback<A, B> {
    pairs: Vec<(A, B)>,
}

impl<A: Clone + PartialEq, B: Clone + PartialEq, C: PartialEq> Pullback<A, B> {
    fn new(
        as_: Vec<A>, bs: Vec<B>,
        f: impl Fn(&A) -> C,
        g: impl Fn(&B) -> C,
    ) -> Self {
        let mut pairs = vec![];
        for a in &as_ {
            for b in &bs {
                if f(a) == g(b) {
                    pairs.push((a.clone(), b.clone()));
                }
            }
        }
        Pullback { pairs }
    }
}

// COLIMITS (least upper bounds of diagrams)

// Coproduct: universal property
// For any type C with injections inl: A->C, inr: B->C,
// there exists a unique f: Either<A,B> -> C
fn coproduct_universal<A, B, C>(
    inl: impl Fn(A) -> C,
    inr: impl Fn(B) -> C,
) -> impl Fn(Result<A, B>) -> C {
    move |e| match e {
        Ok(a)  => inl(a),
        Err(b) => inr(b),
    }
}

// Initial object: unique morphism from Never (!)
// fn initial<B>(x: !) -> B { x }  -- not stable in Rust; approximate with enum

// Coequalizer: quotient by the equivalence induced by two functions
// (simplified: partition by equivalence classes)
fn coequalizer_example() {
    // f(x) = x mod 2, g(x) = x mod 2 + 0
    // Coequalizer of f, g: 0..9 -> {0,1} is just the image
    let source: Vec<i32> = (0..10).collect();
    let f = |x: &i32| x % 2;
    let g = |x: &i32| x.abs() % 2;
    // coequalize: quotient source by f(a) = g(b) implies a ~ b
    let mut classes: std::collections::HashMap<i32, Vec<i32>> = std::collections::HashMap::new();
    for x in source {
        classes.entry(f(&x)).or_default().push(x);
    }
    println!("Coequalizer classes: {:?}", classes);
}

fn main() {
    // Product universal property
    let pair = product_universal(|s: &String| s.len(), |s: &String| s.to_uppercase());
    println!("Product: {:?}", pair(String::from("hello"))); // (5, "HELLO")

    // Coproduct universal property
    let fold = coproduct_universal(
        |n: i32| n * 2,       // left branch
        |s: String| s.len() as i32, // right branch
    );
    println!("Coprod left:  {}", fold(Ok(5)));                  // 10
    println!("Coprod right: {}", fold(Err(String::from("hi")))); // 2

    // Equalizer
    let nums: Vec<i32> = (-5..=5).collect();
    let eq = Equalizer::new(nums, |x| x * x, |x| (*x).abs() * (*x).abs());
    println!("Equalizer (x²=|x|²): {:?}", eq.elements); // all elements (always equal)

    // Pullback
    let pb = Pullback::new(
        vec![1_i32, 2, 3, 4], vec![2_i32, 4, 6, 8],
        |x| x % 2,
        |y| y % 2,
    );
    println!("Pullback (same parity): {} pairs", pb.pairs.len());

    coequalizer_example();
}
```

Each construction is characterized by its universal property: a unique morphism from/into the object making all triangles commute.

## OCaml Approach

OCaml types directly embody limits and colimits:

```ocaml
(* Product = record/tuple *)
let product_univ fst snd c = (fst c, snd c)

(* Coproduct = variant *)
let coprod_univ inl inr = function
  | Left a  -> inl a
  | Right b -> inr b

(* Equalizer = filtered list via List.filter *)
let equalizer f g xs = List.filter (fun x -> f x = g x) xs

(* Pullback = cartesian product filtered *)
let pullback f g xs ys =
  List.concat_map (fun x ->
    List.filter_map (fun y ->
      if f x = g y then Some (x, y) else None) ys) xs
```

OCaml's type system makes the connection more transparent; Rust requires more scaffolding to express the same categorical ideas.

## Key Differences

| Concept | Rust | OCaml |
|---|---|---|
| Product | `(A, B)` tuple | record or tuple |
| Coproduct | `enum Either<A,B>` | `type ('a,'b) either` |
| Terminal | `()` unit | `unit` |
| Initial | `!` (never, unstable) | `type empty = \|` |
| Equalizer | `Vec` filter | `List.filter` |
| Pullback | nested filter | `List.concat_map` |

Limits/colimits are why product types and sum types look the way they do: they are the unique objects satisfying universal properties up to isomorphism.

## Exercises

1. Implement a type-level product and coproduct using a custom `Either<L,R>` enum and verify the universal property functions type-check.
2. Encode the pushout (dual of pullback) as a type: given `A <- C -> B`, the pushout is a type that merges A and B while identifying the images of C.
3. Show that `Option<A>` is the coproduct `A + ()` by implementing `from_option` and `to_option` witnessing the isomorphism.
4. Implement the limit of a chain diagram `A₀ <- A₁ <- A₂ <- ...` (inverse limit) as a type of compatible sequences.
5. Prove that right adjoints preserve limits and left adjoints preserve colimits by showing a concrete example in Rust.
