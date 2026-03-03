Convert this OCaml example to idiomatic Rust.

Directory: examples/257-state-machine-turnstile/

## OCaml source
```ocaml
type state = Locked | Unlocked
type event = Coin | Push

let transition state event = match state, event with
  | Locked, Coin -> Unlocked
  | Unlocked, Push -> Locked
  | Locked, Push -> Locked
  | Unlocked, Coin -> Unlocked

let state_name = function Locked -> "Locked" | Unlocked -> "Unlocked"

let () =
  let events = [Coin; Push; Push; Coin; Coin; Push] in
  let final = List.fold_left (fun s e ->
    let s' = transition s e in
    Printf.printf "%s -> %s\n" (state_name s) (state_name s');
    s'
  ) Locked events in
  Printf.printf "Final: %s\n" (state_name final)
```

## Topic
Encoding state machines with variants and pattern matching — Turnstile
Difficulty: Intermediate | Category: State machines

Read CLAUDE.md in this directory — it defines all quality standards, file structure, and self-verification steps. Follow it exactly.

When done, report:
DONE — 257-state-machine-turnstile — cargo fmt ✓ clippy ✓ test ✓ [N tests passed]
