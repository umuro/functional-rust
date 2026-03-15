(* Example 168: Expression Parser *)
(* Expression parser with precedence (Pratt parsing technique) *)

type 'a parse_result = ('a * string, string) result
type 'a parser = string -> 'a parse_result

type expr =
  | Num of float
  | BinOp of string * expr * expr
  | UnaryMinus of expr

let ws0 input =
  let rec skip i = if i < String.length input &&
    (input.[i] = ' ' || input.[i] = '\t') then skip (i+1) else i in
  let i = skip 0 in String.sub input i (String.length input - i)

(* Parse a number *)
let parse_number input =
  let s = ws0 input in
  let len = String.length s in
  let pos = ref 0 in
  if !pos < len && s.[!pos] = '-' then incr pos;
  while !pos < len && s.[!pos] >= '0' && s.[!pos] <= '9' do incr pos done;
  if !pos < len && s.[!pos] = '.' then begin
    incr pos;
    while !pos < len && s.[!pos] >= '0' && s.[!pos] <= '9' do incr pos done end;
  if !pos = 0 || (!pos = 1 && s.[0] = '-') then Error "Expected number"
  else Ok (Num (float_of_string (String.sub s 0 !pos)),
           String.sub s !pos (len - !pos))

(* Approach 1: Pratt parser *)
let prefix_binding_power = function
  | "-" -> Some 9
  | _ -> None

let infix_binding_power = function
  | "+" | "-" -> Some (5, 6)
  | "*" | "/" -> Some (7, 8)
  | "^" -> Some (10, 9)  (* right-associative *)
  | _ -> None

let parse_op input =
  let s = ws0 input in
  if String.length s > 0 then
    let c = String.make 1 s.[0] in
    if c = "+" || c = "-" || c = "*" || c = "/" || c = "^" then
      Ok (c, String.sub s 1 (String.length s - 1))
    else Error "Expected operator"
  else Error "Expected operator"

let rec pratt_expr min_bp input =
  let s = ws0 input in
  (* Prefix: unary minus or atom *)
  let lhs_result =
    if String.length s > 0 && s.[0] = '(' then
      let inner = String.sub s 1 (String.length s - 1) in
      match pratt_expr 0 inner with
      | Ok (e, rest) ->
        let r = ws0 rest in
        if String.length r > 0 && r.[0] = ')' then
          Ok (e, String.sub r 1 (String.length r - 1))
        else Error "Expected ')'"
      | Error e -> Error e
    else if String.length s > 0 && s.[0] = '-' then
      match prefix_binding_power "-" with
      | Some rbp ->
        let rest = String.sub s 1 (String.length s - 1) in
        (match pratt_expr rbp rest with
         | Ok (rhs, rem) -> Ok (UnaryMinus rhs, rem)
         | Error e -> Error e)
      | None -> parse_number s
    else parse_number s
  in
  match lhs_result with
  | Error e -> Error e
  | Ok (lhs, rest) -> pratt_loop min_bp lhs rest

and pratt_loop min_bp lhs input =
  match parse_op input with
  | Error _ -> Ok (lhs, input)
  | Ok (op, _) ->
    match infix_binding_power op with
    | None -> Ok (lhs, input)
    | Some (lbp, rbp) ->
      if lbp < min_bp then Ok (lhs, input)
      else
        match parse_op input with
        | Ok (_, after_op) ->
          (match pratt_expr rbp after_op with
           | Ok (rhs, rem) -> pratt_loop min_bp (BinOp (op, lhs, rhs)) rem
           | Error e -> Error e)
        | Error e -> Error e

let parse_expr = pratt_expr 0

(* Tests *)
let () =
  (match parse_expr "1 + 2" with
   | Ok (BinOp ("+", Num 1., Num 2.), "") -> ()
   | _ -> failwith "Test 1");

  (match parse_expr "1 + 2 * 3" with
   | Ok (BinOp ("+", Num 1., BinOp ("*", Num 2., Num 3.)), "") -> ()
   | _ -> failwith "Test 2: precedence");

  (match parse_expr "(1 + 2) * 3" with
   | Ok (BinOp ("*", BinOp ("+", Num 1., Num 2.), Num 3.), "") -> ()
   | _ -> failwith "Test 3: parens");

  print_endline "✓ All tests passed"
