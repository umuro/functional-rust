(* 003: Pattern Matching
   Tuples, variants, nested patterns, guards — OCaml's pattern matching is exhaustive *)

(* --- Approach 1: Tuple matching --- *)

let describe_pair = function
  | (0, 0) -> "origin"
  | (x, 0) -> Printf.sprintf "x-axis at %d" x
  | (0, y) -> Printf.sprintf "y-axis at %d" y
  | (x, y) -> Printf.sprintf "point (%d, %d)" x y

(* --- Approach 2: Variant (enum) matching --- *)

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
  | Circle _    -> "circle"
  | Rectangle _ -> "rectangle"
  | Triangle _  -> "triangle"

(* --- Approach 3: Recursive expression trees with nested patterns --- *)

type expr =
  | Num of int
  | Add of expr * expr
  | Mul of expr * expr

let rec eval = function
  | Num n       -> n
  | Add (a, b)  -> eval a + eval b
  | Mul (a, b)  -> eval a * eval b

(* Guard patterns *)
let classify_number n =
  match n with
  | n when n < 0    -> "negative"
  | 0               -> "zero"
  | n when n mod 2 = 0 -> "positive even"
  | _               -> "positive odd"

let () =
  Printf.printf "%s\n" (describe_pair (0, 0));
  Printf.printf "%s\n" (describe_pair (3, 0));
  Printf.printf "%s\n" (describe_pair (0, 5));
  Printf.printf "%s\n" (describe_pair (2, 3));
  Printf.printf "circle area r=1: %.4f\n" (area (Circle 1.0));
  Printf.printf "rectangle 3x4: %.1f\n" (area (Rectangle (3.0, 4.0)));
  let e = Add (Num 1, Mul (Num 2, Num 3)) in
  Printf.printf "eval (1 + 2*3) = %d\n" (eval e);
  List.iter (fun n -> Printf.printf "%d: %s\n" n (classify_number n)) [-5; 0; 4; 7]
