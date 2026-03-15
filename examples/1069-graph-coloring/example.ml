(* 1069: Graph Coloring — Backtracking *)

(* Approach 1: Basic backtracking *)
let graph_coloring adj_matrix num_colors =
  let n = Array.length adj_matrix in
  let colors = Array.make n 0 in
  let is_safe node color =
    let safe = ref true in
    for i = 0 to n - 1 do
      if adj_matrix.(node).(i) = 1 && colors.(i) = color then
        safe := false
    done;
    !safe
  in
  let rec solve node =
    if node = n then true
    else begin
      let found = ref false in
      for c = 1 to num_colors do
        if not !found && is_safe node c then begin
          colors.(node) <- c;
          if solve (node + 1) then found := true
          else colors.(node) <- 0
        end
      done;
      !found
    end
  in
  if solve 0 then Some (Array.to_list colors) else None

(* Approach 2: With adjacency list *)
let graph_coloring_adj adj_list num_colors n =
  let colors = Array.make n 0 in
  let is_safe node color =
    List.for_all (fun neighbor ->
      colors.(neighbor) <> color
    ) adj_list.(node)
  in
  let rec solve node =
    if node = n then true
    else
      let found = ref false in
      for c = 1 to num_colors do
        if not !found && is_safe node c then begin
          colors.(node) <- c;
          if solve (node + 1) then found := true
          else colors.(node) <- 0
        end
      done;
      !found
  in
  if solve 0 then Some (Array.to_list colors) else None

let () =
  (* Petersen-like graph: 4 nodes, edges: 0-1, 1-2, 2-3, 3-0, 0-2 *)
  let adj = [|
    [|0;1;1;1|];
    [|1;0;1;0|];
    [|1;1;0;1|];
    [|1;0;1;0|]
  |] in
  (match graph_coloring adj 3 with
   | Some colors -> assert (List.length colors = 4)
   | None -> assert false);
  assert (graph_coloring adj 2 = None);

  let adj_list = [|[1;2;3]; [0;2]; [0;1;3]; [0;2]|] in
  (match graph_coloring_adj adj_list 3 4 with
   | Some colors -> assert (List.length colors = 4)
   | None -> assert false);

  Printf.printf "✓ All tests passed\n"
