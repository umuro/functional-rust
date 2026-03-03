(* Topological Sort — DAG Ordering *)
module SS = Set.Make(String)

(* Version 1: DFS-based topological sort *)
let topo_sort edges =
  let neighbors node =
    List.filter_map (fun (a, b) -> if a = node then Some b else None) edges in
  let all_nodes = List.fold_left (fun s (a, b) -> SS.add a (SS.add b s)) SS.empty edges in
  let rec visit node (visited, order) =
    if SS.mem node visited then (visited, order)
    else
      let visited = SS.add node visited in
      let visited, order = List.fold_left (fun acc n ->
        visit n acc) (visited, order) (neighbors node) in
      (visited, node :: order)
  in
  let _, order = SS.fold (fun node acc -> visit node acc) all_nodes (SS.empty, []) in
  order

let () =
  let edges = [("a","b");("a","c");("b","d");("c","d");("d","e")] in
  let result = topo_sort edges in
  assert (List.mem "e" result);
  List.iter (Printf.printf "%s ") result
