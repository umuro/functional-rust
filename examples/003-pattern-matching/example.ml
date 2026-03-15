(* 003: Pattern Matching *)
(* Tuples, variants, nested patterns, guards *)

(* Approach 1: Simple tuple matching *)
let describe_pair = function
  | (0, 0) -> "origin"
  | (x, 0) -> Printf.sprintf "x-axis at %d" x
  | (0, y) -> Printf.sprintf "y-axis at %d" y
  | (x, y) -> Printf.sprintf "point (%d, %d)" x y

(* Approach 2: Variant types *)
type shape =
  | Circle of float
  | Rectangle of float * float
  | Triangle of float * float * float

let area = function
  | Circle r -> Float.pi *. r *. r
  | Rectangle (w, h) -> w *. h
  | Triangle (a, b, c) ->
    let s = (a +. b +. c) /. 2.0 in
    sqrt (s *. (s -. a) *. (s -. b) *. (s -. c))

let shape_name = function
  | Circle _ -> "circle"
  | Rectangle _ -> "rectangle"
  | Triangle _ -> "triangle"

(* Approach 3: Nested patterns with guards *)
type expr =
  | Num of int
  | Add of expr * expr
  | Mul of expr * expr

let rec eval = function
  | Num n -> n
  | Add (a, b) -> eval a + eval b
  | Mul (a, b) -> eval a * eval b

let classify_number n =
  match n with
  | n when n < 0 -> "negative"
  | 0 -> "zero"
  | n when n mod 2 = 0 -> "positive even"
  | _ -> "positive odd"

(* Tests *)
let () =
  assert (describe_pair (0, 0) = "origin");
  assert (describe_pair (3, 0) = "x-axis at 3");
  assert (describe_pair (0, 5) = "y-axis at 5");
  let c = Circle 1.0 in
  assert (abs_float (area c -. Float.pi) < 0.001);
  assert (area (Rectangle (3.0, 4.0)) = 12.0);
  assert (shape_name c = "circle");
  let e = Add (Num 1, Mul (Num 2, Num 3)) in
  assert (eval e = 7);
  assert (classify_number (-5) = "negative");
  assert (classify_number 0 = "zero");
  assert (classify_number 4 = "positive even");
  assert (classify_number 7 = "positive odd");
  Printf.printf "✓ All tests passed\n"
