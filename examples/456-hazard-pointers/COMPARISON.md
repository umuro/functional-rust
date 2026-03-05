# Hazard Pointers

Safe memory reclamation for lock-free structures.

## Concept
1. Thread announces pointer it's using (hazard pointer)
2. Before freeing, check all hazard pointers
3. If protected, defer; if not, free

### Rust Pattern
```rust
hp.protect(ptr);           // Announce
// ... use ptr safely ...
hp.clear();                // Done

// Reclaimer
for retired in &retired_list {
    if !any_hazard_protects(retired) {
        drop(retired);     // Safe to free
    }
}
```
