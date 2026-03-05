(* Recursive descent parser in OCaml *)

(* ── AST ─────────────────────────────────────────────────────────────────────── *)
type expr =
  | Num of float
  | Add of expr * expr
  | Sub of expr * expr
  | Mul of expr * expr
  | Div of expr * expr

let rec eval = function
  | Num n     -> n
  | Add (a,b) -> eval a +. eval b
  | Sub (a,b) -> eval a -. eval b
  | Mul (a,b) -> eval a *. eval b
  | Div (a,b) -> eval a /. eval b

(* ── Lexer (character-level) ─────────────────────────────────────────────────── *)
let s = ref ""
let pos = ref 0

let peek () = if !pos < String.length !s then Some !s.[!pos] else None
let next () = incr pos

let skip_ws () =
  while match peek () with Some ' ' | Some '\t' -> true | _ -> false do next () done

let parse_number () =
  skip_ws ();
  let start = !pos in
  (match peek () with Some '-' -> next () | _ -> ());
  while (match peek () with Some c when c >= '0' && c <= '9' || c = '.' -> true | _ -> false) do next () done;
  let tok = String.sub !s start (!pos - start) in
  float_of_string tok

(* ── Grammar ─────────────────────────────────────────────────────────────────── *)
let rec parse_expr () =
  let left = ref (parse_term ()) in
  skip_ws ();
  let continue = ref true in
  while !continue do
    match peek () with
    | Some '+' -> next (); let r = parse_term () in left := Add (!left, r); skip_ws ()
    | Some '-' -> next (); let r = parse_term () in left := Sub (!left, r); skip_ws ()
    | _ -> continue := false
  done;
  !left

and parse_term () =
  let left = ref (parse_factor ()) in
  skip_ws ();
  let continue = ref true in
  while !continue do
    match peek () with
    | Some '*' -> next (); let r = parse_factor () in left := Mul (!left, r); skip_ws ()
    | Some '/' -> next (); let r = parse_factor () in left := Div (!left, r); skip_ws ()
    | _ -> continue := false
  done;
  !left

and parse_factor () =
  skip_ws ();
  match peek () with
  | Some '(' ->
    next ();
    let e = parse_expr () in
    skip_ws ();
    (match peek () with Some ')' -> next () | _ -> failwith "expected ')'");
    e
  | _ -> Num (parse_number ())

let parse input =
  s := input; pos := 0;
  parse_expr ()

let () =
  let tests = [
    "1 + 2 * 3",        (* = 7 *)
    "(1 + 2) * 3",      (* = 9 *)
    "10 / 2 - 3",       (* = 2 *)
    "2 * (3 + 4) / 2",  (* = 7 *)
  ] in
  List.iter (fun expr ->
    let result = eval (parse expr) in
    Printf.printf "%s = %g\n" expr result
  ) tests
