(* Red-Black Tree with Okasaki's Functional Balancing *)

type color = Red | Black
type 'a rbtree = E | T of color * 'a rbtree * 'a * 'a rbtree

(* Okasaki's balance: all four red-red violation cases → one balanced result *)
let balance = function
  | Black, T (Red, T (Red, a, x, b), y, c), z, d
  | Black, T (Red, a, x, T (Red, b, y, c)), z, d
  | Black, a, x, T (Red, T (Red, b, y, c), z, d)
  | Black, a, x, T (Red, b, y, T (Red, c, z, d)) ->
    T (Red, T (Black, a, x, b), y, T (Black, c, z, d))
  | color, a, x, b -> T (color, a, x, b)

(* Insert with rebalancing; root always repainted black *)
let insert x t =
  let rec ins = function
    | E -> T (Red, E, x, E)
    | T (color, a, y, b) ->
      if x < y then balance (color, ins a, y, b)
      else if x > y then balance (color, a, y, ins b)
      else T (color, a, y, b)
  in
  match ins t with T (_, a, y, b) -> T (Black, a, y, b) | E -> E

(* Membership check *)
let rec mem x = function
  | E -> false
  | T (_, a, y, b) -> x = y || (if x < y then mem x a else mem x b)

(* In-order traversal to list *)
let rec to_list = function
  | E -> [] | T (_, a, v, b) -> to_list a @ [v] @ to_list b

let () =
  let t = List.fold_left (fun t x -> insert x t) E [5;3;7;1;4;6;8;2;9] in
  assert (to_list t = [1;2;3;4;5;6;7;8;9]);
  assert (mem 5 t = true);
  assert (mem 0 t = false);
  (* Duplicates are ignored *)
  let t2 = insert 5 (insert 3 t) in
  assert (to_list t2 = [1;2;3;4;5;6;7;8;9]);
  List.iter (Printf.printf "%d ") (to_list t);
  print_endline "ok"
