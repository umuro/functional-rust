use std::collections::{BTreeMap, HashMap};

// ── Solution 1: Idiomatic Rust — mutable trie ─────────────────────────────
#[derive(Default)]
pub struct Trie {
    is_word: bool,
    children: HashMap<char, Trie>,
}

impl Trie {
    pub fn new() -> Self {
        Trie::default()
    }

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

    pub fn starts_with(&self, prefix: &str) -> bool {
        let mut node = self;
        for c in prefix.chars() {
            match node.children.get(&c) {
                None => return false,
                Some(child) => node = child,
            }
        }
        true
    }
}

// ── Solution 2: Functional / persistent trie ──────────────────────────────
#[derive(Clone, Default)]
pub struct FunctionalTrie {
    is_word: bool,
    children: BTreeMap<char, FunctionalTrie>,
}

impl FunctionalTrie {
    pub fn empty() -> Self {
        FunctionalTrie::default()
    }

    pub fn insert(self, word: &str) -> Self {
        fn go(mut node: FunctionalTrie, chars: &[char]) -> FunctionalTrie {
            match chars {
                [] => FunctionalTrie {
                    is_word: true,
                    ..node
                },
                [c, rest @ ..] => {
                    let child = node.children.remove(c).unwrap_or_default();
                    node.children.insert(*c, go(child, rest));
                    node
                }
            }
        }
        let chars: Vec<char> = word.chars().collect();
        go(self, &chars)
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
        let chars: Vec<char> = word.chars().collect();
        go(self, &chars)
    }
}

pub fn build_trie<'a>(words: impl IntoIterator<Item = &'a str>) -> FunctionalTrie {
    words
        .into_iter()
        .fold(FunctionalTrie::empty(), |t, w| t.insert(w))
}

fn main() {
    // ── Mutable trie ──────────────────────────────────────────────────────
    println!("=== Mutable Trie ===");
    let mut t = Trie::new();
    for w in ["cat", "car", "card", "care", "dare"] {
        t.insert(w);
    }
    for w in ["cat", "ca", "card", "dare", "dog"] {
        println!("{}: {}", w, t.contains(w));
    }
    println!("starts_with(\"car\"): {}", t.starts_with("car"));
    println!("starts_with(\"dog\"): {}", t.starts_with("dog"));

    // ── Functional trie (mirrors OCaml fold) ─────────────────────────────
    println!("\n=== Functional Trie ===");
    let ft = build_trie(["cat", "car", "card", "care", "dare"]);
    for w in ["cat", "ca", "card", "dare", "dog"] {
        println!("{}: {}", w, ft.contains(w));
    }
}

/* Output:
   === Mutable Trie ===
   cat: true
   ca: false
   card: true
   dare: true
   dog: false
   starts_with("car"): true
   starts_with("dog"): false

   === Functional Trie ===
   cat: true
   ca: false
   card: true
   dare: true
   dog: false
*/
