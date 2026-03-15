(* 083: Display — custom string representation *)

(* Approach 1: Simple to_string *)
type color = Red | Green | Blue

let color_to_string = function
  | Red -> "Red"
  | Green -> "Green"
  | Blue -> "Blue"

type point = { x: float; y: float }

let point_to_string p =
  Printf.sprintf "(%.1f, %.1f)" p.x p.y

(* Approach 2: Complex formatting *)
type person = { name: string; age: int; email: string }

let person_to_string p =
  Printf.sprintf "%s (age %d, %s)" p.name p.age p.email

let person_to_debug p =
  Printf.sprintf "{ name = %S; age = %d; email = %S }" p.name p.age p.email

(* Approach 3: Recursive display *)
type 'a tree = Leaf | Node of 'a tree * 'a * 'a tree

let rec tree_to_string to_s = function
  | Leaf -> "."
  | Node (l, v, r) ->
    Printf.sprintf "(%s %s %s)"
      (tree_to_string to_s l)
      (to_s v)
      (tree_to_string to_s r)

(* Tests *)
let () =
  assert (color_to_string Red = "Red");
  assert (point_to_string { x = 3.0; y = 4.0 } = "(3.0, 4.0)");
  let p = { name = "Alice"; age = 30; email = "alice@ex.com" } in
  assert (person_to_string p = "Alice (age 30, alice@ex.com)");
  let t = Node (Node (Leaf, 1, Leaf), 2, Node (Leaf, 3, Leaf)) in
  assert (tree_to_string string_of_int t = "((. 1 .) 2 (. 3 .))");
  Printf.printf "✓ All tests passed\n"
