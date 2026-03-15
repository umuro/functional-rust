(* 1036: Graph with Arena Allocation *)
(* Use arrays as arenas — index-based graph representation *)

(* Approach 1: Array-based arena graph *)
type node = {
  label: string;
  edges: int list;  (* indices into the arena *)
}

let arena_graph () =
  (* Build a graph: 0:A -> 1:B, 2:C; 1:B -> 2:C; 2:C -> 0:A *)
  let arena = [|
    { label = "A"; edges = [1; 2] };
    { label = "B"; edges = [2] };
    { label = "C"; edges = [0] };
  |] in
  assert (arena.(0).label = "A");
  assert (arena.(0).edges = [1; 2]);
  (* Follow edges from A *)
  let neighbors_of_a = List.map (fun i -> arena.(i).label) arena.(0).edges in
  assert (neighbors_of_a = ["B"; "C"])

(* Approach 2: Build graph incrementally *)
let build_graph () =
  let nodes = Array.make 4 { label = ""; edges = [] } in
  nodes.(0) <- { label = "start"; edges = [1; 2] };
  nodes.(1) <- { label = "mid1"; edges = [3] };
  nodes.(2) <- { label = "mid2"; edges = [3] };
  nodes.(3) <- { label = "end"; edges = [] };
  (* BFS from node 0 *)
  let visited = Array.make 4 false in
  let queue = Queue.create () in
  Queue.push 0 queue;
  visited.(0) <- true;
  let order = ref [] in
  while not (Queue.is_empty queue) do
    let idx = Queue.pop queue in
    order := nodes.(idx).label :: !order;
    List.iter (fun neighbor ->
      if not visited.(neighbor) then begin
        visited.(neighbor) <- true;
        Queue.push neighbor queue
      end
    ) nodes.(idx).edges
  done;
  let bfs_order = List.rev !order in
  assert (bfs_order = ["start"; "mid1"; "mid2"; "end"])

(* Approach 3: Weighted edges *)
type weighted_node = {
  w_label: string;
  w_edges: (int * float) list;  (* (target_index, weight) *)
}

let weighted_graph () =
  let arena = [|
    { w_label = "A"; w_edges = [(1, 1.0); (2, 4.0)] };
    { w_label = "B"; w_edges = [(2, 2.0)] };
    { w_label = "C"; w_edges = [] };
  |] in
  let total_weight_from_a =
    List.fold_left (fun acc (_, w) -> acc +. w) 0.0 arena.(0).w_edges
  in
  assert (total_weight_from_a = 5.0)

let () =
  arena_graph ();
  build_graph ();
  weighted_graph ();
  Printf.printf "✓ All tests passed\n"
