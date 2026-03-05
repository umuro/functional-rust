# OCaml vs Rust: Recursive Types with Box

## Side-by-Side Code

### OCaml

```ocaml
(* OCaml heap-allocates everything; recursive types just work *)
type 'a tree = Leaf | Node of 'a tree * 'a * 'a tree

let rec insert x = function
  | Leaf -> Node (Leaf, x, Leaf)
  | Node (l, v, r) ->
    if x < v then Node (insert x l, v, r)
    else if x > v then Node (l, v, insert x r)
    else Node (l, v, r)

let rec to_sorted_list = function
  | Leaf -> []
  | Node (l, v, r) -> to_sorted_list l @ [v] @ to_sorted_list r

(* Linked list *)
type 'a mylist = Nil | Cons of 'a * 'a mylist

let rec length = function Nil -> 0 | Cons (_, t) -> 1 + length t

(* Expression AST *)
type expr =
  | Num of float
  | Add of expr * expr
  | Mul of expr * expr
  | Neg of expr

let rec eval = function
  | Num n -> n
  | Add (a, b) -> eval a +. eval b
  | Mul (a, b) -> eval a *. eval b
  | Neg e -> -. eval e
```

### Rust (idiomatic — Box for heap indirection)

```rust
#[derive(Debug)]
pub enum Tree<T> {
    Leaf,
    Node(Box<Tree<T>>, T, Box<Tree<T>>),
}

impl<T: Ord> Tree<T> {
    pub fn insert(self, x: T) -> Self {
        match self {
            Tree::Leaf => Tree::Node(Box::new(Tree::Leaf), x, Box::new(Tree::Leaf)),
            Tree::Node(l, v, r) => {
                if x < v      { Tree::Node(Box::new(l.insert(x)), v, r) }
                else if x > v { Tree::Node(l, v, Box::new(r.insert(x))) }
                else          { Tree::Node(l, v, r) }
            }
        }
    }

    pub fn to_sorted_vec(&self) -> Vec<&T> {
        match self {
            Tree::Leaf => vec![],
            Tree::Node(l, v, r) => {
                let mut result = l.to_sorted_vec();
                result.push(v);
                result.extend(r.to_sorted_vec());
                result
            }
        }
    }
}
```

### Rust (linked list with Box)

```rust
#[derive(Debug)]
pub enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}
```

### Rust (expression AST with Box)

```rust
#[derive(Debug)]
pub enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

pub fn eval(expr: &Expr) -> f64 {
    match expr {
        Expr::Num(n)    => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
        Expr::Neg(e)    => -eval(e),
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Recursive tree | `type 'a tree = Leaf \| Node of 'a tree * 'a * 'a tree` | `enum Tree<T> { Leaf, Node(Box<Tree<T>>, T, Box<Tree<T>>) }` |
| Linked list | `type 'a mylist = Nil \| Cons of 'a * 'a mylist` | `enum List<T> { Nil, Cons(T, Box<List<T>>) }` |
| Optional value | `'a option` | `Option<T>` |
| Heap pointer | implicit (GC) | `Box<T>` (explicit) |
| Insert signature | `val insert : 'a -> 'a tree -> 'a tree` | `fn insert(self, x: T) -> Self` |
| Eval signature | `val eval : expr -> float` | `fn eval(expr: &Expr) -> f64` |

## Key Insights

1. **Heap allocation is implicit in OCaml, explicit in Rust.** OCaml's GC stores every value behind a pointer; the programmer never thinks about it. Rust forces you to say `Box::new(...)` exactly where heap indirection occurs, making allocation costs visible and auditable.

2. **`Box<T>` is pointer-sized.** The compiler knows `sizeof(Box<T>) == sizeof(usize)`, so `sizeof(Node) = sizeof(Box<Tree<T>>) + sizeof(T) + sizeof(Box<Tree<T>>)` is computable. Without `Box`, the size formula would be self-referential and the compiler rejects it with `E0072: recursive type has infinite size`.

3. **`insert` consumes `self` instead of returning a new allocation.** OCaml's `insert` creates new `Node` values sharing subtrees via the GC. Rust's version does the same by consuming the old tree and constructing a new one — no `Clone` required because ownership transfers.

4. **Pattern matching looks identical.** Both languages destructure recursive types with `match`/`function`. The only textual difference is `Box::new(...)` at construction sites and the absence of dereferencing when matching (Rust auto-derefs through `Box` in patterns).

5. **The same pattern scales to any recursive data structure.** Trees, lists, expression ASTs, s-expressions, trie nodes — any recursive enum uses `Box` on the recursive fields. The shape of the code is always the same; only the payload type and match arms change.

## When to Use Each Style

**Use idiomatic Rust (iterator/method style) when:** building production data structures where you want to leverage `std` traits (`Iterator`, `Default`, `From`).

**Use recursive Rust (close to OCaml) when:** translating algorithms directly from functional pseudocode or a textbook, where preserving the structural correspondence helps understanding and correctness review.
