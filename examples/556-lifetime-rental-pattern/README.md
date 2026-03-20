📖 **[View on hightechmind.io →](https://hightechmind.io/rust/556-lifetime-rental-pattern)**

---

# Rental Pattern

## Problem Statement

The rental pattern addresses a common need: a type that owns its data and provides borrowing access to it — "renting out" references into its own storage. This is a structured version of the owning-reference problem. The `rental` crate (now deprecated) automated this with macros; the `ouroboros` and `self_cell` crates provide safe modern alternatives. Understanding the manual implementation helps explain why the borrow checker prevents naive self-referential structs and what makes the pattern sound.

## Learning Outcomes

- How `Rental` owns a `String` and provides `&str` views via `rent(&self) -> &str`
- How `ParsedRental` stores raw data and indices derived from it, computing views on demand
- Why the returned `&str` from `rent` is tied to `self`'s lifetime (cannot outlive the rental)
- How lazy parsing separates expensive work from construction
- Where rental appears: HTTP request parsing, JSON tree traversal, configuration loading

## Rust Application

`Rental` stores `data: String` and `rent(&self) -> &str` returns `&self.data` — the lifetime of the returned reference is tied to `self`. `rent_slice(&self, start, end) -> &str` returns a windowed view. `ParsedRental` stores `raw: String` and `parsed: Vec<usize>` (indices into `raw`) — parsing computes indices but stores no `&str` references, avoiding self-referential issues. Methods compute `&str` slices from stored indices on demand.

Key patterns:
- `fn rent(&self) -> &str { &self.data }` — borrowing from self
- `Vec<usize>` of indices into owned `String` — lazy parsing without self-reference
- Lifetime: the rented `&str` cannot outlive the `Rental` that owns the data

## OCaml Approach

OCaml makes the rental pattern trivial — a record holding a `string` and methods returning slices of it are straightforward:

```ocaml
type rental = { raw: string; mutable parsed: int list }
let rent r = r.raw
let rent_slice r s e = String.sub r.raw s (e - s)  (* copies *)
```

The GC ensures the `raw` string stays alive as long as any view exists.

## Key Differences

1. **Lifetime enforcement**: Rust's type system ensures `rent(&self) -> &str` cannot outlive `self`; OCaml's GC achieves the same guarantee dynamically.
2. **Lazy parse**: Rust's lazy parse stores `Vec<usize>` indices, computing `&str` views on demand — a common pattern to avoid self-reference; OCaml stores lazy `Lazy.t` computations.
3. **Crate ecosystem**: `ouroboros`, `self_cell`, and `yoke` provide macro-generated safe rental APIs for complex cases; OCaml has no equivalent because the problem does not exist.
4. **Slice copying**: `rent_slice` in OCaml (`String.sub`) copies the data; Rust `&str` slices are zero-copy views into the owned `String`.

## Exercises

1. **CSV rental**: Implement `struct CsvRental { raw: String, row_offsets: Vec<(usize, usize)> }` where `row_offsets` stores `(start, end)` pairs; `row(&self, n: usize) -> &str` returns a zero-copy view.
2. **Lazy parsed rental**: Add a `fields(&self, row: usize) -> Vec<&str>` method to `CsvRental` that splits the row on commas and returns field slices — all zero-copy from the owned `String`.
3. **Parse-on-demand**: Implement a `Config` struct that stores a raw `String` and parses it into a `HashMap<String, String>` lazily using `OnceLock`, providing `get(&self, key: &str) -> Option<&str>`.
