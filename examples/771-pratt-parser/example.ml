(* Pratt parser in OCaml — handles (1+2)*3, unary minus, right-assoc ^ *)

(* ── Tokens ─────────────────────────────────────────────────────────────────── *)
type token = TNum of float | TPlus | TMinus | TStar | TSlash | TCaret
           | TLParen | TRParen | TEof

(* ── AST ─────────────────────────────────────────────────────────────────────── *)
type expr =
  | Num of float
  | Unary  of char * expr
  | Binary of char * expr * expr

let rec eval = function
  | Num n -> n
  | Unary  ('-', e) -> -. (eval e)
  | Unary  (_, e)   -> eval e
  | Binary ('+', a, b) -> eval a +. eval b
  | Binary ('-', a, b) -> eval a -. eval b
  | Binary ('*', a, b) -> eval a *. eval b
  | Binary ('/', a, b) -> eval a /. eval b
  | Binary ('^', a, b) -> Float.pow (eval a) (eval b)
  | Binary _ -> 0.0

(* ── Lexer ─────────────────────────────────────────────────────────────────── *)
let tokenize s =
  let tokens = ref [] in
  let i = ref 0 in
  let len = String.length s in
  while !i < len do
    let c = s.[!i] in
    incr i;
    match c with
    | ' ' | '\t' -> ()
    | '+' -> tokens := TPlus   :: !tokens
    | '-' -> tokens := TMinus  :: !tokens
    | '*' -> tokens := TStar   :: !tokens
    | '/' -> tokens := TSlash  :: !tokens
    | '^' -> tokens := TCaret  :: !tokens
    | '(' -> tokens := TLParen :: !tokens
    | ')' -> tokens := TRParen :: !tokens
    | c when c >= '0' && c <= '9' ->
      let start = !i - 1 in
      while !i < len && (s.[!i] >= '0' && s.[!i] <= '9' || s.[!i] = '.') do incr i done;
      tokens := TNum (float_of_string (String.sub s start (!i - start))) :: !tokens
    | _ -> ()
  done;
  List.rev (TEof :: !tokens)

(* ── Pratt parser ─────────────────────────────────────────────────────────── *)
let tokens = ref []
let current () = match !tokens with t :: _ -> t | [] -> TEof
let consume () = match !tokens with _ :: rest -> tokens := rest | [] -> ()

let infix_bp = function
  | TPlus | TMinus -> (10, 11)
  | TStar | TSlash -> (20, 21)
  | TCaret          -> (30, 29)  (* right-associative: right bp < left bp *)
  | _ -> (0, 0)

let op_char = function
  | TPlus -> '+' | TMinus -> '-' | TStar -> '*' | TSlash -> '/' | TCaret -> '^'
  | _ -> '?'

let rec parse_bp min_bp =
  let left = ref (parse_nud ()) in
  let continue = ref true in
  while !continue do
    let tok = current () in
    let (lbp, rbp) = infix_bp tok in
    if lbp <= min_bp then continue := false
    else begin
      consume ();
      let right = parse_bp rbp in
      left := Binary (op_char tok, !left, right)
    end
  done;
  !left

and parse_nud () =
  match current () with
  | TNum n -> consume (); Num n
  | TMinus ->
    consume ();
    let e = parse_bp 25 in  (* higher than * to bind tight *)
    Unary ('-', e)
  | TLParen ->
    consume ();
    let e = parse_bp 0 in
    (match current () with TRParen -> consume () | _ -> failwith "expected ')'");
    e
  | t -> failwith (Printf.sprintf "unexpected token: %d" (Obj.tag (Obj.repr t)))

let parse s =
  tokens := tokenize s;
  parse_bp 0

let () =
  let tests = [
    "(1 + 2) * 3",       (* = 9 *)
    "2 ^ 3 ^ 2",         (* = 512, right-assoc: 2^(3^2) = 2^9 *)
    "-2 * 3",            (* = -6 *)
    "1 + 2 * 3 - 4 / 2", (* = 5 *)
  ] in
  List.iter (fun s ->
    Printf.printf "%s = %g\n" s (eval (parse s))
  ) tests
