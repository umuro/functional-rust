(* Example 167: Recursive Parser *)
(* Recursive parsers for recursive grammars *)

type 'a parse_result = ('a * string, string) result
type 'a parser = string -> 'a parse_result

let satisfy pred desc : char parser = fun input ->
  if String.length input > 0 && pred input.[0] then
    Ok (input.[0], String.sub input 1 (String.length input - 1))
  else Error (Printf.sprintf "Expected %s" desc)

let tag expected : string parser = fun input ->
  let len = String.length expected in
  if String.length input >= len && String.sub input 0 len = expected then
    Ok (expected, String.sub input len (String.length input - len))
  else Error (Printf.sprintf "Expected \"%s\"" expected)

let many0 p : 'a list parser = fun input ->
  let rec go acc r = match p r with Ok (v, r') -> go (v::acc) r' | Error _ -> Ok (List.rev acc, r)
  in go [] input

let ws0 : unit parser = fun input ->
  let rec skip i = if i < String.length input &&
    (input.[i] = ' ' || input.[i] = '\t' || input.[i] = '\n') then skip (i+1) else i in
  let i = skip 0 in Ok ((), String.sub input i (String.length input - i))

(* Recursive data type: nested lists *)
type sexp = Atom of string | List of sexp list

(* Approach 1: Direct mutual recursion (OCaml makes this easy) *)
let rec parse_sexp input =
  match satisfy (fun c -> c >= 'a' && c <= 'z') "letter" input with
  | Ok (c, rest) ->
    let rec go acc r =
      match satisfy (fun c -> c >= 'a' && c <= 'z' || c >= '0' && c <= '9') "alnum" r with
      | Ok (c, r') -> go (acc ^ String.make 1 c) r'
      | Error _ -> Ok (Atom acc, r) in
    go (String.make 1 c) rest
  | Error _ -> parse_sexp_list input

and parse_sexp_list input =
  match tag "(" input with
  | Error e -> Error e
  | Ok (_, rest) ->
    let rec go acc remaining =
      match ws0 remaining with
      | Ok ((), r) ->
        (match tag ")" r with
         | Ok (_, r') -> Ok (List (List.rev acc), r')
         | Error _ ->
           match parse_sexp r with
           | Ok (v, r') -> go (v :: acc) r'
           | Error e -> Error e)
      | Error e -> Error e
    in
    go [] rest

(* Approach 2: Using a ref cell for forward declaration *)
let make_recursive () =
  let p = ref (fun _ -> Error "uninitialized" : sexp parse_result) in
  let parser : sexp parser = fun input -> !p input in
  let set_parser (actual : sexp parser) = p := actual in
  (parser, set_parser)

(* Approach 3: Fix-point combinator *)
let fix (f : 'a parser -> 'a parser) : 'a parser =
  let rec p input = (f p) input in
  p

let nested_parens : int parser =
  fix (fun self -> fun input ->
    match tag "(" input with
    | Ok (_, rest) ->
      (match self rest with
       | Ok (depth, r2) ->
         (match tag ")" r2 with
          | Ok (_, r3) -> Ok (depth + 1, r3)
          | Error e -> Error e)
       | Error e -> Error e)
    | Error _ -> Ok (0, input)  (* base case *)
  )

(* Tests *)
let () =
  (match parse_sexp "hello" with
   | Ok (Atom "hello", "") -> ()
   | _ -> failwith "Test 1");
  (match parse_sexp "(a b c)" with
   | Ok (List [Atom "a"; Atom "b"; Atom "c"], "") -> ()
   | _ -> failwith "Test 2");
  (match parse_sexp "(a (b c))" with
   | Ok (List [Atom "a"; List [Atom "b"; Atom "c"]], "") -> ()
   | _ -> failwith "Test 3");

  assert (nested_parens "((()))" = Ok (3, ""));
  assert (nested_parens "()" = Ok (1, ""));
  assert (nested_parens "" = Ok (0, ""));

  print_endline "✓ All tests passed"
