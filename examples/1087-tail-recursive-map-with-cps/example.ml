(* Naive map — not tail-recursive, stack overflow on large lists *)
let rec map_naive f = function
  | [] -> []
  | h :: t -> f h :: map_naive f t

(* Tail-recursive with accumulator + reverse *)
let map_tr f lst =
  let rec go acc = function
    | [] -> List.rev acc
    | h :: t -> go (f h :: acc) t
  in go [] lst

(* CPS — tail-recursive, preserves order without reverse *)
let map_cps f lst =
  let rec go k = function
    | [] -> k []
    | h :: t -> go (fun rest -> k (f h :: rest)) t
  in go Fun.id lst

let () =
  (* Empty list *)
  assert (map_naive (fun x -> x * 2) [] = []);
  assert (map_tr (fun x -> x * 2) [] = []);
  assert (map_cps (fun x -> x * 2) [] = []);

  (* Single element *)
  assert (map_naive (fun x -> x * 3) [5] = [15]);
  assert (map_tr (fun x -> x * 3) [5] = [15]);
  assert (map_cps (fun x -> x * 3) [5] = [15]);

  (* Multiple elements *)
  assert (map_naive (fun x -> x * 2) [1;2;3;4;5] = [2;4;6;8;10]);
  assert (map_tr (fun x -> x * 2) [1;2;3;4;5] = [2;4;6;8;10]);
  assert (map_cps (fun x -> x * 2) [1;2;3;4;5] = [2;4;6;8;10]);

  (* Type transformation *)
  assert (map_tr string_of_int [1;2;3] = ["1";"2";"3"]);
  assert (map_cps string_of_int [1;2;3] = ["1";"2";"3"]);

  print_endline "ok"
