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
      let take = if w <= cap then v + solve (i+1) (cap - w) else 0 in
      let skip = solve (i+1) cap in
      let best = max take skip in
      Hashtbl.add cache (i, cap) best;
      best
  in solve 0 capacity

let () =
  let items = [(2,3); (3,4); (4,5); (5,6)] in
  Printf.printf "knapsack 8 = %d\n" (knapsack items 8)