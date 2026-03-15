(* Example 051: Functors Introduction *)
(* A Functor is a type that supports mapping a function over its contents *)

(* Approach 1: Custom Maybe type with map *)
module Maybe = struct
  type 'a t = Nothing | Just of 'a

  let map f = function
    | Nothing -> Nothing
    | Just x -> Just (f x)

  let pure x = Just x

  let to_string f = function
    | Nothing -> "Nothing"
    | Just x -> Printf.sprintf "Just(%s)" (f x)
end

(* Approach 2: Functor signature and implementation for a Box type *)
module type FUNCTOR = sig
  type 'a t
  val map : ('a -> 'b) -> 'a t -> 'b t
end

module Box : FUNCTOR = struct
  type 'a t = Box of 'a
  let map f (Box x) = Box (f x)
end

(* Approach 3: Functor for a tree type *)
type 'a tree = Leaf | Node of 'a tree * 'a * 'a tree

let rec tree_map f = function
  | Leaf -> Leaf
  | Node (l, v, r) -> Node (tree_map f l, f v, tree_map f r)

(* Tests *)
let () =
  (* Test Maybe functor *)
  let x = Maybe.Just 5 in
  let y = Maybe.map (fun n -> n * 2) x in
  assert (y = Maybe.Just 10);

  let n = Maybe.Nothing in
  let m = Maybe.map (fun n -> n * 2) n in
  assert (m = Maybe.Nothing);

  (* Test chained maps *)
  let result = Maybe.map (fun s -> String.length s) (Maybe.Just "hello") in
  assert (result = Maybe.Just 5);

  (* Test tree functor *)
  let t = Node (Node (Leaf, 1, Leaf), 2, Node (Leaf, 3, Leaf)) in
  let t2 = tree_map (fun x -> x * 10) t in
  assert (t2 = Node (Node (Leaf, 10, Leaf), 20, Node (Leaf, 30, Leaf)));

  Printf.printf "✓ All tests passed\n"
