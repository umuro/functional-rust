(* Naive map — not tail-recursive, stack overflow on large lists *)
let rec map_naive f = function
  | [] -> []
  | h :: t -> f h :: map_naive f t

(* Tail-recursive with reverse — accumulates in reverse, then flips *)
let map_tr f lst =
  let rec go acc = function
    | [] -> List.rev acc
    | h :: t -> go (f h :: acc) t
  in go [] lst

(* CPS — tail-recursive, preserves order without reversing *)
let map_cps f lst =
  let rec go k = function
    | [] -> k []
    | h :: t -> go (fun rest -> k (f h :: rest)) t
  in go Fun.id lst

(* Tests *)
let () =
  (* Basic functionality *)
  assert (map_naive (fun x -> x * 2) [1;2;3;4] = [2;4;6;8]);
  assert (map_tr (fun x -> x * 2) [1;2;3;4] = [2;4;6;8]);
  assert (map_cps (fun x -> x * 2) [1;2;3;4] = [2;4;6;8]);

  (* Empty list *)
  assert (map_naive (fun x -> x + 1) [] = []);
  assert (map_tr (fun x -> x + 1) [] = []);
  assert (map_cps (fun x -> x + 1) [] = []);

  (* Single element *)
  assert (map_naive (fun x -> x + 1) [5] = [6]);
  assert (map_tr (fun x -> x + 1) [5] = [6]);
  assert (map_cps (fun x -> x + 1) [5] = [6]);

  (* Large input — only tail-recursive versions survive *)
  let big = List.init 1_000_000 Fun.id in
  let result = map_tr (fun x -> x * 2) big in
  assert (List.length result = 1_000_000);
  assert (List.hd result = 0);
  assert (List.nth result 999_999 = 1_999_998);

  let result_cps = map_cps (fun x -> x * 2) big in
  assert (List.length result_cps = 1_000_000);
  assert (List.hd result_cps = 0);

  print_endline "ok"
