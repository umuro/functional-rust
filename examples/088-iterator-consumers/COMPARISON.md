## Core Insight

Adapters are lazy; consumers are eager. A consumer pulls values through the chain and produces a final result. Without a consumer, the iterator chain does nothing.

## OCaml Approach
- `Seq.fold_left` is the universal consumer
- `List.of_seq` to collect
- `Seq.iter` for side effects

## Rust Approach
- `.sum()`, `.product()`, `.count()` — specific consumers
- `.collect()` — universal collector
- `.for_each()` — side-effect consumer
- `.fold()` — general-purpose consumer

## Comparison Table

| Consumer | OCaml | Rust |
|----------|-------|------|
| Sum | `Seq.fold_left (+) 0` | `.sum()` |
| Collect | `List.of_seq` | `.collect::<Vec<_>>()` |
| Count | `Seq.fold_left (fun n _ -> n+1) 0` | `.count()` |
| For each | `Seq.iter f` | `.for_each(f)` |
| Fold | `Seq.fold_left f init` | `.fold(init, f)` |
