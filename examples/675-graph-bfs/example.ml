(* BFS in OCaml *)
module IntSet = Set.Make(Int)
module IntMap = Map.Make(Int)

let bfs graph start =
  let rec loop visited queue acc =
    match queue with
    | [] -> List.rev acc
    | v :: rest ->
        let neighbors = try IntMap.find v graph with Not_found -> [] in
        let new_ns = List.filter (fun n -> not (IntSet.mem n visited)) neighbors in
        let visited' = List.fold_left (fun s n -> IntSet.add n s) visited new_ns in
        loop visited' (rest @ new_ns) (v :: acc)
  in
  loop (IntSet.singleton start) [start] []
