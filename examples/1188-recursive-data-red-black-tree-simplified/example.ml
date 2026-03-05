(* Recursive Data — Red-Black Tree (simplified) *)
(* Balanced binary search tree with colors *)

type color = Red | Black
type 'a rbtree = E | T of color * 'a rbtree * 'a * 'a rbtree

let balance = function
  | (Black, T (Red, T (Red, a, x, b), y, c), z, d)
  | (Black, T (Red, a, x, T (Red, b, y, c)), z, d)
  | (Black, a, x, T (Red, T (Red, b, y, c), z, d))
  | (Black, a, x, T (Red, b, y, T (Red, c, z, d))) ->
    T (Red, T (Black, a, x, b), y, T (Black, c, z, d))
  | (c, l, v, r) -> T (c, l, v, r)

let insert x t =
  let rec ins = function
    | E -> T (Red, E, x, E)
    | T (c, l, v, r) ->
      if x < v then balance (c, ins l, v, r)
      else if x > v then balance (c, l, v, ins r)
      else T (c, l, v, r)
  in match ins t with T (_, l, v, r) -> T (Black, l, v, r) | E -> E

let tree = List.fold_left (fun t x -> insert x t) E [5;3;7;1;4;6;8;2]
let rec size = function E -> 0 | T(_,l,_,r) -> 1 + size l + size r
let () = Printf.printf "RB tree size: %d\n" (size tree)
