type 'a avl = Empty | Node of 'a avl * 'a * 'a avl * int

let height = function Empty -> 0 | Node (_, _, _, h) -> h
let node l v r = Node (l, v, r, 1 + max (height l) (height r))
let balance t = match t with Empty -> 0 | Node (l,_,r,_) -> height l - height r

let rotate_right = function
  | Node (Node (ll, lv, lr, _), v, r, _) -> node (node ll lv lr) v r
  | t -> t

let rotate_left = function
  | Node (l, v, Node (rl, rv, rr, _), _) -> node l v (node rl rv rr)
  | t -> t

let rebalance t = match balance t with
  | b when b > 1 -> rotate_right t
  | b when b < -1 -> rotate_left t
  | _ -> t

let rec insert x = function
  | Empty -> node Empty x Empty
  | Node (l, v, r, _) ->
    if x < v then rebalance (node (insert x l) v r)
    else if x > v then rebalance (node l v (insert x r))
    else node l v r

let rec inorder = function
  | Empty -> [] | Node (l,v,r,_) -> inorder l @ [v] @ inorder r

let () =
  let t = List.fold_left (fun t x -> insert x t) Empty [7;3;9;1;5;8;10;2] in
  assert (inorder t = [1;2;3;5;7;8;9;10]);
  (* Balance check: height should be small *)
  assert (height t <= 4);
  print_endline "ok"
