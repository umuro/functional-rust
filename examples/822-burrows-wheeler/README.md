# 822: Burrows-Wheeler Transform

**Difficulty:** 4  **Level:** Advanced

Permute a string into a form that compresses dramatically better — the transformation at the heart of bzip2.

## The Problem This Solves

The Burrows-Wheeler Transform (BWT) rearranges a string's characters so that identical characters cluster together, making the output far more compressible by run-length encoding or move-to-front coding. It's a reversible transformation — you can recover the original string exactly from the BWT output plus a single index.

BWT is the core of bzip2 compression, widely used for compressing source code, log files, and biological sequences. It also underpins the FM-index, the data structure behind modern short-read DNA aligners (BWA, Bowtie). Understanding BWT is essential for anyone working with text compression algorithms or genomic data processing.

This example implements both the forward transform (string → BWT + index) and the inverse (BWT + index → original string), allowing you to verify correctness and understand the structure.

## The Intuition

**Forward BWT**: append a sentinel character `$` (lexicographically smallest) to the string. Form all n+1 cyclic rotations. Sort them lexicographically. The BWT is the last column of this sorted matrix. The index tells you which row is the original string (the rotation starting with the real first character followed by `$` at the end).

Why does this compress well? Rows with the same prefix (sorted together) tend to end with the same character — because strings with similar suffixes often have similar preceding characters. This creates runs of identical characters in the last column, which RLE and move-to-front compress efficiently.

**Inverse BWT**: from the last column `L`, compute the first column `F` by sorting. Build `T[i]` = the rank of `L[i]` among equal characters in `L`. Then follow the chain: start at `index`, next = `T[index]`, until you've recovered all characters.

O(n log n) for sorting rotations. In Rust, we avoid materializing all rotations (O(n²) space) by sorting indices with a custom comparator that compares rotations as slices.

## How It Works in Rust

```rust
fn bwt_forward(s: &str) -> (Vec<u8>, usize) {
    let mut bytes: Vec<u8> = s.bytes().collect();
    bytes.push(b'$'); // sentinel — must be lexicographically smallest
    let n = bytes.len();

    // Sort rotation indices without materializing rotations (saves O(n²) space)
    let mut indices: Vec<usize> = (0..n).collect();
    indices.sort_unstable_by(|&a, &b| {
        // Compare rotation a vs rotation b as circular slices
        for k in 0..n {
            let ca = bytes[(a + k) % n];
            let cb = bytes[(b + k) % n];
            if ca != cb { return ca.cmp(&cb); }
        }
        std::cmp::Ordering::Equal
    });

    // Last column = character before each sorted rotation's start
    let last_col: Vec<u8> = indices.iter()
        .map(|&i| bytes[(i + n - 1) % n])
        .collect();

    // Original string is the rotation that starts with bytes[0]
    // and has $ at position n-1, which sorts as the row where index=0
    let orig_row = indices.iter().position(|&i| i == 0).unwrap();
    (last_col, orig_row)
}

fn bwt_inverse(last_col: &[u8], orig_row: usize) -> Vec<u8> {
    let n = last_col.len();

    // First column = sort of last column
    let mut first_col = last_col.to_vec();
    first_col.sort_unstable();

    // T[i] = where does last_col[i] map in first_col?
    // i.e. rank of last_col[i] among equal characters (stable sort order)
    let mut rank = vec![0usize; n];
    let mut count = [0usize; 256];
    for &b in last_col { count[b as usize] += 1; }

    // Prefix sum to find start positions in first_col
    let mut start = [0usize; 256];
    for i in 1..256 { start[i] = start[i-1] + count[i-1]; }

    // Assign T[i] based on occurrence rank
    let mut seen = [0usize; 256];
    let mut t = vec![0usize; n];
    for (i, &b) in last_col.iter().enumerate() {
        t[i] = start[b as usize] + seen[b as usize];
        seen[b as usize] += 1;
    }

    // Follow T chain from orig_row to recover original string
    let mut result = vec![0u8; n];
    let mut row = orig_row;
    for i in (0..n).rev() {
        result[i] = last_col[row];
        row = t[row];
    }
    // Remove sentinel $
    result.into_iter().filter(|&b| b != b'$').collect()
}
```

`sort_unstable_by` on indices is the key space optimization — comparing rotations via modular indexing `(a + k) % n` instead of allocating `n` separate strings. The `count`/`start`/`seen` arrays replace a full sort during inversion, running in O(n + |alphabet|).

## What This Unlocks

- **bzip2 compression**: BWT + move-to-front + Huffman coding achieves better compression ratios than deflate/gzip for text and source code.
- **FM-index for DNA alignment**: the suffix array + BWT enables O(m) pattern search in a compressed index of the genome — used in BWA, Bowtie2, and HISAT2.
- **Text index compression**: BWT-based indexes store the entire human genome in ~750MB while supporting fast substring search.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Rotation representation | String list (allocates all rotations) | Sorted indices with modular indexing — O(1) space per rotation |
| Sorting comparator | `String.compare` on explicit strings | Closure with `% n` arithmetic — no allocation |
| Byte vs char | `Bytes.t` / `Char.code` | `u8` directly — BWT is inherently a byte operation |
| Inversion T-array | Functional fold with counters | Mutable `count`/`start`/`seen` arrays — O(|alphabet|) |
| Sentinel character | Any lexicographically minimal char | `b'$'` — explicit byte literal |
