(* Prim's MST — O(V²) with linear scan (clean, readable) *)

let prim adj n =
  let key    = Array.make n max_int in
  let parent = Array.make n (-1) in
  let inMST  = Array.make n false in
  key.(0) <- 0;
  let total = ref 0 in
  let mst   = ref [] in
  for _ = 0 to n - 1 do
    (* Find unvisited vertex with minimum key *)
    let u = ref (-1) in
    for v = 0 to n - 1 do
      if not inMST.(v) && key.(v) < max_int then
        if !u = -1 || key.(v) < key.(!u) then u := v
    done;
    if !u >= 0 then begin
      inMST.(!u) <- true;
      if parent.(!u) >= 0 then begin
        total := !total + key.(!u);
        mst   := (!u, parent.(!u), key.(!u)) :: !mst
      end;
      List.iter (fun (w, v) ->
        if not inMST.(v) && w < key.(v) then begin
          key.(v)    <- w;
          parent.(v) <- !u
        end
      ) adj.(!u)
    end
  done;
  (!total, List.rev !mst)

let () =
  (* Undirected graph as adjacency list: adj.(u) = [(weight, v); ...] *)
  let adj = Array.make 5 [] in
  let add_edge u v w =
    adj.(u) <- (w, v) :: adj.(u);
    adj.(v) <- (w, u) :: adj.(v)
  in
  add_edge 0 1 2; add_edge 0 3 6;
  add_edge 1 2 3; add_edge 1 3 8; add_edge 1 4 5;
  add_edge 2 4 7;
  add_edge 3 4 9;
  let (total, mst) = prim adj 5 in
  Printf.printf "MST total weight: %d\n" total;
  List.iter (fun (u, v, w) ->
    Printf.printf "  edge %d-%d  weight=%d\n" v u w
  ) mst
