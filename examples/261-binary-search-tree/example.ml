type 'a bst = Leaf | Node of 'a bst * 'a * 'a bst

let rec insert x = function
  | Leaf -> Node (Leaf, x, Leaf)
  | Node (l, v, r) ->
    if x < v then Node (insert x l, v, r)
    else if x > v then Node (l, v, insert x r)
    else Node (l, v, r)

let rec mem x = function
  | Leaf -> false
  | Node (l, v, r) ->
    if x = v then true
    else if x < v then mem x l
    else mem x r

let rec inorder = function
  | Leaf -> []
  | Node (l, v, r) -> inorder l @ [v] @ inorder r

let () =
  let tree = List.fold_left (fun t x -> insert x t) Leaf [5;3;7;1;4;6;8] in
  assert (inorder tree = [1;3;4;5;6;7;8]);
  assert (mem 4 tree = true);
  assert (mem 9 tree = false);
  assert (inorder Leaf = []);
  print_endline "ok"
