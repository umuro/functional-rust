(* Example 067: Foldable Trait *)
(* Custom fold over tree/list structures *)

module type FOLDABLE = sig
  type 'a t
  val fold_left : ('b -> 'a -> 'b) -> 'b -> 'a t -> 'b
  val fold_right : ('a -> 'b -> 'b) -> 'a t -> 'b -> 'b
end

(* Approach 1: Foldable for custom list *)
type 'a mylist = Nil | Cons of 'a * 'a mylist

module MyListFoldable : FOLDABLE with type 'a t = 'a mylist = struct
  type 'a t = 'a mylist
  let rec fold_left f acc = function
    | Nil -> acc
    | Cons (x, xs) -> fold_left f (f acc x) xs
  let rec fold_right f lst acc = match lst with
    | Nil -> acc
    | Cons (x, xs) -> f x (fold_right f xs acc)
end

(* Approach 2: Foldable for binary tree *)
type 'a tree = Leaf | Node of 'a tree * 'a * 'a tree

module TreeFoldable : FOLDABLE with type 'a t = 'a tree = struct
  type 'a t = 'a tree
  let rec fold_left f acc = function
    | Leaf -> acc
    | Node (l, v, r) ->
      let acc = fold_left f acc l in
      let acc = f acc v in
      fold_left f acc r
  let rec fold_right f tree acc = match tree with
    | Leaf -> acc
    | Node (l, v, r) ->
      fold_right f l (f v (fold_right f r acc))
end

(* Approach 3: Derived operations from fold *)
let to_list (type a) (module F : FOLDABLE with type 'x t = a) xs =
  F.fold_right (fun x acc -> x :: acc) xs []

let sum (type a) (module F : FOLDABLE with type 'x t = a) xs =
  F.fold_left (fun acc x -> acc + x) 0 xs

let length (type a) (module F : FOLDABLE with type 'x t = a) xs =
  F.fold_left (fun acc _ -> acc + 1) 0 xs

let () =
  let lst = Cons (1, Cons (2, Cons (3, Nil))) in
  assert (MyListFoldable.fold_left (+) 0 lst = 6);
  assert (MyListFoldable.fold_right (fun x acc -> x :: acc) lst [] = [1; 2; 3]);

  let tree = Node (Node (Leaf, 1, Leaf), 2, Node (Leaf, 3, Leaf)) in
  assert (TreeFoldable.fold_left (+) 0 tree = 6);
  assert (TreeFoldable.fold_right (fun x acc -> x :: acc) tree [] = [1; 2; 3]);

  Printf.printf "✓ All tests passed\n"
