(* DFS in OCaml *)
module IntSet = Set.Make(Int)
module IntMap = Map.Make(Int)

let rec dfs graph visited v =
  if IntSet.mem v visited then visited
  else begin
    Printf.printf "%d " v;
    let visited = IntSet.add v visited in
    let neighbors = try IntMap.find v graph with Not_found -> [] in
    List.fold_left (dfs graph) visited neighbors
  end

let () =
  let g = IntMap.of_seq (List.to_seq [(1, [2;3]); (2, [4]); (3, [4]); (4, [])]) in
  print_string "DFS: ";
  ignore (dfs g IntSet.empty 1);
  print_newline ()
