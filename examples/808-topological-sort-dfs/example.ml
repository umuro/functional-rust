(* Topological Sort — DFS with cycle detection O(V+E) *)

let topo_sort adj n =
  let state  = Array.make n 0 in  (* 0=white,1=grey,2=black *)
  let result = ref [] in
  let has_cycle = ref false in

  let rec dfs u =
    state.(u) <- 1;
    List.iter (fun v ->
      if state.(v) = 1 then has_cycle := true
      else if state.(v) = 0 then dfs v
    ) adj.(u);
    state.(u) <- 2;
    result := u :: !result
  in
  for v = 0 to n - 1 do
    if state.(v) = 0 then dfs v
  done;
  if !has_cycle then None else Some !result

let () =
  (* DAG: build order dependencies *)
  let n   = 6 in
  let adj = Array.make n [] in
  let add u v = adj.(u) <- v :: adj.(u) in
  (* 5->2->0, 5->0, 4->0, 4->1, 2->3, 3->1 *)
  add 5 2; add 5 0; add 4 0; add 4 1; add 2 3; add 3 1;
  (match topo_sort adj n with
   | None   -> Printf.printf "Cycle detected!\n"
   | Some o -> Printf.printf "Topological order: [%s]\n"
       (String.concat " -> " (List.map string_of_int o)));

  (* Graph with cycle *)
  let adj2 = Array.make 3 [] in
  Array.set adj2 0 [1]; Array.set adj2 1 [2]; Array.set adj2 2 [0];
  (match topo_sort adj2 3 with
   | None   -> Printf.printf "Cycle detected (correct)!\n"
   | Some o -> Printf.printf "Order: [%s]\n"
       (String.concat "->" (List.map string_of_int o)))
