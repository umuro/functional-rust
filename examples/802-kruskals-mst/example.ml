(* Kruskal's MST — sort + Union-Find O(E log E) *)

(* Union-Find with path compression and union by rank *)
let make_uf n =
  let parent = Array.init n (fun i -> i) in
  let rank   = Array.make n 0 in
  (parent, rank)

let rec find parent v =
  if parent.(v) = v then v
  else begin
    parent.(v) <- find parent parent.(v);  (* path compression *)
    parent.(v)
  end

let union parent rank u v =
  let pu = find parent u and pv = find parent v in
  if pu = pv then false
  else begin
    if rank.(pu) < rank.(pv) then parent.(pu) <- pv
    else if rank.(pu) > rank.(pv) then parent.(pv) <- pu
    else begin parent.(pv) <- pu; rank.(pu) <- rank.(pu) + 1 end;
    true
  end

let kruskal n edges =
  (* edges: (weight, u, v) list *)
  let sorted = List.sort (fun (w1,_,_) (w2,_,_) -> compare w1 w2) edges in
  let (parent, rank) = make_uf n in
  List.fold_left (fun (total, mst) (w, u, v) ->
    if union parent rank u v then
      (total + w, (u, v, w) :: mst)
    else
      (total, mst)
  ) (0, []) sorted

let () =
  let edges = [
    (2, 0, 1); (6, 0, 3);
    (3, 1, 2); (8, 1, 3); (5, 1, 4);
    (7, 2, 4); (9, 3, 4)
  ] in
  let (total, mst) = kruskal 5 edges in
  Printf.printf "MST total weight: %d\n" total;
  List.iter (fun (u, v, w) ->
    Printf.printf "  edge %d-%d  weight=%d\n" u v w
  ) (List.rev mst)
