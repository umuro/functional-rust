# Partition Results

| Concept | Rust |
|---------|------|
| Split | `partition(Result::is_ok)` |
| Extract Ok | `.flatten()` or `.map(unwrap)` |
| Extract Err | `.map(unwrap_err)` |
