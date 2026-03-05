(* Kosaraju's SCC — two-pass DFS, O(V+E) *)

let kosaraju adj n =
  let visited = Array.make n false in
  let finish  = ref [] in  (* finish-time stack *)

  (* Pass 1: DFS on original graph, record finish order *)
  let rec dfs1 u =
    if not visited.(u) then begin
      visited.(u) <- true;
      List.iter dfs1 adj.(u);
      finish := u :: !finish
    end
  in
  for v = 0 to n - 1 do dfs1 v done;

  (* Build transposed graph *)
  let radj = Array.make n [] in
  for u = 0 to n - 1 do
    List.iter (fun v -> radj.(v) <- u :: radj.(v)) adj.(u)
  done;

  (* Pass 2: DFS on transposed graph in reverse finish order *)
  let visited2 = Array.make n false in
  let sccs     = ref [] in

  let rec dfs2 u scc =
    if not visited2.(u) then begin
      visited2.(u) <- true;
      let scc' = u :: scc in
      List.fold_left (fun acc v -> dfs2 v acc) scc' radj.(u)
    end else scc
  in
  List.iter (fun u ->
    if not visited2.(u) then
      sccs := List.sort compare (dfs2 u []) :: !sccs
  ) !finish;
  !sccs

let () =
  let n   = 8 in
  let adj = Array.make n [] in
  let add u v = adj.(u) <- v :: adj.(u) in
  add 0 1; add 1 2; add 2 0; add 2 3;
  add 3 4; add 4 5; add 5 3;
  add 6 5; add 6 7; add 7 6;
  let sccs = kosaraju adj n in
  Printf.printf "SCCs (%d total):\n" (List.length sccs);
  List.iteri (fun i scc ->
    Printf.printf "  SCC %d: [%s]\n" (i+1)
      (String.concat "," (List.map string_of_int scc))
  ) sccs
