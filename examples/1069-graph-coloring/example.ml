(* 1069: Graph Coloring — Backtracking and Greedy
   Assign colors to nodes so no two adjacent nodes share a color. *)

module IntSet = Set.Make(Int)

(* Approach 1: Backtracking with adjacency matrix *)
let graph_coloring adj num_colors =
  let n = Array.length adj in
  let colors = Array.make n 0 in
  let is_safe node color =
    not (Array.exists (fun i -> adj.(node).(i) = 1 && colors.(i) = color)
      (Array.init n (fun i -> i)))
  in
  let rec solve node =
    if node = n then true
    else begin
      let found = ref false in
      let c = ref 1 in
      while !c <= num_colors && not !found do
        if is_safe node !c then begin
          colors.(node) <- !c;
          if solve (node + 1) then found := true
          else colors.(node) <- 0
        end;
        incr c
      done;
      !found
    end
  in
  if solve 0 then Some (Array.to_list colors) else None

(* Approach 2: Backtracking with adjacency list *)
let graph_coloring_list adj num_colors =
  let n = Array.length adj in
  let colors = Array.make n 0 in
  let is_safe node color =
    List.for_all (fun nb -> colors.(nb) <> color) adj.(node)
  in
  let rec solve node =
    if node = n then true
    else begin
      let found = ref false in
      let c = ref 1 in
      while !c <= num_colors && not !found do
        if is_safe node !c then begin
          colors.(node) <- !c;
          if solve (node + 1) then found := true
          else colors.(node) <- 0
        end;
        incr c
      done;
      !found
    end
  in
  if solve 0 then Some (Array.to_list colors) else None

(* Approach 3: Greedy coloring — not always optimal but fast *)
let greedy_coloring adj =
  let n = Array.length adj in
  let colors = Array.make n 0 in
  for node = 0 to n - 1 do
    (* Collect colors used by already-colored neighbors *)
    let used = List.fold_left
      (fun acc nb -> if colors.(nb) > 0 then IntSet.add colors.(nb) acc else acc)
      IntSet.empty adj.(node)
    in
    (* Assign smallest color not in used *)
    let c = ref 1 in
    while IntSet.mem !c used do incr c done;
    colors.(node) <- !c
  done;
  Array.to_list colors

let () =
  let adj_matrix = [|
    [|0;1;1;1|]; [|1;0;1;0|]; [|1;1;0;1|]; [|1;0;1;0|]
  |] in

  (match graph_coloring adj_matrix 3 with
   | None -> assert false
   | Some colors ->
     (* No adjacent nodes share a color *)
     for i = 0 to 3 do for j = 0 to 3 do
       if adj_matrix.(i).(j) = 1 then
         assert (List.nth colors i <> List.nth colors j)
     done done);

  assert (graph_coloring adj_matrix 2 = None);

  let adj_list = [| [1;2;3]; [0;2]; [0;1;3]; [0;2] |] in

  (match graph_coloring_list adj_list 3 with
   | None -> assert false
   | Some colors -> assert (List.length colors = 4));

  let greedy = greedy_coloring adj_list in
  List.iteri (fun node neighbors ->
    List.iter (fun nb ->
      assert (List.nth greedy node <> List.nth greedy nb)
    ) neighbors
  ) (Array.to_list adj_list);

  Printf.printf "All graph-coloring tests passed.\n"
