📖 **[View on hightechmind.io →](https://hightechmind.io/rust/369-cow-clone-on-write)**

---

# 369: Clone-on-Write (Cow) Pattern

**Difficulty:** 3  **Level:** Advanced

Delay cloning until modification is actually needed - zero-cost abstraction for read-heavy workloads.

## The Problem This Solves

Many functions might modify their input but often don't need to. A string sanitizer might escape special characters - but most strings have none. A normalizer might trim whitespace - but most strings are already trimmed.

Without Cow, you either:
1. Always clone (wasteful for unmodified inputs)
2. Return Option/Result indicating change (awkward API)
3. Mutate in place (requires &mut, loses original)

Cow lets you return a borrowed reference when no change is needed, and an owned value when modification occurs - with the same type.

## How It Works

`Cow<'a, T>` is an enum:
- `Cow::Borrowed(&'a T)` - references existing data
- `Cow::Owned(T::Owned)` - owns allocated data

The caller doesn't need to know which variant they got - both deref to `&T`.

## Key Benefit

Zero allocation in the common case, automatic allocation when needed. Perfect for normalization, sanitization, and transformation functions.
