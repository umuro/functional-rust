type color = Red | Black
type 'a rbtree = E | T of color * 'a rbtree * 'a * 'a rbtree

(* Okasaki's 4-case balance — all cases produce T(Red, T(Black,a,x,b), y, T(Black,c,z,d)) *)
let balance = function
  | Black, T (Red, T (Red, a, x, b), y, c), z, d
  | Black, T (Red, a, x, T (Red, b, y, c)), z, d
  | Black, a, x, T (Red, T (Red, b, y, c), z, d)
  | Black, a, x, T (Red, b, y, T (Red, c, z, d)) ->
    T (Red, T (Black, a, x, b), y, T (Black, c, z, d))
  | color, a, x, b -> T (color, a, x, b)

(* Recursive insert: new leaf is Red, balance repairs any double-Red violation *)
let insert x t =
  let rec ins = function
    | E -> T (Red, E, x, E)
    | T (color, a, y, b) ->
      if x < y then balance (color, ins a, y, b)
      else if x > y then balance (color, a, y, ins b)
      else T (color, a, y, b)
  in
  (* Always blacken the root *)
  match ins t with T (_, a, y, b) -> T (Black, a, y, b) | E -> E

let rec mem x = function
  | E -> false
  | T (_, a, y, b) -> x = y || (if x < y then mem x a else mem x b)

let rec to_list = function
  | E -> []
  | T (_, a, v, b) -> to_list a @ [v] @ to_list b

let () =
  let t = List.fold_left (fun t x -> insert x t) E [5; 3; 7; 1; 4; 6; 8; 2; 9] in
  assert (to_list t = [1; 2; 3; 4; 5; 6; 7; 8; 9]);
  assert (mem 5 t);
  assert (mem 1 t);
  assert (mem 9 t);
  assert (not (mem 0 t));
  assert (not (mem 10 t));

  (* Root must always be Black *)
  (match t with T (Black, _, _, _) -> () | _ -> assert false);

  (* Duplicates are ignored: set semantics *)
  let t2 = List.fold_left (fun t x -> insert x t) E [3; 3; 1; 2; 1] in
  assert (to_list t2 = [1; 2; 3]);

  (* Sorted insertion exercises all balance cases *)
  let t3 = List.fold_left (fun t x -> insert x t) E [1; 2; 3; 4; 5] in
  assert (to_list t3 = [1; 2; 3; 4; 5]);

  print_endline "ok"
