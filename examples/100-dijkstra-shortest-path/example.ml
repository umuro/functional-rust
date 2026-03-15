(* Dijkstra's Shortest Path — Functional approach with immutable priority queue *)

module IntMap = Map.Make(Int)

(* Priority queue as a sorted association list (simple, functional) *)
module PQ = struct
  type t = (int * int) list  (* (distance, node) pairs *)

  let empty = []

  let insert dist node pq =
    let rec go = function
      | [] -> [(dist, node)]
      | ((d, _) as x) :: rest ->
        if dist <= d then (dist, node) :: x :: rest
        else x :: go rest
    in go pq

  let pop = function
    | [] -> None
    | (d, n) :: rest -> Some ((d, n), rest)
end

(* Graph as adjacency list: node -> (neighbor, weight) list *)
type graph = (int * int) list IntMap.t

let add_edge g u v w =
  let neighbors = try IntMap.find u g with Not_found -> [] in
  IntMap.add u ((v, w) :: neighbors) g

let dijkstra (graph : graph) source =
  let dist = IntMap.singleton source 0 in
  let pq = PQ.insert 0 source PQ.empty in
  let rec loop pq dist =
    match PQ.pop pq with
    | None -> dist
    | Some ((d, u), pq') ->
      let current_dist = try IntMap.find u dist with Not_found -> max_int in
      if d > current_dist then loop pq' dist  (* skip stale entries *)
      else
        let neighbors = try IntMap.find u graph with Not_found -> [] in
        let pq'', dist' =
          List.fold_left (fun (pq_acc, dist_acc) (v, w) ->
            let new_dist = d + w in
            let old_dist = try IntMap.find v dist_acc with Not_found -> max_int in
            if new_dist < old_dist then
              (PQ.insert new_dist v pq_acc, IntMap.add v new_dist dist_acc)
            else
              (pq_acc, dist_acc)
          ) (pq', dist) neighbors
        in
        loop pq'' dist'
  in
  loop pq dist

let () =
  (* Build graph:  0 --1-- 1 --2-- 2
                   |              |
                   4              1
                   |              |
                   3 ------3----- 4  *)
  let g = IntMap.empty in
  let g = add_edge g 0 1 1 in
  let g = add_edge g 1 2 2 in
  let g = add_edge g 0 3 4 in
  let g = add_edge g 2 4 1 in
  let g = add_edge g 3 4 3 in
  let result = dijkstra g 0 in
  Printf.printf "Shortest distances from node 0:\n";
  IntMap.iter (fun node dist ->
    Printf.printf "  node %d: %d\n" node dist
  ) result

(* Output:
   Shortest distances from node 0:
     node 0: 0
     node 1: 1
     node 2: 3
     node 3: 4
     node 4: 4
*)
