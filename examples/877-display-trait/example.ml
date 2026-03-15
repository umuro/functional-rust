(* Example 083: Display Trait *)
(* OCaml to_string → Rust fmt::Display *)

(* Approach 1: Custom to_string functions *)
type color = Red | Green | Blue

let color_to_string = function
  | Red -> "Red"
  | Green -> "Green"
  | Blue -> "Blue"

type point = { x : float; y : float }

let point_to_string p =
  Printf.sprintf "(%.2f, %.2f)" p.x p.y

(* Approach 2: Using Format module (like Rust's fmt) *)
type matrix = float array array

let matrix_to_string m =
  let rows = Array.map (fun row ->
    let cells = Array.map (fun v -> Printf.sprintf "%6.2f" v) row in
    "| " ^ String.concat " " (Array.to_list cells) ^ " |"
  ) m in
  String.concat "\n" (Array.to_list rows)

(* Approach 3: Recursive/nested display *)
type 'a tree = Leaf | Node of 'a tree * 'a * 'a tree

let rec tree_to_string to_s = function
  | Leaf -> "."
  | Node (l, v, r) ->
    Printf.sprintf "(%s %s %s)"
      (tree_to_string to_s l) (to_s v) (tree_to_string to_s r)

(* Printf-compatible custom printer *)
let pp_point fmt p =
  Format.fprintf fmt "(%.2f, %.2f)" p.x p.y

let pp_color fmt c =
  Format.fprintf fmt "%s" (color_to_string c)

(* Tests *)
let () =
  assert (color_to_string Red = "Red");
  assert (color_to_string Blue = "Blue");

  let p = { x = 3.14; y = 2.72 } in
  assert (point_to_string p = "(3.14, 2.72)");

  let m = [| [| 1.0; 2.0 |]; [| 3.0; 4.0 |] |] in
  let s = matrix_to_string m in
  assert (String.length s > 0);

  let tree = Node (Node (Leaf, 1, Leaf), 2, Node (Leaf, 3, Leaf)) in
  let s = tree_to_string string_of_int tree in
  assert (s = "((. 1 .) 2 (. 3 .))");

  Printf.printf "✓ All tests passed\n"
