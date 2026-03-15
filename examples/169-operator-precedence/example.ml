(* Example 169: Operator Precedence *)
(* Binary operators with left/right associativity and precedence levels *)

type 'a parse_result = ('a * string, string) result
type 'a parser = string -> 'a parse_result

type assoc = Left | Right

type op_info = {
  symbol: string;
  precedence: int;
  associativity: assoc;
}

type expr =
  | Num of float
  | BinOp of string * expr * expr

(* Approach 1: Table-driven operator precedence *)
let operators = [
  { symbol = "||"; precedence = 1; associativity = Left };
  { symbol = "&&"; precedence = 2; associativity = Left };
  { symbol = "=="; precedence = 3; associativity = Left };
  { symbol = "!="; precedence = 3; associativity = Left };
  { symbol = "<";  precedence = 4; associativity = Left };
  { symbol = ">";  precedence = 4; associativity = Left };
  { symbol = "<="; precedence = 4; associativity = Left };
  { symbol = ">="; precedence = 4; associativity = Left };
  { symbol = "+";  precedence = 5; associativity = Left };
  { symbol = "-";  precedence = 5; associativity = Left };
  { symbol = "*";  precedence = 6; associativity = Left };
  { symbol = "/";  precedence = 6; associativity = Left };
  { symbol = "%";  precedence = 6; associativity = Left };
  { symbol = "^";  precedence = 7; associativity = Right };
]

let find_op sym = List.find_opt (fun op -> op.symbol = sym) operators

let binding_power op_info =
  let base = op_info.precedence * 2 in
  match op_info.associativity with
  | Left -> (base, base + 1)    (* left: left bp < right bp *)
  | Right -> (base + 1, base)   (* right: left bp > right bp *)

let ws0 input =
  let rec skip i = if i < String.length input &&
    (input.[i] = ' ' || input.[i] = '\t') then skip (i+1) else i in
  let i = skip 0 in String.sub input i (String.length input - i)

let parse_number input =
  let s = ws0 input in
  let len = String.length s in
  let pos = ref 0 in
  while !pos < len && s.[!pos] >= '0' && s.[!pos] <= '9' do incr pos done;
  if !pos < len && s.[!pos] = '.' then begin
    incr pos;
    while !pos < len && s.[!pos] >= '0' && s.[!pos] <= '9' do incr pos done end;
  if !pos = 0 then Error "Expected number"
  else Ok (Num (float_of_string (String.sub s 0 !pos)),
           String.sub s !pos (len - !pos))

(* Try to parse a multi-char operator *)
let parse_operator input =
  let s = ws0 input in
  (* Try 2-char ops first, then 1-char *)
  let try_len len =
    if String.length s >= len then
      let sym = String.sub s 0 len in
      match find_op sym with
      | Some info -> Some (info, String.sub s len (String.length s - len))
      | None -> None
    else None in
  match try_len 2 with
  | Some result -> Ok result
  | None ->
    match try_len 1 with
    | Some result -> Ok result
    | None -> Error "Expected operator"

(* Approach 2: Pratt parser using the table *)
let rec parse_expr_pratt min_bp input =
  let (lhs, rest) = match parse_number input with
    | Ok r -> r
    | Error e -> raise (Failure e) in
  pratt_loop min_bp lhs rest

and pratt_loop min_bp lhs input =
  match parse_operator input with
  | Error _ -> (lhs, input)
  | Ok (info, _) ->
    let (lbp, rbp) = binding_power info in
    if lbp < min_bp then (lhs, input)
    else
      match parse_operator input with
      | Ok (_, after_op) ->
        let (rhs, rem) = parse_expr_pratt rbp after_op in
        pratt_loop min_bp (BinOp (info.symbol, lhs, rhs)) rem
      | Error _ -> (lhs, input)

let parse_expr input =
  try Ok (parse_expr_pratt 0 input)
  with Failure e -> Error e

(* Approach 3: Precedence climbing *)
let rec climb_expr input min_prec =
  let (mut_lhs, mut_rest) = match parse_number input with
    | Ok r -> r | Error e -> raise (Failure e) in
  let lhs = ref mut_lhs in
  let rest = ref mut_rest in
  let continue_loop = ref true in
  while !continue_loop do
    match parse_operator !rest with
    | Error _ -> continue_loop := false
    | Ok (info, _) ->
      if info.precedence < min_prec then continue_loop := false
      else begin
        let next_min = match info.associativity with
          | Left -> info.precedence + 1
          | Right -> info.precedence in
        let (_, after_op) = match parse_operator !rest with
          | Ok r -> r | Error _ -> continue_loop := false; ("", !rest) in (* dummy *)
        if !continue_loop then begin
          let (rhs, rem) = climb_expr after_op next_min in
          lhs := BinOp (info.symbol, !lhs, rhs);
          rest := rem end
      end
  done;
  (!lhs, !rest)

(* Tests *)
let () =
  (match parse_expr "1 + 2 * 3" with
   | Ok (BinOp ("+", Num 1., BinOp ("*", Num 2., Num 3.)), "") -> ()
   | _ -> failwith "Precedence test");

  (match parse_expr "2 ^ 3 ^ 2" with
   | Ok (BinOp ("^", Num 2., BinOp ("^", Num 3., Num 2.)), "") -> ()
   | _ -> failwith "Right assoc test");

  (match parse_expr "1 + 2 + 3" with
   | Ok (BinOp ("+", BinOp ("+", Num 1., Num 2.), Num 3.), "") -> ()
   | _ -> failwith "Left assoc test");

  print_endline "✓ All tests passed"
