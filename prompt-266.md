Convert this OCaml example to idiomatic Rust.

Directory: examples/266-recursive-descent-parser/

## OCaml source
```ocaml
type expr = Num of int | Add of expr * expr | Mul of expr * expr

let rec parse_expr tokens = 
  let left, rest = parse_term tokens in
  match rest with
  | "+" :: rest' ->
    let right, rest'' = parse_expr rest' in
    (Add (left, right), rest'')
  | _ -> (left, rest)
and parse_term tokens =
  let left, rest = parse_atom tokens in
  match rest with
  | "*" :: rest' ->
    let right, rest'' = parse_term rest' in
    (Mul (left, right), rest'')
  | _ -> (left, rest)
and parse_atom = function
  | n :: rest -> (Num (int_of_string n), rest)
  | [] -> failwith "unexpected end"

let rec eval = function
  | Num n -> n | Add (a,b) -> eval a + eval b | Mul (a,b) -> eval a * eval b

let () =
  let tokens = ["2";"+";"3";"*";"4"] in
  let ast, _ = parse_expr tokens in
  Printf.printf "2+3*4 = %d\n" (eval ast)
```

## Topic
Parsing arithmetic expressions into an AST with recursive descent, demonstrating mutual recursion and algebraic data types.

Read CLAUDE.md in this directory — it defines all quality standards, file structure, and self-verification steps. Follow it exactly.

When done, report:
DONE — 266-recursive-descent-parser — cargo fmt ✓ clippy ✓ test ✓ [N tests passed]
