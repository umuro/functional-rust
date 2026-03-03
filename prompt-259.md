Convert this OCaml example to idiomatic Rust.

Directory: examples/259-result-monad-error-chaining/

## OCaml source
```ocaml
let ( >>= ) r f = match r with
  | Error _ as e -> e
  | Ok x -> f x

let parse_int s =
  match int_of_string_opt s with
  | Some n -> Ok n
  | None -> Error ("Not an integer: " ^ s)

let check_positive n =
  if n > 0 then Ok n else Error "Must be positive"

let check_even n =
  if n mod 2 = 0 then Ok n else Error "Must be even"

let validate s =
  parse_int s >>= check_positive >>= check_even

let () =
  List.iter (fun s ->
    match validate s with
    | Ok n -> Printf.printf "%s -> Ok %d\n" s n
    | Error e -> Printf.printf "%s -> Error: %s\n" s e
  ) ["42"; "-3"; "abc"; "7"]
```

## Topic
Result monad with bind for railway-oriented programming
Difficulty: Intermediate | Category: Monadic patterns

Read CLAUDE.md in this directory — it defines all quality standards, file structure, and self-verification steps. Follow it exactly.

When done, report:
DONE — 259-result-monad-error-chaining — cargo fmt ✓ clippy ✓ test ✓ [N tests passed]
