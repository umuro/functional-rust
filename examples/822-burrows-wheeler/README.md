📖 **[View on hightechmind.io →](https://hightechmind.io/rust/822-burrows-wheeler)**

---

# Burrows-Wheeler Transform
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Data compression algorithms like bzip2 achieve high compression ratios by first applying the Burrows-Wheeler Transform (BWT) to rearrange text so that similar characters cluster together, making run-length encoding and move-to-front coding highly effective. The BWT is a reversible transformation: it takes a string, generates all rotations, sorts them lexicographically, and returns the last column. The magic is that similar contexts cluster the last-column characters. The inverse BWT recovers the original string exactly. BWT is also used in bioinformatics for FM-index construction, enabling compressed full-text indexes that power genome alignment tools like BWA and Bowtie.

## Learning Outcomes

- Understand BWT as the last column of the sorted rotation matrix
- Implement forward BWT: generate all rotations, sort, take last characters, record original row index
- Implement inverse BWT using the first/last column correspondence and the rank property
- Recognize why BWT aids compression: characters with similar contexts appear together
- Connect BWT to the FM-index and suffix array for full-text search

## Rust Application

```rust
pub fn bwt_transform(s: &str) -> (String, usize) {
    let n = s.len();
    let mut rotations: Vec<usize> = (0..n).collect();
    rotations.sort_by(|&a, &b| {
        let ra = s[a..].chars().chain(s[..a].chars());
        let rb = s[b..].chars().chain(s[..b].chars());
        ra.cmp(rb)
    });
    let bwt: String = rotations.iter().map(|&i| s.chars().nth((i + n - 1) % n).unwrap()).collect();
    let original_row = rotations.iter().position(|&i| i == 0).unwrap();
    (bwt, original_row)
}
```

Rather than materializing all n rotations as strings (O(n^2) memory), we sort indices with a comparator that simulates rotation comparison using `chain`. This reduces memory from O(n^2) to O(n log n) for the sort. The last character of rotation i is at position `(i + n - 1) % n` in the original string. The `original_row` index is needed for inverse transformation. Rust's iterator chaining makes the rotation simulation clean and allocation-free.

## OCaml Approach

OCaml implements BWT with `Array.init n (fun i -> i)` for rotation indices and `Array.sort` with a comparator simulating rotation. `String.get s ((i + n - 1) mod n)` gets the last character. OCaml's `String.init` builds the BWT string. The inverse BWT uses a `Array.sort`-built rank table mapping characters to their position in the first column. OCaml's functional style uses `Array.fold_left` for the inverse reconstruction loop. The `String.concat "" [s1; s2]` approach for rotation comparison is simpler but allocates O(n) per comparison.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Rotation comparison | `chain` iterator, zero allocation | `String.sub` concat or simulated |
| BWT string | `String` collected from iterator | `String.init` with `Array.get` |
| Inverse LF-mapping | Rank array via counting | Array sort + index correspondence |
| Memory (sort) | O(n) indices + O(n log n) sort | Same approach |
| Rotation materialization | Avoided via index sort | Avoided similarly |
| Compression use | Foundation for bzip2-like | Same theoretical role |

## Exercises

1. Implement the inverse BWT to recover the original string from `(bwt, original_row)`.
2. Integrate BWT with move-to-front encoding and run-length encoding to build a simple compressor.
3. Measure compression ratio on English text vs. random bytes to demonstrate BWT's effectiveness.
4. Build a simplified FM-index using the BWT and a rank/select data structure for O(m) pattern search.
5. Compare BWT-based compression ratios with deflate (gzip) on natural language text.
