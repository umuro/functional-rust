(* Example 117: Recursive Types with Box *)

(* Approach 1: Binary tree *)
type 'a tree = Leaf | Node of 'a tree * 'a * 'a tree

let rec insert x = function
  | Leaf -> Node (Leaf, x, Leaf)
  | Node (l, v, r) ->
    if x < v then Node (insert x l, v, r)
    else if x > v then Node (l, v, insert x r)
    else Node (l, v, r)

let rec to_sorted_list = function
  | Leaf -> []
  | Node (l, v, r) -> to_sorted_list l @ [v] @ to_sorted_list r

let approach1 () =
  let t = List.fold_left (fun acc x -> insert x acc) Leaf [5;3;7;1;4;6;8] in
  let sorted = to_sorted_list t in
  assert (sorted = [1;3;4;5;6;7;8]);
  Printf.printf "Sorted: %s\n" (String.concat ", " (List.map string_of_int sorted))

(* Approach 2: Linked list *)
type 'a mylist = Nil | Cons of 'a * 'a mylist

let rec length = function Nil -> 0 | Cons (_, rest) -> 1 + length rest
let rec map f = function Nil -> Nil | Cons (x, rest) -> Cons (f x, map f rest)

let approach2 () =
  let lst = Cons (1, Cons (2, Cons (3, Nil))) in
  assert (length lst = 3);
  let doubled = map (fun x -> x * 2) lst in
  assert (length doubled = 3);
  Printf.printf "Length: %d\n" (length lst)

(* Approach 3: Expression tree *)
type expr =
  | Lit of float
  | Var of string
  | BinOp of string * expr * expr

let rec show = function
  | Lit f -> string_of_float f
  | Var s -> s
  | BinOp (op, a, b) -> Printf.sprintf "(%s %s %s)" (show a) op (show b)

let approach3 () =
  let e = BinOp ("+", Lit 1.0, BinOp ("*", Var "x", Lit 3.0)) in
  let s = show e in
  Printf.printf "Expr: %s\n" s;
  assert (String.length s > 0)

let () =
  approach1 ();
  approach2 ();
  approach3 ();
  Printf.printf "✓ All tests passed\n"
