(* Recursive Descent Parser — OCaml
   Parses arithmetic expressions into an AST, then evaluates them.
   Demonstrates mutual recursion and algebraic data types. *)

type expr = Num of int | Add of expr * expr | Mul of expr * expr

(* Idiomatic OCaml: mutually recursive parsing functions that thread
   the remaining token list through return values. *)
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
  | [] -> failwith "unexpected end of input"

(* Recursive evaluator — structural recursion over the AST *)
let rec eval = function
  | Num n -> n
  | Add (a, b) -> eval a + eval b
  | Mul (a, b) -> eval a * eval b

let () =
  let tokens = ["2"; "+"; "3"; "*"; "4"] in
  let ast, _ = parse_expr tokens in
  assert (eval ast = 14);                  (* 2 + (3*4), not (2+3)*4 *)

  let tokens2 = ["1"; "*"; "2"; "+"; "3"; "*"; "4"] in
  let ast2, _ = parse_expr tokens2 in
  assert (eval ast2 = 14);                 (* (1*2) + (3*4) = 2 + 12 *)

  let tokens3 = ["1"; "+"; "2"; "+"; "3"] in
  let ast3, _ = parse_expr tokens3 in
  assert (eval ast3 = 6);                  (* right-associative: 1+(2+3) *)

  Printf.printf "2+3*4 = %d\n" (eval ast);
  Printf.printf "1*2+3*4 = %d\n" (eval ast2);
  Printf.printf "1+2+3 = %d\n" (eval ast3);
  print_endline "ok"
