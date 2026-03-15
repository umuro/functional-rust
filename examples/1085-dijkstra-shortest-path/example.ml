(* Dijkstra's Shortest Path — OCaml functional style *)

module IntMap = Map.Make(Int)

type edge = { target: int; weight: int }

let build_graph edges =
  List.fold_left (fun g (src, dst, w) ->
    let existing = try IntMap.find src g with Not_found -> [] in
    IntMap.add src ({ target = dst; weight = w } :: existing) g
  ) IntMap.empty edges

let dijkstra graph source =
  let rec loop heap dist =
    match heap with
    | [] -> dist
    | (cost, node) :: rest ->
      let current = try IntMap.find node dist with Not_found -> max_int in
      if cost > current then loop rest dist
      else
        let neighbors = try IntMap.find node graph with Not_found -> [] in
        let dist', heap' = List.fold_left (fun (d, h) edge ->
          let new_dist = cost + edge.weight in
          let old = try IntMap.find edge.target d with Not_found -> max_int in
          if new_dist < old then
            (IntMap.add edge.target new_dist d, (new_dist, edge.target) :: h)
          else (d, h)
        ) (dist, rest) neighbors in
        loop (List.sort compare heap') dist'
  in
  loop [(0, source)] (IntMap.singleton source 0)

let () =
  let graph = build_graph [
    (0, 1, 1); (1, 2, 2); (0, 3, 4); (2, 4, 1); (3, 4, 3)
  ] in
  let dist = dijkstra graph 0 in
  IntMap.iter (fun node d ->
    Printf.printf "  node %d: %d\n" node d
  ) dist
