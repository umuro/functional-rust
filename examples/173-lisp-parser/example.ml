(* Example 173: Lisp / S-expression Parser *)
(* S-expressions: atoms, lists, nested structures *)

type 'a parse_result = ('a * string, string) result
type 'a parser = string -> 'a parse_result

type sexp =
  | Atom of string
  | Number of float
  | Str of string
  | List of sexp list
  | Bool of bool
  | Nil

let ws0 input =
  let rec skip i = if i < String.length input &&
    (input.[i] = ' ' || input.[i] = '\t' || input.[i] = '\n' || input.[i] = '\r') then skip (i+1) else i in
  let i = skip 0 in String.sub input i (String.length input - i)

(* Approach 1: Parse atom (symbol) *)
let is_atom_char c =
  (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') ||
  (c >= '0' && c <= '9') || c = '_' || c = '-' || c = '+' ||
  c = '*' || c = '/' || c = '?' || c = '!' || c = '.'

let parse_atom input =
  let s = ws0 input in
  let len = String.length s in
  let pos = ref 0 in
  while !pos < len && is_atom_char s.[!pos] do incr pos done;
  if !pos = 0 then Error "Expected atom"
  else
    let token = String.sub s 0 !pos in
    let rest = String.sub s !pos (len - !pos) in
    (* Classify *)
    if token = "nil" then Ok (Nil, rest)
    else if token = "#t" || token = "true" then Ok (Bool true, rest)
    else if token = "#f" || token = "false" then Ok (Bool false, rest)
    else
      try Ok (Number (float_of_string token), rest)
      with _ -> Ok (Atom token, rest)

(* Approach 2: Parse string literal *)
let parse_string input =
  let s = ws0 input in
  if String.length s = 0 || s.[0] <> '"' then Error "Expected string"
  else
    let buf = Buffer.create 32 in
    let rec go i =
      if i >= String.length s then Error "Unterminated string"
      else if s.[i] = '"' then
        Ok (Str (Buffer.contents buf), String.sub s (i+1) (String.length s - i - 1))
      else if s.[i] = '\\' && i + 1 < String.length s then begin
        (match s.[i+1] with
         | 'n' -> Buffer.add_char buf '\n'
         | 't' -> Buffer.add_char buf '\t'
         | '"' -> Buffer.add_char buf '"'
         | '\\' -> Buffer.add_char buf '\\'
         | c -> Buffer.add_char buf '\\'; Buffer.add_char buf c);
        go (i + 2)
      end else begin
        Buffer.add_char buf s.[i]; go (i + 1)
      end
    in go 1

(* Approach 3: Full S-expression parser *)
let rec parse_sexp input =
  let s = ws0 input in
  if String.length s = 0 then Error "Unexpected EOF"
  else if s.[0] = '(' then parse_list s
  else if s.[0] = '\'' then  (* quote *)
    match parse_sexp (String.sub s 1 (String.length s - 1)) with
    | Ok (v, rest) -> Ok (List [Atom "quote"; v], rest)
    | Error e -> Error e
  else if s.[0] = '"' then parse_string s
  else parse_atom s

and parse_list input =
  let rest = String.sub input 1 (String.length input - 1) in
  let rec go acc remaining =
    let remaining = ws0 remaining in
    if String.length remaining = 0 then Error "Unterminated list"
    else if remaining.[0] = ')' then
      Ok (List (List.rev acc), String.sub remaining 1 (String.length remaining - 1))
    else
      match parse_sexp remaining with
      | Ok (v, rest) -> go (v :: acc) rest
      | Error e -> Error e
  in go [] rest

let rec sexp_to_string = function
  | Atom s -> s
  | Number n -> string_of_float n
  | Str s -> Printf.sprintf "\"%s\"" s
  | Bool true -> "#t"
  | Bool false -> "#f"
  | Nil -> "nil"
  | List items -> "(" ^ String.concat " " (List.map sexp_to_string items) ^ ")"

(* Tests *)
let () =
  assert (parse_sexp "hello" = Ok (Atom "hello", ""));
  assert (parse_sexp "42" = Ok (Number 42., ""));
  assert (parse_sexp "\"hi\"" = Ok (Str "hi", ""));
  assert (parse_sexp "nil" = Ok (Nil, ""));
  assert (parse_sexp "#t" = Ok (Bool true, ""));

  (match parse_sexp "(+ 1 2)" with
   | Ok (List [Atom "+"; Number 1.; Number 2.], "") -> ()
   | _ -> failwith "List test");

  (match parse_sexp "(define (square x) (* x x))" with
   | Ok (List [Atom "define"; List [Atom "square"; Atom "x"];
               List [Atom "*"; Atom "x"; Atom "x"]], "") -> ()
   | _ -> failwith "Nested test");

  (match parse_sexp "'hello" with
   | Ok (List [Atom "quote"; Atom "hello"], "") -> ()
   | _ -> failwith "Quote test");

  print_endline "✓ All tests passed"
