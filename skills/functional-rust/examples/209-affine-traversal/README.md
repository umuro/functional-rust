# 209: Affine Traversal

**Difficulty:** ⭐⭐⭐  **Level:** Advanced

A Lens that might find nothing — `preview` returns `Option<A>`, and `over` is a no-op when the target is absent.

## The Problem This Solves

You have a struct with optional fields, or a `HashMap` where a key might not exist, or the first element of a possibly-empty `Vec`. You want to read or modify those values when they exist — and silently do nothing when they don't.

With a plain `Option`, you handle it inline every time:

```rust
// Reading an optional email
if let Some(ref email) = user.email {
    println!("Email: {}", email);
}

// Updating an optional email to uppercase
if let Some(ref email) = user.email {
    user = User { email: Some(email.to_uppercase()), ..user };
}

// Now do the same for phone, address, profile_pic, ...
// Each field needs its own if-let block, every time
```

With a `HashMap` lookup:

```rust
// Get, transform, re-insert
if let Some(v) = map.get("key") {
    let new_v = transform(v.clone());
    map.insert("key".to_string(), new_v);
}
```

These are all the same pattern: "if it exists, do something; if not, leave the structure unchanged." An Affine Traversal bundles that pattern into a reusable optic — one value that knows both how to *find* the target (returning `Option`) and how to *set* it. This exists to solve exactly that pain.

## The Intuition

In the optics hierarchy:
- A **Lens** always finds exactly one value (a mandatory field)
- An **Affine Traversal** finds *at most one* value (an optional field) — like a Lens that can fail
- A **Traversal** finds *zero or more* values

An Affine Traversal has two operations:

1. **`preview`** — try to get the value: returns `Option<A>`. Returns `None` if the target doesn't exist.
2. **`set`** / **`over`** — modify the value if it exists; return the structure unchanged if it doesn't.

**Analogy:** An Affine Traversal is like a GPS navigator that might say "there's no route." `preview` is "can you find me a route?" — returns `Some(route)` or `None`. `over` is "take this route and add 10 minutes" — if there's no route, you're still at the same location.

```rust
// Affine Traversal: S is the whole structure, A is the optional focus
struct Affine<S, A> {
    preview: Box<dyn Fn(&S) -> Option<A>>,  // read: might find nothing
    set:     Box<dyn Fn(A, &S) -> S>,       // write: always returns S
                                             //        (unchanged if not present)
}

// over: derived from preview + set
// If target exists → apply f, write back
// If target absent → return structure unchanged
fn over(&self, f: impl FnOnce(A) -> A, s: &S) -> S where S: Clone {
    match (self.preview)(s) {
        Some(v) => (self.set)(f(v), s),
        None    => s.clone(),              // no-op
    }
}
```

The name "affine" comes from mathematics — an affine traversal focuses on at most one element (0 or 1), bridging the gap between a Lens (exactly 1) and a full Traversal (0 to many).

## How It Works in Rust

```rust
// Step 1: The struct — two boxed closures
struct Affine<S, A> {
    preview: Box<dyn Fn(&S) -> Option<A>>,
    set:     Box<dyn Fn(A, &S) -> S>,
}

// Step 2: Affine for an optional struct field
#[derive(Clone)]
struct User {
    name:  String,
    email: Option<String>,
    phone: Option<String>,
}

fn email_affine() -> Affine<User, String> {
    Affine::new(
        |u| u.email.clone(),               // preview: return the Option<String>
        |e, u| User { email: Some(e), ..u.clone() },  // set: replace email
    )
}

let alice = User { name: "Alice".into(), email: Some("alice@x.com".into()), phone: None };
let bob   = User { name: "Bob".into(),   email: None,                       phone: Some("555".into()) };

// preview — gets Option<A>
email_affine().preview(&alice);  // Some("alice@x.com")
email_affine().preview(&bob);    // None

// over — modifies when present, no-op when absent
let alice2 = email_affine().over(|e| e.to_uppercase(), &alice);
alice2.email;  // Some("ALICE@X.COM")

let bob2 = email_affine().over(|e| e.to_uppercase(), &bob);
bob2.email;    // None  ← no-op, bob had no email

// Step 3: Affine for HashMap — "lens into a key that might not exist"
fn at_key(key: String) -> Affine<HashMap<String, String>, String> {
    let k1 = key.clone();
    let k2 = key;
    Affine::new(
        move |m| m.get(&k1).cloned(),               // preview: might be None
        move |v, m| {
            let mut m2 = m.clone();
            m2.insert(k2.clone(), v);               // set: always inserts
            m2
        },
    )
}

let m: HashMap<String, String> = [("a".into(), "1".into())].into();
at_key("a".into()).preview(&m);   // Some("1")
at_key("z".into()).preview(&m);   // None

// Step 4: Composing two Affine Traversals
// "get the email of the user at key 'admin' in a HashMap<String, User>"
// compose_affine(map_at_admin, user_email):
//   preview = map_at_admin.preview(m).and_then(|u| user_email.preview(&u))
//   over    = if key exists, modify user's email; otherwise no-op

fn compose_affine<S, A, B>(
    outer: Affine<S, A>,  // S -> Option<A>
    inner: Affine<A, B>,  // A -> Option<B>
) -> Affine<S, B>
where S: Clone + 'static, A: Clone + 'static, B: 'static
{
    // preview: chain the two Options with and_then
    // set: find the outer target, modify it with inner's set, write back
    // ...
}
```

## What This Unlocks

- **Safe optional field access** — define one `Affine` per optional field; `over` handles the absent case automatically everywhere, no scattered `if let Some` boilerplate.
- **HashMap / cache operations** — "update the value at key X if it exists" is exactly an Affine Traversal, and it composes with other optics.
- **Composition with Lens and Prism** — an Affine Traversal sits between a Lens (always succeeds) and a Prism (constructs or deconstructs a variant). Understanding Affine Traversal completes the picture of how the optics hierarchy works.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Affine type | Record `{ preview: 's -> 'a option; set: 'a -> 's -> 's }` | `struct Affine<S, A>` with two `Box<dyn Fn>` fields |
| Optional fields | `string option` fields in record | `Option<String>` fields in struct |
| Map type | `StringMap` via functor / association list | `HashMap<String, String>` from `std::collections` |
| `over` on absent | Pattern match → return `s` unchanged | `match preview(s) { None => s.clone() }` — requires `Clone` bound |
| Composition | Option chaining via `>>=` | `and_then` for preview; match-then-set for the write direction |
