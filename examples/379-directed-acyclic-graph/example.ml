(* DAG and Topological Sort in OCaml using Kahn's algorithm *)

let topological_sort adj n =
  let in_degree = Array.make n 0 in
  Array.iter (fun neighbors ->
    List.iter (fun v -> in_degree.(v) <- in_degree.(v) + 1) neighbors
  ) adj;
  let queue = Queue.create () in
  Array.iteri (fun v d -> if d = 0 then Queue.add v queue) in_degree;
  let result = ref [] in
  while not (Queue.is_empty queue) do
    let u = Queue.pop queue in
    result := u :: !result;
    List.iter (fun v ->
      in_degree.(v) <- in_degree.(v) - 1;
      if in_degree.(v) = 0 then Queue.add v queue
    ) adj.(u)
  done;
  if List.length !result = n then Some (List.rev !result)
  else None  (* cycle detected *)

let () =
  (* Build graph: 5 -> 2 -> 0 -> 1, 5 -> 0, 4 -> 0, 4 -> 1, 2 -> 3, 3 -> 1 *)
  let n = 6 in
  let adj = Array.make n [] in
  let edges = [(5,2);(5,0);(4,0);(4,1);(2,0);(2,3);(3,1)] in
  List.iter (fun (u,v) -> adj.(u) <- v :: adj.(u)) edges;
  match topological_sort adj n with
  | Some order ->
    Printf.printf "Topological order: ";
    List.iter (fun v -> Printf.printf "%d " v) order;
    print_newline ()
  | None ->
    Printf.printf "Graph has a cycle!\n"
