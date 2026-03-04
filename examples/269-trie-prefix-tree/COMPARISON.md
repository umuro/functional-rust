# OCaml vs Rust: Trie Prefix Tree

## Side-by-Side Code

### OCaml
```ocaml
module CMap = Map.Make(Char)

type trie = { is_word: bool; children: trie CMap.t }

let empty = { is_word = false; children = CMap.empty }

let insert word trie =
  let rec go i node =
    if i = String.length word then { node with is_word = true }
    else
      let c = word.[i] in
      let child = try CMap.find c node.children with Not_found -> empty in
      { node with children = CMap.add c (go (i+1) child) node.children }
  in go 0 trie

let mem word trie =
  let rec go i node =
    if i = String.length word then node.is_word
    else match CMap.find_opt word.[i] node.children with
    | None -> false | Some child -> go (i+1) child
  in go 0 trie
```

### Rust (idiomatic — mutable)
```rust
#[derive(Default)]
pub struct Trie {
    is_word: bool,
    children: HashMap<char, Trie>,
}

impl Trie {
    pub fn insert(&mut self, word: &str) {
        let mut node = self;
        for c in word.chars() {
            node = node.children.entry(c).or_default();
        }
        node.is_word = true;
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut node = self;
        for c in word.chars() {
            match node.children.get(&c) {
                None => return false,
                Some(child) => node = child,
            }
        }
        node.is_word
    }
}
```

### Rust (functional / persistent — mirrors OCaml)
```rust
#[derive(Clone, Default)]
pub struct FunctionalTrie {
    is_word: bool,
    children: BTreeMap<char, FunctionalTrie>,
}

impl FunctionalTrie {
    pub fn insert(self, word: &str) -> Self {
        fn go(mut node: FunctionalTrie, chars: &[char]) -> FunctionalTrie {
            match chars {
                [] => FunctionalTrie { is_word: true, ..node },
                [c, rest @ ..] => {
                    let child = node.children.remove(c).unwrap_or_default();
                    node.children.insert(*c, go(child, rest));
                    node
                }
            }
        }
        go(self, &word.chars().collect::<Vec<_>>())
    }

    pub fn contains(&self, word: &str) -> bool {
        fn go(node: &FunctionalTrie, chars: &[char]) -> bool {
            match chars {
                [] => node.is_word,
                [c, rest @ ..] => node
                    .children
                    .get(c)
                    .is_some_and(|child| go(child, rest)),
            }
        }
        go(self, &word.chars().collect::<Vec<_>>())
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Trie type | `type trie = { is_word: bool; children: trie CMap.t }` | `struct Trie { is_word: bool, children: HashMap<char, Trie> }` |
| Character map | `Map.Make(Char)` (balanced BST functor) | `HashMap<char, _>` or `BTreeMap<char, _>` |
| Insert (functional) | `val insert : string -> trie -> trie` | `fn insert(self, word: &str) -> Self` |
| Insert (mutable) | — | `fn insert(&mut self, word: &str)` |
| Membership | `val mem : string -> trie -> bool` | `fn contains(&self, word: &str) -> bool` |
| Empty trie | `empty : trie` | `Trie::default()` / `Trie::new()` |

## Key Insights

1. **Functor vs concrete type:** OCaml's `Map.Make(Char)` generates a type-safe ordered map for `char` keys at the module level. Rust uses `HashMap<char, T>` (O(1)) or `BTreeMap<char, T>` (ordered, O(log n)) as concrete generic types — no functor machinery needed.

2. **Mutability is idiomatic:** The OCaml version is purely functional (persistent nodes). Idiomatic Rust prefers `&mut self` and mutation in place via `entry().or_default()` — simpler, faster, and familiar to Rust readers.

3. **Structural sharing costs:** OCaml's GC provides structural sharing of unchanged subtrees for free. True persistence in Rust requires `Rc<RefCell<_>>` or `Arc<Mutex<_>>`, adding complexity. The functional Rust version here *consumes* the old trie rather than sharing it.

4. **Nested helper functions:** Both languages use an inner recursive function (`let rec go` in OCaml, `fn go` nested inside the method in Rust) to thread the index/offset through recursion without exposing it in the public API.

5. **`is_some_and` vs `map_or`:** Clippy's `unnecessary_map_or` lint enforces `option.is_some_and(pred)` over `option.map_or(false, pred)` in modern Rust — a readability win that has no OCaml equivalent since OCaml uses `match` or `Option.map`.

## When to Use Each Style

**Use idiomatic mutable Rust when:** you need the best performance and don't require multiple immutable snapshots of the trie — e.g., building an autocomplete index once and querying it many times.

**Use functional/consuming Rust when:** you want to mirror OCaml's builder-pattern API (`trie = trie.insert("word")`) or need the code to read like a functional pipeline using `Iterator::fold`.
