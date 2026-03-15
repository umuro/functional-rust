(* 972: Persistent Binary Search Tree *)
(* Functional update: insert/delete return new root, old tree unchanged *)

type 'a bst =
  | Empty
  | Node of 'a bst * 'a * 'a bst

(* Approach 1: Insert returns new tree (functional style) *)

let rec insert tree x =
  match tree with
  | Empty -> Node (Empty, x, Empty)
  | Node (l, v, r) ->
    if x < v then Node (insert l x, v, r)
    else if x > v then Node (l, v, insert r x)
    else tree  (* duplicate: return same tree *)

let rec member tree x =
  match tree with
  | Empty -> false
  | Node (l, v, r) ->
    if x = v then true
    else if x < v then member l x
    else member r x

let rec min_val = function
  | Empty -> None
  | Node (Empty, v, _) -> Some v
  | Node (l, _, _) -> min_val l

(* Approach 2: Functional delete *)

let rec delete tree x =
  match tree with
  | Empty -> Empty
  | Node (l, v, r) ->
    if x < v then Node (delete l x, v, r)
    else if x > v then Node (l, v, delete r x)
    else
      (* Found: merge left and right subtrees *)
      match min_val r with
      | None -> l  (* no right subtree *)
      | Some m -> Node (l, m, delete r m)

let rec to_list tree =
  match tree with
  | Empty -> []
  | Node (l, v, r) -> to_list l @ [v] @ to_list r

let () =
  let t0 = Empty in
  let t1 = insert t0 5 in
  let t2 = insert t1 3 in
  let t3 = insert t2 7 in
  let t4 = insert t3 1 in
  let t5 = insert t4 4 in

  (* t4 still exists unchanged *)
  assert (to_list t4 = [1; 3; 5; 7]);
  assert (to_list t5 = [1; 3; 4; 5; 7]);

  assert (member t5 4);
  assert (member t5 5);
  assert (not (member t5 2));
  assert (not (member t5 6));

  let t6 = delete t5 3 in
  assert (to_list t6 = [1; 4; 5; 7]);
  assert (to_list t5 = [1; 3; 4; 5; 7]);  (* t5 unchanged! *)

  let t7 = delete t5 5 in
  assert (to_list t7 = [1; 3; 4; 7]);

  Printf.printf "✓ All tests passed\n"
