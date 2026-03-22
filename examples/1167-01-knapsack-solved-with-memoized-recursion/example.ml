(* Idiomatic OCaml: 0/1 knapsack with memoized recursion *)
let knapsack items capacity =
  let n = List.length items in
  let items = Array.of_list items in
  let cache = Hashtbl.create 256 in
  let rec solve i cap =
    if i >= n || cap <= 0 then 0
    else match Hashtbl.find_opt cache (i, cap) with
    | Some v -> v
    | None ->
      let (w, v) = items.(i) in
      let without = solve (i+1) cap in
      let with_item = if w <= cap then v + solve (i+1) (cap - w) else 0 in
      let best = max without with_item in
      Hashtbl.add cache (i, cap) best;
      best
  in solve 0 capacity

let () =
  let items = [(2,3);(3,4);(4,5);(5,6)] in
  assert (knapsack items 8 = 10);
  assert (knapsack items 0 = 0);
  assert (knapsack [] 10 = 0);
  assert (knapsack [(3,10)] 5 = 10);
  assert (knapsack [(10,100)] 5 = 0);
  print_endline "ok"
