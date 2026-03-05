(* Dijkstra in OCaml - simplified *)
module PQ = Set.Make(struct type t = int * int let compare = compare end)

let dijkstra graph start n =
  let dist = Array.make n max_int in
  dist.(start) <- 0;
  let rec loop pq =
    if PQ.is_empty pq then () else
    let (d, u) = PQ.min_elt pq in
    let pq = PQ.remove (d, u) pq in
    if d > dist.(u) then loop pq else
    let pq = List.fold_left (fun pq (v, w) ->
      let nd = d + w in
      if nd < dist.(v) then (dist.(v) <- nd; PQ.add (nd, v) pq)
      else pq
    ) pq (try List.assoc u graph with Not_found -> []) in
    loop pq
  in loop (PQ.singleton (0, start)); dist
