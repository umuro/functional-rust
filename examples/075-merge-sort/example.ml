(* 075: Merge Sort *)

(* Approach 1: Classic recursive merge sort *)
let rec merge l1 l2 =
  match l1, l2 with
  | [], l | l, [] -> l
  | x :: xs, y :: ys ->
    if x <= y then x :: merge xs (y :: ys)
    else y :: merge (x :: xs) ys

let rec split = function
  | [] -> ([], [])
  | [x] -> ([x], [])
  | x :: y :: rest ->
    let (l, r) = split rest in
    (x :: l, y :: r)

let rec merge_sort = function
  | [] -> []
  | [x] -> [x]
  | lst ->
    let (l, r) = split lst in
    merge (merge_sort l) (merge_sort r)

(* Approach 2: Generic merge sort with comparator *)
let rec merge_by cmp l1 l2 =
  match l1, l2 with
  | [], l | l, [] -> l
  | x :: xs, y :: ys ->
    if cmp x y <= 0 then x :: merge_by cmp xs (y :: ys)
    else y :: merge_by cmp (x :: xs) ys

let rec merge_sort_by cmp = function
  | [] -> []
  | [x] -> [x]
  | lst ->
    let (l, r) = split lst in
    merge_by cmp (merge_sort_by cmp l) (merge_sort_by cmp r)

(* Approach 3: Bottom-up style using fold *)
let merge_sort_fold lst =
  let singletons = List.map (fun x -> [x]) lst in
  let rec reduce = function
    | [] -> []
    | [x] -> x
    | pairs ->
      let rec pair_merge = function
        | [] -> []
        | [x] -> [x]
        | x :: y :: rest -> merge x y :: pair_merge rest
      in
      reduce (pair_merge pairs)
  in
  reduce singletons

(* Tests *)
let () =
  assert (merge_sort [5; 3; 8; 1; 9; 2; 7] = [1; 2; 3; 5; 7; 8; 9]);
  assert (merge_sort [] = []);
  assert (merge_sort [1] = [1]);
  assert (merge_sort [2; 1] = [1; 2]);
  assert (merge_sort_by compare [5; 3; 8; 1] = [1; 3; 5; 8]);
  assert (merge_sort_by (fun a b -> compare b a) [5; 3; 8; 1] = [8; 5; 3; 1]);
  assert (merge_sort_fold [5; 3; 8; 1; 9; 2; 7] = [1; 2; 3; 5; 7; 8; 9]);
  Printf.printf "✓ All tests passed\n"
