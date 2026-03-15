(* 083: Display Trait / to_string
   OCaml uses to_string functions or Format module for custom printing *)

(* --- Approach 1: Simple to_string for variants --- *)

type color = Red | Green | Blue

let color_to_string = function
  | Red   -> "Red"
  | Green -> "Green"
  | Blue  -> "Blue"

type point = { x: float; y: float }

let point_to_string { x; y } =
  Printf.sprintf "(%.1f, %.1f)" x y

(* --- Approach 2: Record with a display function --- *)

type person = { name: string; age: int; email: string }

let person_to_string { name; age; email } =
  Printf.sprintf "%s (age %d, %s)" name age email

(* --- Approach 3: Recursive pretty-printer for a tree --- *)

type 'a tree = Leaf | Node of 'a tree * 'a * 'a tree

let rec tree_to_string show = function
  | Leaf          -> "."
  | Node (l, v, r) ->
    Printf.sprintf "(%s %s %s)"
      (tree_to_string show l)
      (show v)
      (tree_to_string show r)

(* Using Format module for structured output *)
let pp_list pp_elem ppf xs =
  let open Format in
  fprintf ppf "[";
  List.iteri (fun i x ->
    if i > 0 then fprintf ppf "; ";
    pp_elem ppf x) xs;
  fprintf ppf "]"

let () =
  Printf.printf "%s\n" (color_to_string Red);
  Printf.printf "%s\n" (color_to_string Green);
  Printf.printf "%s\n" (point_to_string { x = 3.0; y = 4.0 });
  let p = { name = "Alice"; age = 30; email = "alice@ex.com" } in
  Printf.printf "%s\n" (person_to_string p);

  let tree =
    Node (Node (Leaf, 1, Leaf), 2, Node (Leaf, 3, Leaf))
  in
  Printf.printf "%s\n" (tree_to_string string_of_int tree);

  (* Format-based list printer *)
  let buf = Buffer.create 32 in
  let ppf = Format.formatter_of_buffer buf in
  pp_list (fun ppf n -> Format.fprintf ppf "%d" n) ppf [1;2;3];
  Format.pp_print_flush ppf ();
  Printf.printf "%s\n" (Buffer.contents buf)
