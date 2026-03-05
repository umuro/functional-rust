# Sentinel Values vs Result — Comparison

## Core Insight
Sentinel values (-1, null, "") encode failure in the success type. Option/Result use separate types, making failure handling compiler-enforced.

## OCaml Approach
- Same progression: sentinel → Option → Result
- OCaml culture strongly favors Option for lookups
- `List.assoc_opt`, `Hashtbl.find_opt` return Option
- No null in OCaml — already safer than most languages

## Rust Approach
- Identical progression: sentinel → Option → Result
- `Option<usize>` instead of `-1i32` for "not found"
- Standard library consistently uses Option/Result
- No null at all — `Option<T>` is the only way to express absence

## Comparison Table

| Aspect | Sentinel | Option | Result |
|--------|----------|--------|--------|
| Type safety | None | Compiler-enforced | Compiler-enforced |
| Error info | Implicit | "missing" only | Why it's missing |
| Ambiguity | -1 might be valid | None is clear | Err(reason) is clear |
| Forgotten check | Silent bug | Compile error | Compile error |
| Use when | Never (legacy code) | Absence is expected | Absence needs explanation |
