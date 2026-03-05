(* Topological Sort in OCaml - DFS *)
module IntSet = Set.Make(Int)

let topo_sort graph n =
  let rec dfs v visited result =
    if IntSet.mem v visited then (visited, result)
    else
      let visited = IntSet.add v visited in
      let neighbors = try List.assoc v graph with Not_found -> [] in
      let (visited, result) = 
        List.fold_left (fun (vis, res) u -> dfs u vis res) (visited, result) neighbors
      in
      (visited, v :: result)
  in
  let (_, result) = 
    List.fold_left (fun (vis, res) i -> dfs i vis res) (IntSet.empty, []) (List.init n Fun.id)
  in result
