type color = Red | Black
type 'a rbtree = E | T of color * 'a rbtree * 'a * 'a rbtree

let balance = function
  | Black, T (Red, T (Red, a, x, b), y, c), z, d
  | Black, T (Red, a, x, T (Red, b, y, c)), z, d
  | Black, a, x, T (Red, T (Red, b, y, c), z, d)
  | Black, a, x, T (Red, b, y, T (Red, c, z, d)) ->
    T (Red, T (Black, a, x, b), y, T (Black, c, z, d))
  | color, a, x, b -> T (color, a, x, b)

let insert x t =
  let rec ins = function
    | E -> T (Red, E, x, E)
    | T (color, a, y, b) ->
      if x < y then balance (color, ins a, y, b)
      else if x > y then balance (color, a, y, ins b)
      else T (color, a, y, b)
  in
  match ins t with T (_, a, y, b) -> T (Black, a, y, b) | E -> E

let rec mem x = function
  | E -> false
  | T (_, a, y, b) -> x = y || (if x < y then mem x a else mem x b)

let rec to_list = function
  | E -> [] | T (_, a, v, b) -> to_list a @ [v] @ to_list b

let () =
  let t = List.fold_left (fun t x -> insert x t) E [5;3;7;1;4;6;8;2;9] in
  assert (to_list t = [1;2;3;4;5;6;7;8;9]);
  assert (mem 5 t);
  assert (not (mem 10 t));
  (* Duplicates are ignored *)
  let t2 = insert 5 t in
  assert (to_list t2 = [1;2;3;4;5;6;7;8;9]);
  (* Root is always black *)
  (match t with T (Black, _, _, _) -> () | _ -> assert false);
  print_endline "ok"