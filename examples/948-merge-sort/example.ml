let rec merge cmp l1 l2 = match l1, l2 with
  | [], l | l, [] -> l
  | h1 :: t1, h2 :: t2 ->
    if cmp h1 h2 <= 0 then h1 :: merge cmp t1 l2
    else h2 :: merge cmp l1 t2

let rec split = function
  | [] -> [], []
  | [x] -> [x], []
  | a :: b :: rest ->
    let l, r = split rest in
    a :: l, b :: r

let rec merge_sort cmp = function
  | ([] | [_]) as l -> l
  | l ->
    let left, right = split l in
    merge cmp (merge_sort cmp left) (merge_sort cmp right)

let () =
  let sorted = merge_sort compare [5; 2; 8; 1; 9; 3] in
  assert (sorted = [1; 2; 3; 5; 8; 9]);
  assert (merge_sort compare [] = []);
  assert (merge_sort compare [42] = [42]);
  assert (merge_sort compare [3; 1; 2; 1; 3] = [1; 1; 2; 3; 3]);
  print_endline "All assertions passed."
