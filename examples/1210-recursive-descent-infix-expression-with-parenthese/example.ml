(* Recursive Descent — Infix Expression with Parentheses *)
(* Full expression parser with operator precedence *)

type expr = Num of int | Binop of char * expr * expr

let rec eval = function
  | Num n -> n
  | Binop ('+', a, b) -> eval a + eval b
  | Binop ('-', a, b) -> eval a - eval b
  | Binop ('*', a, b) -> eval a * eval b
  | Binop ('/', a, b) -> eval a / eval b
  | Binop _ -> failwith "unknown op"

(* Simple evaluator using Dijkstra's shunting yard *)
let calc s =
  let tokens = String.split_on_char ' ' s in
  (* Simple recursive descent *)
  let pos = ref 0 in
  let toks = Array.of_list tokens in
  let peek () = if !pos < Array.length toks then toks.(!pos) else "" in
  let consume () = let t = peek () in incr pos; t in
  let rec expr () =
    let left = term () in
    match peek () with
    | "+" -> ignore (consume ()); Binop ('+', left, expr ())
    | "-" -> ignore (consume ()); Binop ('-', left, expr ())
    | _ -> left
  and term () =
    let left = atom () in
    match peek () with
    | "*" -> ignore (consume ()); Binop ('*', left, term ())
    | "/" -> ignore (consume ()); Binop ('/', left, term ())
    | _ -> left
  and atom () = Num (int_of_string (consume ()))
  in
  eval (expr ())

let () = Printf.printf "2 + 3 * 4 = %d\n" (calc "2 + 3 * 4")
