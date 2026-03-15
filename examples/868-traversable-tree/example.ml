(* Example 069: Traversable for Binary Tree *)
(* Map over a tree with effects (Option/Result) *)

type 'a tree = Leaf | Node of 'a tree * 'a * 'a tree

(* Approach 1: Traverse with Option *)
let rec traverse_option f = function
  | Leaf -> Some Leaf
  | Node (l, v, r) ->
    match traverse_option f l with
    | None -> None
    | Some l' ->
      match f v with
      | None -> None
      | Some v' ->
        match traverse_option f r with
        | None -> None
        | Some r' -> Some (Node (l', v', r'))

(* Approach 2: Traverse with Result *)
let rec traverse_result f = function
  | Leaf -> Ok Leaf
  | Node (l, v, r) ->
    match traverse_result f l with
    | Error e -> Error e
    | Ok l' ->
      match f v with
      | Error e -> Error e
      | Ok v' ->
        match traverse_result f r with
        | Error e -> Error e
        | Ok r' -> Ok (Node (l', v', r'))

(* Approach 3: Map (traverse with identity effect) *)
let rec map f = function
  | Leaf -> Leaf
  | Node (l, v, r) -> Node (map f l, f v, map f r)

(* Test helpers *)
let safe_double x = if x > 50 then None else Some (x * 2)
let parse_positive x = if x > 0 then Ok x else Error (Printf.sprintf "Not positive: %d" x)

let rec to_list = function
  | Leaf -> []
  | Node (l, v, r) -> to_list l @ [v] @ to_list r

let () =
  let tree = Node (Node (Leaf, 1, Leaf), 2, Node (Leaf, 3, Leaf)) in

  (* Traverse Option — all succeed *)
  let result = traverse_option safe_double tree in
  assert (result = Some (Node (Node (Leaf, 2, Leaf), 4, Node (Leaf, 6, Leaf))));

  (* Traverse Option — one fails *)
  let big_tree = Node (Node (Leaf, 10, Leaf), 60, Leaf) in
  assert (traverse_option safe_double big_tree = None);

  (* Traverse Result *)
  assert (traverse_result parse_positive tree = Ok tree);
  let neg_tree = Node (Leaf, (-1), Leaf) in
  assert (traverse_result parse_positive neg_tree = Error "Not positive: -1");

  (* Map *)
  let doubled = map (fun x -> x * 2) tree in
  assert (to_list doubled = [2; 4; 6]);

  Printf.printf "✓ All tests passed\n"
