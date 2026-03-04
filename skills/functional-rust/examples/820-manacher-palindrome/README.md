# 820: Manacher's Algorithm

**Difficulty:** 4  **Level:** Advanced

Find the longest palindromic substring in O(n) by reusing mirror symmetry inside already-known palindromes — never re-examining a character twice.

## The Problem This Solves

The naïve approach to finding the longest palindrome expands around each center in O(n) per center, giving O(n²) total. For real-world use cases — DNA palindrome detection (restriction enzyme sites), string compression, natural language processing — O(n²) on multi-megabyte inputs is too slow.

Manacher's algorithm reduces this to O(n) by exploiting one insight: if you're inside a known large palindrome, the mirror of your current center already tells you a lower bound for your palindrome radius. The Z-box technique from the Z-algorithm applied to palindrome geometry.

The classic transform (inserting `#` between characters) eliminates the odd/even palindrome distinction entirely, letting you handle "racecar" and "abba" with identical code. This kind of uniform representation — making the irregular case disappear — is a hallmark of elegant algorithm design.

## The Intuition

Transform `"abc"` into `"#a#b#c#"` so every palindrome has an odd length and a definite center. Maintain `(c, r)`: the center and right boundary of the rightmost palindrome found so far. For each new center `i`: if `i < r`, initialize `P[i]` from the mirror `P[2c - i]`, capped at `r - i` (can't extend past the known palindrome). Then expand. If the expansion extends past `r`, update `(c, r)`. Each position is processed at most twice: once inside an existing palindrome (mirror lookup, O(1)) and once during expansion. Total: O(n).

OCaml and Rust implement identical logic; the difference is Rust's method syntax (`.min()`) versus OCaml's polymorphic `min` function.

## How It Works in Rust

```rust
// Transform "racecar" → "#r#a#c#e#c#a#r#"
// Every palindrome is now odd-length with a definite center
fn transform(s: &str) -> Vec<u8> {
    let mut t = Vec::with_capacity(2 * s.len() + 1);
    t.push(b'#');
    for b in s.bytes() {
        t.push(b);
        t.push(b'#');
    }
    t
}

fn manacher(t: &[u8]) -> Vec<usize> {
    let n = t.len();
    let mut p = vec![0usize; n];      // p[i] = palindrome radius at center i
    let (mut c, mut r) = (0usize, 0usize);  // Rightmost palindrome: center, right boundary

    for i in 0..n {
        if i < r {
            let mirror = 2 * c - i;
            p[i] = p[mirror].min(r - i);  // Mirror lower bound, capped at Z-box
        }
        // Expand around i — each character expanded at most once total
        let mut a = p[i] + 1;
        while i >= a && i + a < n && t[i - a] == t[i + a] {
            a += 1;
        }
        p[i] = a - 1;
        if i + p[i] > r {
            c = i;
            r = i + p[i];  // New rightmost palindrome
        }
    }
    p
}

fn longest_palindrome(s: &str) -> &str {
    let t = transform(s);
    let p = manacher(&t);
    // Find center with maximum radius
    let (best_c, best_r) = p.iter().enumerate()
        .max_by_key(|&(_, &r)| r)
        .map(|(i, &r)| (i, r))
        .unwrap();
    // Map transformed index back to original: start = (best_c - best_r) / 2
    let start = (best_c - best_r) / 2;
    &s[start..start + best_r]
}
```

The index mapping `(best_c - best_r) / 2` works because the `#` sentinels always occupy even positions in the transformed string, so dividing by 2 recovers the original index.

## What This Unlocks

- **Bioinformatics**: Palindromic DNA sequences are restriction enzyme recognition sites; Manacher finds all of them in O(n).
- **String compression**: Detecting the longest palindrome is a step in grammar-based compression and LZ-variant schemes.
- **Competitive programming**: Any problem asking about palindromic substrings in O(n) needs Manacher; it also generalizes to palindrome hashing and eertree (palindromic tree).

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Transform string | `String.concat "" (List.init ...)` | Byte-level `Vec<u8>`, `push` loop |
| Mutable radius array | `Array.make n 0` | `vec![0usize; n]` |
| Min of two values | `min a b` (polymorphic) | `a.min(b)` — method call |
| Centre tracking | `let c = ref 0; let r = ref 0` | `let (mut c, mut r) = (0, 0)` |
| Result extraction | `Array.fold_left` with index tracking | `.enumerate().max_by_key(...)` |
